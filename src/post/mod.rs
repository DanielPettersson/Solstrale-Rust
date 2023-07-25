//! Post processors for applying effects to the raw rendered image

mod oidn;
mod bloom;

use std::error::Error;

use enum_dispatch::enum_dispatch;

use crate::geo::vec3::Vec3;
pub use crate::post::bloom::BloomPostProcessor;
pub use crate::post::oidn::OidnPostProcessor;

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

    /// Does this post-processor need albedo or normal colors
    fn needs_albedo_and_normal_colors(&self) -> bool;
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
