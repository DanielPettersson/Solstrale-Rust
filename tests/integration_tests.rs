use crate::scenes::simple_test_scene;
use image::RgbImage;
use image_compare::Algorithm::RootMeanSquared;
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::RenderConfig;
use std::sync::mpsc::channel;
use std::thread;

mod scenes;

#[test]
fn test_render_simple_scene() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = simple_test_scene(render_config, true);

    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(200, 100, scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut image = RgbImage::new(200, 100);
    for render_output in output_receiver {
        image = render_output.render_image
    }

    compare_output("simple", &image);
}

fn compare_output(name: &str, actual_image: &RgbImage) {
    let expected_image_path = format!("tests/output/out_expected_{}.jpg", name);
    let expected_image = image::open(&expected_image_path)
        .expect(&format!("Could not load {}", &expected_image_path))
        .into_rgb8();

    let score =
        image_compare::rgb_similarity_structure(&RootMeanSquared, &expected_image, actual_image)
            .expect("Failed to compare images")
            .score;

    assert!(score > 0.95, "Comparison score is: {}", score)
}
