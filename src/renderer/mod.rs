use std::sync::mpsc::{Receiver, Sender};

use image::RgbImage;
use simple_error::SimpleError;

use crate::camera::CameraConfig;
use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{Hittable, Hittables};
use crate::post::PostProcessors;
use crate::renderer::shader::{AlbedoShader, NormalShader, Shaders};

pub mod shader;

///Input to the ray tracer for how the image should be rendered
pub struct RenderConfig {
    pub samples_per_pixel: i32,
    pub shader: Shaders,
    pub post_processor: Option<PostProcessors>,
}

/// Contains all information needed to render an image
pub struct Scene {
    pub world: Hittables,
    pub camera: CameraConfig,
    pub background_color: Vec3,
    pub render_config: RenderConfig,
}

/// progress reported back to the caller of the raytrace function
pub struct RenderProgress {
    pub progress: f64,
    pub render_image: RgbImage,
}

/// Renderer is a central part of the raytracer responsible for controlling the
/// process reporting back progress to the caller
pub struct Renderer<'a> {
    scene: Scene,
    pub lights: HittableList,
    output: &'a Sender<RenderProgress>,
    abort: &'a Receiver<bool>,
    albedo_shader: AlbedoShader,
    normal_shader: NormalShader,
    max_millis_between_progress_output: i64,
}

impl<'a> Renderer<'a> {
    /// Creates a new renderer given a scene and channels for communicating with the caller
    pub fn new(
        scene: Scene,
        output: &'a Sender<RenderProgress>,
        abort: &'a Receiver<bool>,
    ) -> Result<Renderer<'a>, SimpleError> {
        let mut lights = HittableList::new();
        find_lights(&scene.world, &mut lights);

        if lights.list.len() == 0 {
            return Err(SimpleError::new("Scene should have at least one light"));
        }

        return Ok(Renderer {
            scene,
            lights,
            output,
            abort,
            albedo_shader: AlbedoShader {},
            normal_shader: NormalShader {},
            max_millis_between_progress_output: 500,
        });
    }

    pub fn ray_color(&self, _: Ray, _: i32) -> (Vec3, Vec3, Vec3) {
        (ZERO_VECTOR, ZERO_VECTOR, ZERO_VECTOR)
    }
}

fn find_lights(s: &Hittables, list: &mut HittableList) {
    match s.children() {
        None => {
            if s.is_light() {
                list.add(s.clone_light());
            }
        }
        Some(children) => {
            for child in children {
                find_lights(&child, list)
            }
        }
    }
}
