//! The renderer takes a [`Scene`] as input, renders it and reports [`RenderProgress`]

use std::error::Error;
use std::ops::Deref;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use image::{ImageBuffer, RgbImage};
use simple_error::SimpleError;

use crate::camera::{Camera, CameraConfig};
use crate::geo::vec3::{Vec3, ZERO_VECTOR};
use crate::geo::{Ray, Uv};
use crate::hittable::HittableList;
use crate::hittable::{Hittable, Hittables};
use crate::material::AttenuatedColor;
use crate::post::{PostProcessor, PostProcessors};
use crate::random::random_normal_float;
use crate::renderer::shader::{AlbedoShader, NormalShader, Shader, Shaders};
use crate::util::interval::RAY_INTERVAL;
use crate::util::rgb_color::to_rgb_color;

pub mod shader;

///Input to the ray tracer for how the image should be rendered
#[derive(Clone)]
pub struct RenderConfig {
    /// Number of times each pixel should be sampled
    pub samples_per_pixel: u32,
    /// Shader to use when rendering the image
    pub shader: Shaders,
    /// Post processor to apply to the rendered image
    pub post_processor: Option<PostProcessors>,
}

/// Contains all information needed to render an image
pub struct Scene {
    /// World is the hittable objects in the scene
    pub world: Hittables,
    /// A camera for defining the view of the world
    pub camera: CameraConfig,
    /// Background color of the scene
    pub background_color: Vec3,
    /// Render configuration
    pub render_config: RenderConfig,
}

/// progress reported back to the caller of the raytrace function
pub struct RenderProgress {
    /// progress is reported between 0 -> 1 and represents a percentage of completion
    pub progress: f64,
    /// Current speed of rendering in number of frames per second
    pub fps: Option<f64>,
    /// Estimated time left until rendering is complete
    pub estimated_time_left: Duration,
    /// Output image so far, will be final when progress is 1
    pub render_image: RgbImage,
}

/// Renderer is a central part of the raytracer responsible for controlling the
/// process reporting back progress to the caller
pub struct Renderer {
    scene: Scene,
    /// All the light hittables in the world
    pub lights: Hittables,
    albedo_shader: AlbedoShader,
    normal_shader: NormalShader,
}

/// Result of calculating color for a ray
pub(crate) struct RayColorResult {
    pixel_color: AttenuatedColor,
    albedo_color: Vec3,
    normal_color: Vec3,
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

