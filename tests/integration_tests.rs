use crate::scenes::{create_obj_scene, create_test_scene};
use image::imageops::FilterType;
use image::RgbImage;
use image_compare::Algorithm::RootMeanSquared;
use solstrale::ray_trace;
use solstrale::renderer::shader::{PathTracingShader, Shaders, SimpleShader};
use solstrale::renderer::{RenderConfig, Scene};
use std::collections::HashMap;
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

#[test]
fn test_render_scene() {
    let shaders: HashMap<&str, Shaders> = HashMap::from([
        ("pathTracing", PathTracingShader::new(50)),
        ("simple", SimpleShader::new()),
    ]);

    for (shader_name, shader) in shaders {
        let render_config = RenderConfig {
            samples_per_pixel: 25,
            shader,
            post_processor: None,
        };
        let scene = create_test_scene(render_config);

        render_and_compare_output(scene, shader_name, 200, 100)
    }
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

    let sized_actual = image::imageops::resize(actual_image, 100, 50, FilterType::Gaussian);
    let sized_expected = image::imageops::resize(&expected_image, 100, 50, FilterType::Gaussian);

    let score =
        image_compare::rgb_similarity_structure(&RootMeanSquared, &sized_expected, &sized_actual)
            .expect("Failed to compare images")
            .score;

    if score <= IMAGE_COMPARISON_SCORE_THRESHOLD {
        actual_image
            .save(format!("tests/output/out_actual_{}.jpg", name))
            .unwrap();
    }

    assert!(
        score > IMAGE_COMPARISON_SCORE_THRESHOLD,
        "Comparison score for {} is: {}",
        name,
        score
    )
}
