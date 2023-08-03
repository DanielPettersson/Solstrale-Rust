use std::error::Error;
use image::RgbImage;
use crate::geo::vec3::Vec3;
use crate::post::{pixel_colors_to_rgb_image, PostProcessor, PostProcessors};

#[derive(Clone)]
/// A post processor that does nothing
pub struct NopPostProcessor();

impl NopPostProcessor {
    #![allow(clippy::new_ret_no_self)]
    /// Create a new nop post processor
    pub fn new() -> PostProcessors {
        PostProcessors::NopPostProcessorType(NopPostProcessor())
    }
}


impl PostProcessor for NopPostProcessor {
    fn post_process(&self, pixel_colors: &[Vec3], _albedo_colors: &[Vec3], _normal_colors: &[Vec3], width: u32, height: u32, num_samples: u32) -> Result<RgbImage, Box<dyn Error>> {
        Ok(pixel_colors_to_rgb_image(pixel_colors, width, height, num_samples))
    }

    fn intermediate_post_process(&self, pixel_colors: &[Vec3], _albedo_colors: &[Vec3], _normal_colors: &[Vec3], _width: u32, _height: u32, _num_samples: u32) -> Result<Vec<Vec3>, Box<dyn Error>> {
        Ok(Vec::from(pixel_colors))
    }

    fn needs_albedo_and_normal_colors(&self) -> bool {
        false
    }
}