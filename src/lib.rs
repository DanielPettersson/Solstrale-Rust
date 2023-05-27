//! A path tracer library
//!
//! A port of the Go Solstrale library
//! See [Go Solstrale library][go_solstrale].
//!
//! [go_solstrale]: https://github.com/DanielPettersson/solstrale

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
