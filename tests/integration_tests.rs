use crate::scenes::simple_test_scene;
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::RenderConfig;
use std::sync::mpsc::channel;
use std::thread;

mod scenes;

#[test]
fn test_render_simple_scene() {
    let render_config = RenderConfig {
        samples_per_pixel: 100,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = simple_test_scene(render_config, true);

    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(200, 100, scene, &output_sender, &abort_receiver);
    });
}
