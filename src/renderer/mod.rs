use std::error::Error;
use std::sync::mpsc::{Receiver, SendError, Sender};

use image::{ImageBuffer, RgbImage};
use simple_error::SimpleError;

use crate::camera::{Camera, CameraConfig};
use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{Hittable, Hittables};
use crate::post::PostProcessors;
use crate::random::random_normal_float;
use crate::renderer::shader::{AlbedoShader, NormalShader, Shader, Shaders};
use crate::util::interval::Interval;
use crate::util::rgb_color::to_rgb_color;

pub mod shader;

///Input to the ray tracer for how the image should be rendered
pub struct RenderConfig {
    pub samples_per_pixel: u32,
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
    output: &'a Sender<Result<RenderProgress, Box<dyn Error>>>,
    abort: &'a Receiver<bool>,
    albedo_shader: AlbedoShader,
    normal_shader: NormalShader,
}

const RAY_INTERVAL: Interval = Interval {
    min: 0.001,
    max: f64::INFINITY,
};

impl<'a> Renderer<'a> {
    /// Creates a new renderer given a scene and channels for communicating with the caller
    pub fn new(
        scene: Scene,
        output: &'a Sender<Result<RenderProgress, Box<dyn Error>>>,
        abort: &'a Receiver<bool>,
    ) -> Result<Renderer<'a>, Box<dyn Error>> {
        let mut lights = HittableList::new();
        find_lights(&scene.world, &mut lights);

        if lights.list.len() == 0 {
            return Err(Box::new(SimpleError::new(
                "Scene should have at least one light",
            )));
        }

        return Ok(Renderer {
            scene,
            lights,
            output,
            abort,
            albedo_shader: AlbedoShader {},
            normal_shader: NormalShader {},
        });
    }

    fn ray_color(&self, ray: &Ray, depth: u32) -> (Vec3, Vec3, Vec3) {
        match self.scene.world.hit(ray, &RAY_INTERVAL) {
            Some(rec) => {
                let pixel_color = self
                    .scene
                    .render_config
                    .shader
                    .shade(self, &rec, ray, depth);

                if depth == 0 {
                    if let Some(_) = &self.scene.render_config.post_processor {
                        let albedo_color = self.albedo_shader.shade(self, &rec, ray, depth);
                        let normal_color = self.normal_shader.shade(self, &rec, ray, depth);
                        return (pixel_color, albedo_color, normal_color);
                    }
                }

                return (pixel_color, ZERO_VECTOR, ZERO_VECTOR);
            }
            None => (
                self.scene.background_color,
                self.scene.background_color,
                ZERO_VECTOR,
            ),
        }
    }

    /// Executes the rendering of the image
    pub fn render(&self, image_width: u32, image_height: u32) {
        let pixel_count = image_width * image_height;
        let samples_per_pixel = self.scene.render_config.samples_per_pixel;

        let mut pixel_colors: Vec<Vec3> = vec![ZERO_VECTOR; pixel_count as usize];
        let mut albedo_colors: Vec<Vec3> = vec![ZERO_VECTOR; pixel_count as usize];
        let mut normal_colors: Vec<Vec3> = vec![ZERO_VECTOR; pixel_count as usize];

        let camera = Camera::new(image_width, image_height, &self.scene.camera);

        for sample in 1..=samples_per_pixel {
            if let Some(_) = self.abort.iter().peekable().peek() {
                return;
            }

            for y in 0..image_height {
                for x in 0..image_width {
                    let i = (((image_height - 1) - y) * image_width + x) as usize;

                    let u = (x as f64 + random_normal_float()) / (image_width - 1) as f64;
                    let v = (y as f64 + random_normal_float()) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    let (pixel_color, albedo_color, normal_color) = self.ray_color(&ray, 0);

                    pixel_colors[i] = pixel_colors[i] + pixel_color;

                    if let Some(_) = self.scene.render_config.post_processor {
                        albedo_colors[i] = albedo_colors[i] + albedo_color;
                        normal_colors[i] = normal_colors[i] + normal_color;
                    }
                }
            }
            if create_progress(
                image_width,
                image_height,
                sample,
                samples_per_pixel,
                pixel_colors.clone(),
                self.output,
            )
            .is_err()
            {
                return;
            }
        }
    }
}

fn create_progress(
    image_width: u32,
    image_height: u32,
    sample: u32,
    samples_per_pixel: u32,
    pixel_colors: Vec<Vec3>,
    output: &Sender<Result<RenderProgress, Box<dyn Error>>>,
) -> Result<(), SendError<Result<RenderProgress, Box<dyn Error>>>> {
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let i = (y * image_width + x) as usize;
            img.put_pixel(x, y, to_rgb_color(pixel_colors[i], sample))
        }
    }

    output.send(Ok(RenderProgress {
        progress: sample as f64 / samples_per_pixel as f64,
        render_image: img,
    }))
}

fn find_lights(s: &Hittables, list: &mut HittableList) {
    match s.children() {
        None => {
            if s.is_light() {
                list.add(s.clone());
            }
        }
        Some(children) => {
            for child in children {
                find_lights(&child, list)
            }
        }
    }
}
