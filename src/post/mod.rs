use crate::geo::vec3::Vec3;
use enum_dispatch::enum_dispatch;
use image::RgbImage;
use std::error::Error;

/// Responsible for taking the rendered image and transforming it
#[enum_dispatch]
pub trait PostProcessor {
    fn post_process(
        &self,
        pixel_colors: Vec<Vec3>,
        albedo_colors: Vec<Vec3>,
        normal_colors: Vec<Vec3>,
        width: i32,
        height: i32,
        num_samples: i32,
    ) -> Result<RgbImage, Box<dyn Error>>;
}

#[enum_dispatch(PostProcessor)]
pub enum PostProcessors {
    TodoPostProcessor(TodoPostProcessor),
}

pub struct TodoPostProcessor();

impl PostProcessor for TodoPostProcessor {
    fn post_process(
        &self,
        _: Vec<Vec3>,
        _: Vec<Vec3>,
        _: Vec<Vec3>,
        _: i32,
        _: i32,
        _: i32,
    ) -> Result<RgbImage, Box<dyn Error>> {
        todo!()
    }
}
