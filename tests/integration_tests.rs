use crate::scenes::create_obj_scene;
use image::RgbImage;
use image_compare::Algorithm::RootMeanSquared;
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::{RenderConfig, Scene};
use std::sync::mpsc::channel;
use std::thread;

mod scenes;

const IMAGE_COMPARISON_SCORE_THRESHOLD: f64 = 0.95;

#[test]
fn test_render_obj_with_textures() {
    let render_config = RenderConfig {
        samples_per_pixel: 20,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_obj_scene(render_config);

    render_and_compare_output(scene, "obj", 200, 100);
}

fn render_and_compare_output(scene: Scene, name: &str, width: u32, height: u32) {
    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(width, height, scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut image = RgbImage::new(200, 100);
    for render_output in output_receiver {
        image = render_output.render_image
    }

    compare_output(name, &image);
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

    if score <= IMAGE_COMPARISON_SCORE_THRESHOLD {
        actual_image
            .save(format!("tests/output/out_actual_{}.jpg", name))
            .unwrap();
    }

    assert!(
        score > IMAGE_COMPARISON_SCORE_THRESHOLD,
        "Comparison score is: {}",
        score
    )
}
