use std::sync::mpsc::channel;
use std::thread;

use image::RgbImage;

use solstrale::ray_trace;
use solstrale::renderer::RenderConfig;

use crate::scenes::create_test_scene;

#[path = "../../tests/scenes.rs"]
mod scenes;

fn main() {
    let render_config = RenderConfig {
        width: 800,
        height: 400,
        samples_per_pixel: 1000,
        ..RenderConfig::default()
    };
    let scene = create_test_scene(render_config);

    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut image = RgbImage::new(800, 400);
    for render_output in output_receiver {
        if let Some(render_image) = render_output.render_image {
            image = render_image;
        }
    }

    image.save("out.jpg").unwrap();
}
