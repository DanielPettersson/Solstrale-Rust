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

// RayTrace executes the ray tracing with the given scene and reports progress on
// the output channel. Listens to abort channel for aborting a started ray trace operation
pub fn ray_trace<'a>(
    width: u32,
    height: u32,
    scene: Scene,
    output: &'a Sender<RenderProgress>,
    abort: &'a Receiver<bool>,
) -> Result<(), Box<dyn Error>> {
    Renderer::new(scene)?.render(width as usize, height as usize, output, abort)
}
