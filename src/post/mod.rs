//! Post processors for applying effects to the raw rendered image

use std::error::Error;

use enum_dispatch::enum_dispatch;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::util::gaussian::create_gaussian_blur_weights;

/// Responsible for taking the rendered image and transforming it
#[enum_dispatch]
pub trait PostProcessor {
    /// Execute final postprocessing of the rendered image
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<image::RgbImage, Box<dyn Error>>;

    /// Execute intermediate postprocessing of the rendered image
    fn intermediate_post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<Vec<Vec3>, Box<dyn Error>>;
}

#[enum_dispatch(PostProcessor)]
#[derive(Clone)]
/// An enum of available post processors
pub enum PostProcessors {
    /// [`PostProcessor`] of type [`OidnPostProcessor`]
    OidnPostProcessorType(OidnPostProcessor),
    /// [`PostProcessor`] of type [`BloomPostProcessor`]
    BloomPostProcessorType(BloomPostProcessor),
}

#[derive(Clone)]
/// A post processor that uses Intel Open Image DeNoise on the image
pub struct OidnPostProcessor();

impl OidnPostProcessor {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new oidn post processor
    pub fn new() -> PostProcessors {
        PostProcessors::OidnPostProcessorType(OidnPostProcessor())
    }
}

#[cfg(feature = "oidn-postprocessor")]
impl PostProcessor for OidnPostProcessor {
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<image::RgbImage, Box<dyn Error>> {
        let pixel_rgb = to_rgb_vec(pixel_colors, num_samples);
        let albedo_rgb = to_rgb_vec(albedo_colors, num_samples);
        let normal_rgb = to_rgb_vec(normal_colors, num_samples);
        let mut output = vec![0.0f32; pixel_rgb.len()];

        let device = oidn::Device::new();
        oidn::RayTracing::new(&device)
            .image_dimensions(width as usize, height as usize)
            .albedo_normal(&albedo_rgb, &normal_rgb)
            .srgb(true)
            .hdr(false)
            .clean_aux(true)
            .filter(&pixel_rgb, &mut output)
            .expect("Failed to apply Oidn post processing");

        if let Err(e) = device.get_error() {
            return Err(Box::new(simple_error::SimpleError::new(e.1)));
        }

        let mut img: image::RgbImage = image::ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let i = ((y * width + x) * 3) as usize;
                img.put_pixel(
                    x,
                    y,
                    image::Rgb([
                        (output[i] * 256.) as u8,
                        (output[i + 1] * 256.) as u8,
                        (output[i + 2] * 256.) as u8,
                    ]),
                );
            }
        }

        Ok(img)
    }

    fn intermediate_post_process(&self, _pixel_colors: &[Vec3], _albedo_colors: &[Vec3], _normal_colors: &[Vec3], _width: u32, _height: u32, _num_samples: u32) -> Result<Vec<Vec3>, Box<dyn Error>> {
        Err(Box::new(simple_error::SimpleError::new("Intel Open Image DeNoise can not be used as an intermediate post processor")))
    }
}

#[cfg(not(feature = "oidn-postprocessor"))]
impl PostProcessor for OidnPostProcessor {
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        _albedo_colors: &[Vec3],
        _normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<image::RgbImage, Box<dyn Error>> {
        Ok(pixel_colors_to_rgb_image(pixel_colors, width, height, num_samples))
    }

    fn intermediate_post_process(&self, pixel_colors: &[Vec3], _albedo_colors: &[Vec3], _normal_colors: &[Vec3], _width: u32, _height: u32, _num_samples: u32) -> Result<Vec<Vec3>, Box<dyn Error>> {
        Ok(Vec::from(pixel_colors))
    }
}

#[cfg(feature = "oidn-postprocessor")]
fn to_rgb_vec(vec: &[Vec3], num_samples: u32) -> Vec<f32> {
    vec.iter()
        .flat_map(|v| {
            let c = crate::util::rgb_color::to_float(*v, num_samples);
            vec![c.x as f32, c.y as f32, c.z as f32]
        })
        .collect()
}

#[derive(Clone)]
/// Applies a bloom effect on the pixels colors
pub struct BloomPostProcessor {
    kernel_size_fraction: f64,
    threshold: f64,
    max_intensity: f64,
}

