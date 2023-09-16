#![warn(missing_docs)]
//! A multi-threaded Monte Carlo path tracing library, that as such has features like:
//! * Global illumination
//! * Caustics
//! * Reflection
//! * Refraction
//! * Soft shadows
//!
//! Additionally the library has:
//! * Loading of obj models with included materials
//! * Multi-threaded Bvh creation to greatly speed up rendering
//! * Post processing of rendered images by:
//!   * [Open Image Denoise](https://www.openimagedenoise.org/)
//!   * Bloom filter
//! * Bump mapping
//! * Light attenuation
//!
//! ## Example:
//! ```rust
//! # use std::sync::mpsc::channel;
//! # use std::thread;
//! # use image::RgbImage;
//! # use solstrale::camera::CameraConfig;
//! # use solstrale::geo::vec3::Vec3;
//! # use solstrale::hittable::{Bvh, Sphere, Hittable};
//! # use solstrale::material::{DiffuseLight, Lambertian};
//! # use solstrale::material::texture::SolidColor;
//! # use solstrale::ray_trace;
//! # use solstrale::renderer::{RenderConfig, Scene};
//! # use solstrale::renderer::shader::PathTracingShader;
//! let camera = CameraConfig {
//!     vertical_fov_degrees: 20.,
//!     aperture_size: 0.1,
//!     look_from: Vec3::new(0., 0., 4.),
//!     look_at: Vec3::new(0., 0., 0.),
//!     up: Vec3::new(0., 1., 0.),
//! };
//! let mut world = Vec::new();
//! let yellow = Lambertian::new(SolidColor::new(1., 1., 0.), None);
//! let light = DiffuseLight::new(10., 10., 10., None);
//! world.push(Sphere::new(Vec3::new(0., 0., 0.), 0.5, yellow));
//!
//! let scene = Scene {
//!     world: Bvh::new(world),
//!     camera,
//!     background_color: Vec3::new(0.2, 0.3, 0.5),
//!     render_config: RenderConfig::default(),
//! };
//!
//! let (output_sender, output_receiver) = channel();
//! let (_, abort_receiver) = channel();
//!
//! thread::spawn(move || {
//!     ray_trace(800, 400, scene, &output_sender, &abort_receiver).unwrap();
//! });
//!
//! for render_output in output_receiver {
//!     let _image = render_output.render_image;
//! }
//! ```
//!
//! ## Example output
//! ![happy](https://github.com/DanielPettersson/solstrale-rust/assets/3603911/c5357792-a3dc-42f9-8230-320140f9c30e)
//! ![sponza-bump2](https://github.com/DanielPettersson/solstrale-rust/assets/3603911/0ab79ed9-cddf-46b1-84e7-03cef35f5600)

//! ## Credits
//! The ray tracing is inspired by the excellent [Ray Tracing in One Weekend Book Series](https://github.com/RayTracing/raytracing.github.io) by Peter Shirley

use crate::renderer::{RenderProgress, Renderer, Scene};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};

pub mod camera;
pub mod geo;
pub mod hittable;
pub mod loader;
pub mod material;
pub mod pdf;
pub mod post;
pub mod random;
pub mod renderer;
pub mod util;

/// Executes the ray tracing with the given [`Scene`] and reports [`RenderProgress`] on
/// the output [`Sender`]. Listens to abort [`Receiver`] for aborting a started ray trace operation
///
/// # Arguments
/// * `width` - Width in pixel of the rendered image
/// * `height` - Height in pixel of the rendered image
/// * `output` - Channel where render progress will be sent
/// * `abort` - Channel to send abort signals to the renderer
pub fn ray_trace<'a>(
    width: u32,
    height: u32,
    scene: Scene,
    output: &'a Sender<RenderProgress>,
    abort: &'a Receiver<bool>,
) -> Result<(), Box<dyn Error>> {
    Renderer::new(scene)?.render(width as usize, height as usize, output, abort)
}
