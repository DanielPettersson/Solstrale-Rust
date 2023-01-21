use crate::geo::vec3::Vec3;
use image::RgbImage;
use std::error::Error;

/// Responsible for taking the rendered image and transforming it
pub trait PostProcessor {
    fn post_process(
        pixel_colors: Vec<Vec3>,
        albedo_colors: Vec<Vec3>,
        normal_colors: Vec<Vec3>,
        width: i32,
        height: i32,
        num_samples: i32,
    ) -> Result<RgbImage, Box<dyn Error>>;
}
