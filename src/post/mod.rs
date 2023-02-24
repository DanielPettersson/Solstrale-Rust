use crate::geo::vec3::Vec3;
use crate::util::rgb_color;
use enum_dispatch::enum_dispatch;
use image::{ImageBuffer, Rgb, RgbImage};
use simple_error::SimpleError;
use std::error::Error;

/// Responsible for taking the rendered image and transforming it
#[enum_dispatch]
pub trait PostProcessor {
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<RgbImage, Box<dyn Error>>;
}

#[enum_dispatch(PostProcessor)]
pub enum PostProcessors {
    OidnPostProcessor(OidnPostProcessor),
}

pub struct OidnPostProcessor();

impl OidnPostProcessor {
    pub fn new() -> PostProcessors {
        PostProcessors::OidnPostProcessor(OidnPostProcessor())
    }
}

impl PostProcessor for OidnPostProcessor {
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<RgbImage, Box<dyn Error>> {
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
            return Err(Box::new(SimpleError::new(e.1)));
        }

        let mut img: RgbImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let i = ((y * width + x) * 3) as usize;
                img.put_pixel(
                    x,
                    y,
                    Rgb([
                        (output[i] * 256.) as u8,
                        (output[i + 1] * 256.) as u8,
                        (output[i + 2] * 256.) as u8,
                    ]),
                );
            }
        }

        Ok(img)
    }
}

fn to_rgb_vec(vec: &[Vec3], num_samples: u32) -> Vec<f32> {
    vec.iter()
        .flat_map(|v| {
            let c = rgb_color::to_float(*v, num_samples);
            vec![c.x as f32, c.y as f32, c.z as f32]
        })
        .collect()
}
