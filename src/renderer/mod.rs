use std::error::Error;
use std::ops::Deref;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::sync::{Arc, Mutex};

use image::{ImageBuffer, RgbImage};
use simple_error::SimpleError;

use crate::camera::{Camera, CameraConfig};
use crate::geo::ray::Ray;
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::hittable::hittable_list::HittableList;
use crate::hittable::{Hittable, Hittables};
use crate::post::{PostProcessor, PostProcessors};
use crate::random::random_normal_float;
use crate::renderer::shader::{AlbedoShader, NormalShader, Shader, Shaders};
use crate::util::interval::RAY_INTERVAL;
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
pub struct Renderer {
    scene: Scene,
    pub lights: Hittables,
    albedo_shader: AlbedoShader,
    normal_shader: NormalShader,
}

impl Renderer {
    /// Creates a new renderer given a scene and channels for communicating with the caller
    pub fn new(scene: Scene) -> Result<Renderer, Box<dyn Error>> {
        let mut lights = HittableList::new();
        find_lights(&scene.world, &mut lights);

        let has_lights = match lights.children() {
            Some(mut list) => list.next().is_some(),
            None => false,
        };

        if !has_lights {
            return Err(Box::new(SimpleError::new(
                "Scene should have at least one light",
            )));
        }

        return Ok(Renderer {
            scene,
            lights,
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
    pub fn render(
        &self,
        image_width: usize,
        image_height: usize,
        output: &Sender<RenderProgress>,
        abort: &Receiver<bool>,
    ) -> Result<(), Box<dyn Error>> {
        let pixel_count = image_width * image_height;
        let samples_per_pixel = self.scene.render_config.samples_per_pixel;
        let has_post_processor = self.scene.render_config.post_processor.is_some();

        let pixel_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count as usize]));
        let albedo_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count as usize]));
        let normal_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count as usize]));

        let camera = Arc::new(Camera::new(image_width, image_height, &self.scene.camera));

        let pool = rayon::ThreadPoolBuilder::new()
            .build()
            .expect("Failed to create thread pool");

        for sample in 1..=samples_per_pixel {
            match abort.try_recv() {
                Ok(_) => return Ok(()),
                _ => {}
            }

            pool.scope(|s| {
                for y in 0..image_height {
                    let cam = camera.clone();
                    let cloned_pixel_colors = pixel_colors.clone();
                    let cloned_albedo_colors = albedo_colors.clone();
                    let cloned_normal_colors = normal_colors.clone();

                    s.spawn(move |_| {
                        let mut row_pixel_colors: Vec<Vec3> =
                            vec![ZERO_VECTOR; image_width as usize];
                        let mut row_albedo_colors: Vec<Vec3> = if has_post_processor {
                            vec![ZERO_VECTOR; image_width as usize]
                        } else {
                            Vec::new()
                        };
                        let mut row_normal_colors: Vec<Vec3> = if has_post_processor {
                            vec![ZERO_VECTOR; image_width as usize]
                        } else {
                            Vec::new()
                        };

                        let yi = ((image_height - 1) - y) * image_width;
                        for x in 0..image_width {
                            let u = (x as f64 + random_normal_float()) / (image_width - 1) as f64;
                            let v = (y as f64 + random_normal_float()) / (image_height - 1) as f64;
                            let ray = cam.get_ray(u, v);
                            let (pixel_color, albedo_color, normal_color) = self.ray_color(&ray, 0);

                            row_pixel_colors[x] = pixel_color;

                            if has_post_processor {
                                row_albedo_colors[x] = albedo_color;
                                row_normal_colors[x] = normal_color;
                            }
                        }

                        let mut pc = cloned_pixel_colors.lock().unwrap();
                        for (x, c) in row_pixel_colors.iter().enumerate() {
                            pc[yi + x] += *c;
                        }

                        if has_post_processor {
                            let mut pc = cloned_albedo_colors.lock().unwrap();
                            for (x, c) in row_albedo_colors.iter().enumerate() {
                                pc[yi + x] += *c;
                            }

                            let mut pc = cloned_normal_colors.lock().unwrap();
                            for (x, c) in row_normal_colors.iter().enumerate() {
                                pc[yi + x] += *c;
                            }
                        }
                    });
                }
            });

            create_progress(
                image_width as u32,
                image_height as u32,
                sample,
                samples_per_pixel,
                pixel_colors.lock().unwrap().deref(),
                output,
            )?
        }

        match &self.scene.render_config.post_processor {
            Some(p) => match abort.try_recv() {
                Ok(_) => Ok(()),
                _ => {
                    match p.post_process(
                        pixel_colors.lock().unwrap().deref(),
                        albedo_colors.lock().unwrap().deref(),
                        normal_colors.lock().unwrap().deref(),
                        image_width as u32,
                        image_height as u32,
                        samples_per_pixel,
                    ) {
                        Ok(img) => {
                            output.send(RenderProgress {
                                progress: 1.,
                                render_image: img,
                            })?;
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                }
            },
            None => Ok(()),
        }
    }
}

fn create_progress(
    image_width: u32,
    image_height: u32,
    sample: u32,
    samples_per_pixel: u32,
    pixel_colors: &Vec<Vec3>,
    output: &Sender<RenderProgress>,
) -> Result<(), SendError<RenderProgress>> {
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let i = (y * image_width + x) as usize;
            img.put_pixel(x, y, to_rgb_color(pixel_colors[i], sample))
        }
    }

    output.send(RenderProgress {
        progress: sample as f64 / samples_per_pixel as f64,
        render_image: img,
    })
}

fn find_lights(s: &Hittables, list: &mut Hittables) {
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
