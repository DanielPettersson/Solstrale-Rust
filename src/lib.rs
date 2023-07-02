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
//! * Post processing of rendered images using [Open Image Denoise](https://www.openimagedenoise.org/)
//!
//! ## Example output
//! ![sponza-bump2](https://github.com/DanielPettersson/solstrale-rust/assets/3603911/0ab79ed9-cddf-46b1-84e7-03cef35f5600)
//!
//! ## Credits
//! The ray tracing is inspired by the excellent [Ray Tracing in One Weekend Book Series](https://github.com/RayTracing/raytracing.github.io) by Peter Shirley

use crate::renderer::{RenderProgress, Renderer, Scene};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};

pub mod camera;
pub mod geo;
pub mod hittable;
pub mod material;
pub mod pdf;
pub mod post;
pub mod random;
pub mod renderer;
pub mod util;

/// Executes the ray tracing with the given [`Scene`] and reports [`RenderProgress`] on
/// the output [`Sender`]. Listens to abort [`Receiver`] for aborting a started ray trace operation
pub fn ray_trace<'a>(
    width: u32,
    height: u32,
    scene: Scene,
    output: &'a Sender<RenderProgress>,
    abort: &'a Receiver<bool>,
) -> Result<(), Box<dyn Error>> {
    Renderer::new(scene)?.render(width as usize, height as usize, output, abort)
}
