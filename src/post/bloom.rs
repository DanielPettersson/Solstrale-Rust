use std::error::Error;

use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::post::{pixel_colors_to_rgb_image, PostProcessor, PostProcessors};
use crate::util::gaussian::create_gaussian_blur_weights;

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
    pub fn new(
        kernel_size_fraction: f64,
        threshold: Option<f64>,
        max_intensity: Option<f64>,
    ) -> Result<PostProcessors, simple_error::SimpleError> {
        if !(0. ..=0.5).contains(&kernel_size_fraction) {
            return Err(simple_error::SimpleError::new(
                "kernel_size_fraction must be between 0 and 0.5",
            ));
        }

        let threshold = threshold.unwrap_or(Vec3::new(1., 1., 1.).length());
        let max_intensity = max_intensity.unwrap_or(f64::MAX);

        Ok(PostProcessors::from(BloomPostProcessor {
            kernel_size_fraction,
            threshold,
            max_intensity,
        }))
    }
}

impl PostProcessor for BloomPostProcessor {
    fn post_process(
        &self,
        pixel_colors: &[Vec3],
        albedo_colors: &[Vec3],
        normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<image::RgbImage, Box<dyn Error>> {
        let pixel_colors = self.intermediate_post_process(
            pixel_colors,
            albedo_colors,
            normal_colors,
            width,
            height,
            num_samples,
        )?;
        Ok(pixel_colors_to_rgb_image(
            &pixel_colors,
            width,
            height,
            num_samples,
        ))
    }

    #[allow(clippy::needless_range_loop)]
    fn intermediate_post_process(
        &self,
        pixel_colors: &[Vec3],
        _albedo_colors: &[Vec3],
        _normal_colors: &[Vec3],
        width: u32,
        height: u32,
        num_samples: u32,
    ) -> Result<Vec<Vec3>, Box<dyn Error>> {
        let threshold = self.threshold * num_samples as f64;
        let max_intensity = self.max_intensity * num_samples as f64;
        let kernel_size = (self.kernel_size_fraction * width as f64) as usize * 2 + 1;
        let half_kernel_size = (kernel_size / 2) as i32;

        let weights = create_gaussian_blur_weights(kernel_size, kernel_size as f64 / 5.);

        let bright_colors: Vec<Vec3> = Vec::from(pixel_colors)
            .par_iter()
            .map(|p| {
                if p.length() >= threshold {
                    if p.length() > max_intensity {
                        p.unit() * max_intensity
                    } else {
                        *p
                    }
                } else {
                    ZERO_VECTOR
                }
            })
            .collect();

        let blurred_colors: Vec<Vec3> = (0..(height * width))
            .into_par_iter()
            .map(|xy| {
                let x = (xy % width) as i32;
                let y = (xy / width) as i32;
                let mut col = ZERO_VECTOR;
                for i in 0..kernel_size {
                    col += get_pixel_safe(
                        &bright_colors,
                        x + i as i32 - half_kernel_size,
                        y,
                        width,
                        height,
                    ) * weights[i];
                }
                col
            })
            .collect();

        let blurred_colors: Vec<Vec3> = (0..(height * width))
            .into_par_iter()
            .map(|xy| {
                let x = (xy % width) as i32;
                let y = (xy / width) as i32;
                let mut col = ZERO_VECTOR;
                for i in 0..kernel_size {
                    col += get_pixel_safe(
                        &blurred_colors,
                        x,
                        y + i as i32 - half_kernel_size,
                        width,
                        height,
                    ) * weights[i];
                }
                col
            })
            .collect();

        Ok(pixel_colors
            .into_par_iter()
            .zip(blurred_colors)
            .map(|pp| *pp.0 + pp.1)
            .collect())
    }

    fn needs_albedo_and_normal_colors(&self) -> bool {
        false
    }
}

fn get_pixel_safe(pixel_colors: &[Vec3], x: i32, y: i32, width: u32, height: u32) -> Vec3 {
    let x = x.clamp(0, width as i32 - 1);
    let y = y.clamp(0, height as i32 - 1);
    let i = (y * width as i32 + x) as usize;
    pixel_colors[i]
}