        Ok(Renderer {
            scene,
            lights,
            albedo_shader: AlbedoShader {},
            normal_shader: NormalShader {},
        })
    }

    fn ray_color(&self, ray: &Ray, depth: u32, accumulated_ray_length: f64) -> RayColorResult {
        match self.scene.world.hit(ray, &RAY_INTERVAL) {
            Some(rec) => {
                let attenuated_color = self.scene.render_config.shader.shade(
                    self,
                    &rec,
                    ray,
                    depth,
                    accumulated_ray_length,
                );

                if depth == 0 && self.scene.render_config.post_processor.is_some() {
                    let albedo_color = self
                        .albedo_shader
                        .shade(self, &rec, ray, depth, accumulated_ray_length)
                        .color;
                    let normal_color = self
                        .normal_shader
                        .shade(self, &rec, ray, depth, accumulated_ray_length)
                        .color;
                    return RayColorResult {
                        pixel_color: attenuated_color,
                        albedo_color,
                        normal_color,
                    };
                }

                RayColorResult {
                    pixel_color: attenuated_color,
                    albedo_color: ZERO_VECTOR,
                    normal_color: ZERO_VECTOR,
                }
            }
            None => RayColorResult {
                pixel_color: AttenuatedColor {
                    color: self.scene.background_color,
                    ..AttenuatedColor::default()
                },
                albedo_color: self.scene.background_color,
                normal_color: ZERO_VECTOR,
            },
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
        let mut last_frame_render_time = SystemTime::now();
        let render_start_time = SystemTime::now();
        let pixel_count = image_width * image_height;
        let samples_per_pixel = self.scene.render_config.samples_per_pixel;
        let has_post_processor = self.scene.render_config.post_processor.is_some();

        let pixel_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count]));
        let albedo_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count]));
        let normal_colors: Arc<Mutex<Vec<Vec3>>> =
            Arc::new(Mutex::new(vec![ZERO_VECTOR; pixel_count]));

        let camera = Arc::new(Camera::new(image_width, image_height, &self.scene.camera));

        let pool = rayon::ThreadPoolBuilder::new()
            .build()
            .expect("Failed to create thread pool");

        for sample in 1..=samples_per_pixel {
            if abort.try_recv().is_ok() {
                return Ok(());
            }

            pool.scope(|s| {
                for y in 0..image_height {
                    let camera = camera.clone();
                    let pixel_colors = pixel_colors.clone();
                    let albedo_colors = albedo_colors.clone();
                    let normal_colors = normal_colors.clone();

                    s.spawn(move |_| {
                        let mut row_pixel_colors: Vec<Vec3> = vec![ZERO_VECTOR; image_width];
                        let mut row_albedo_colors: Vec<Vec3> = if has_post_processor {
                            vec![ZERO_VECTOR; image_width]
                        } else {
                            Vec::new()
                        };
                        let mut row_normal_colors: Vec<Vec3> = if has_post_processor {
                            vec![ZERO_VECTOR; image_width]
                        } else {
                            Vec::new()
                        };

                        let yi = ((image_height - 1) - y) * image_width;
                        for x in 0..image_width {
                            let u = (x as f64 + random_normal_float()) / (image_width - 1) as f64;
                            let v = (y as f64 + random_normal_float()) / (image_height - 1) as f64;
                            let ray = camera.get_ray(Uv::new(u as f32, v as f32));
                            let ray_color_res = self.ray_color(&ray, 0, 0.);

                            row_pixel_colors[x] = ray_color_res.pixel_color.get_attenuated_color();

                            if has_post_processor {
                                row_albedo_colors[x] = ray_color_res.albedo_color;
                                row_normal_colors[x] = ray_color_res.normal_color;
                            }
                        }

                        add_row_data(yi, &mut pixel_colors.lock().unwrap(), &row_pixel_colors);
                        if has_post_processor {
                            add_row_data(yi, &mut albedo_colors.lock().unwrap(), &row_albedo_colors);
                            add_row_data(yi, &mut normal_colors.lock().unwrap(), &row_normal_colors);
                        }
                    });
                }
            });

            {
                let render_image = create_render_image(
                    image_width as u32,
                    image_height as u32,
                    sample,
                    &pixel_colors.lock().unwrap()
                );
                let now = SystemTime::now();

                output.send(RenderProgress {
                    progress: sample as f64 / samples_per_pixel as f64,
                    fps: Some(calculate_fps(&mut last_frame_render_time, now)),
                    estimated_time_left: calculate_estimated_time_left(
                        render_start_time,
                        now,
                        sample,
                        samples_per_pixel,
                    ),
                    render_image,
                })?
            }
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
                                fps: None,
                                estimated_time_left: Duration::from_millis(0),
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

fn add_row_data(yi: usize, colors: &mut [Vec3], row_colors: &[Vec3]) {
    for (x, c) in row_colors.iter().enumerate() {
        colors[yi + x] += *c;
    }
}

fn create_render_image(image_width: u32, image_height: u32, sample: u32, colors: &[Vec3]) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let i = (y * image_width + x) as usize;
            img.put_pixel(x, y, to_rgb_color(colors[i], sample))
        }
    }
    img
}

fn calculate_fps(last_frame_render_time: &mut SystemTime, now: SystemTime) -> f64 {
    let micros_since_last_frame = now
        .duration_since(*last_frame_render_time)
        .unwrap_or(Duration::from_millis(1))
        .as_micros();
    *last_frame_render_time = now;

    1_000_000. / micros_since_last_frame as f64
}

fn calculate_estimated_time_left(
    render_start_time: SystemTime,
    now: SystemTime,
    samples_done: u32,
    total_samples: u32,
) -> Duration {
    let time_since_start = now
        .duration_since(render_start_time)
        .unwrap_or(Duration::from_millis(1));
    let samples_left = total_samples - samples_done;

    time_since_start
        .div_f32(samples_done as f32)
        .mul_f32(samples_left as f32)
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
                find_lights(child, list)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::renderer::{calculate_estimated_time_left, calculate_fps};
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_calculate_fps() {
        let mut last_frame_render_time = SystemTime::UNIX_EPOCH + Duration::from_millis(900);
        let now = SystemTime::UNIX_EPOCH + Duration::from_millis(1000);

        let fps = calculate_fps(&mut last_frame_render_time, now);

        assert_eq!(fps, 10.);
        assert_eq!(last_frame_render_time, now);
    }

    #[test]
    fn test_calculate_estimated_time_left() {
        let render_start = SystemTime::UNIX_EPOCH;
        let now = SystemTime::UNIX_EPOCH + Duration::from_millis(1000);

        let mut time_left = calculate_estimated_time_left(render_start, now, 1, 100);
        assert_eq!(time_left, Duration::from_secs(99));

        time_left = calculate_estimated_time_left(render_start, now, 50, 100);
        assert_eq!(time_left, Duration::from_secs(1));

        time_left = calculate_estimated_time_left(render_start, now, 100, 100);
        assert_eq!(time_left, Duration::from_secs(0));
    }
}