impl BloomPostProcessor {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new bloom post processor
    /// # Arguments
    /// * `kernel_size_fraction` Radius of the blur effect, as a fraction of the rendered image's width
    /// * `threshold` Color intensity threshold for applying bloom effect. If not specified, defaults to "white"
    /// * `max_intensity` Maximum color intensity of the bloom effect. If not specified, defaults to unlimited
    pub fn new(kernel_size_fraction: f64, threshold: Option<f64>, max_intensity: Option<f64>) -> Result<PostProcessors, simple_error::SimpleError> {
        if !(0. ..=0.5).contains(&kernel_size_fraction) {
            return Err(simple_error::SimpleError::new("kernel_size_fraction must be between 0 and 0.5"));
        }

        let threshold = threshold.unwrap_or(Vec3::new(1., 1., 1.).length());
        let max_intensity = max_intensity.unwrap_or(f64::MAX);

        Ok(PostProcessors::BloomPostProcessorType(BloomPostProcessor { kernel_size_fraction, threshold, max_intensity }))
    }
}

impl PostProcessor for BloomPostProcessor {

    fn post_process(&self, pixel_colors: &[Vec3], albedo_colors: &[Vec3], normal_colors: &[Vec3], width: u32, height: u32, num_samples: u32) -> Result<image::RgbImage, Box<dyn Error>> {
        let pixel_colors = self.intermediate_post_process(pixel_colors, albedo_colors, normal_colors, width, height, num_samples)?;
        Ok(pixel_colors_to_rgb_image(&pixel_colors, width, height, num_samples))
    }

    #[allow(clippy::needless_range_loop)]
    fn intermediate_post_process(&self, pixel_colors: &[Vec3], _albedo_colors: &[Vec3], _normal_colors: &[Vec3], width: u32, height: u32, num_samples: u32) -> Result<Vec<Vec3>, Box<dyn Error>> {

        let threshold = self.threshold * num_samples as f64;
        let max_intensity = self.max_intensity * num_samples as f64;
        let kernel_size = (self.kernel_size_fraction * width  as f64) as usize * 2 + 1;
        let half_kernel_size = (kernel_size / 2) as i32;

        let weights = create_gaussian_blur_weights(kernel_size, kernel_size as f64 / 5.);

        let mut blur_pixel_colors: Vec<Vec3> = Vec::from(pixel_colors).par_iter().map(|p| if p.length() >= threshold {
            if p.length() > max_intensity {
                p.unit() * max_intensity
            } else {
                *p
            }
        } else {
            ZERO_VECTOR
        }).collect();

        for y in 0..height as i32 {
            for x in 0..width as i32 {
                let mut col = ZERO_VECTOR;
                for i in 0..kernel_size {
                    col += get_pixel_safe(&blur_pixel_colors, x + i as i32 - half_kernel_size, y, width, height) * weights[i];
                }
                blur_pixel_colors[(y * width as i32 + x) as usize] = col;
            }
        }
        for x in 0..width as i32{
            for y in 0..height as i32 {
                let mut col = ZERO_VECTOR;
                for i in 0..kernel_size {
                    col += get_pixel_safe(&blur_pixel_colors, x, y + i as i32 - half_kernel_size, width, height) * weights[i];
                }
                blur_pixel_colors[(y * width as i32 + x) as usize] = col;
            }
        }

        Ok(pixel_colors.into_par_iter().zip(blur_pixel_colors).map(|pp| *pp.0 + pp.1).collect())
    }
}



fn get_pixel_safe(pixel_colors: &[Vec3], x: i32, y: i32, width: u32, height: u32) -> Vec3 {
    let x = x.clamp(0, width as i32 - 1);
    let y = y.clamp(0, height as i32 - 1);
    let i = (y * width as i32 + x) as usize;
    pixel_colors[i]
}

fn pixel_colors_to_rgb_image(pixel_colors: &[Vec3], width: u32, height: u32, num_samples: u32) -> image::RgbImage {
    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            img.put_pixel(x, y, crate::util::rgb_color::to_rgb_color(pixel_colors[i], num_samples))
        }
    }

    img
}
