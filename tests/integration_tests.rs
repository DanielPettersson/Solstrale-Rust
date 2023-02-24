use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

use image::imageops::FilterType;
use image::RgbImage;
use image_compare::Algorithm::RootMeanSquared;

use solstrale::post::OidnPostProcessor;
use solstrale::ray_trace;
use solstrale::renderer::shader::{PathTracingShader, Shaders, SimpleShader};
use solstrale::renderer::{RenderConfig, Scene};

use crate::scenes::{
    create_obj_scene, create_obj_with_box, create_simple_test_scene, create_test_scene,
    create_uv_scene,
};

mod scenes;

const IMAGE_COMPARISON_SCORE_THRESHOLD: f64 = 0.95;

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

#[test]
fn test_render_scene_with_oidn() {
    let render_config = RenderConfig {
        samples_per_pixel: 20,
        shader: PathTracingShader::new(50),
        post_processor: Some(OidnPostProcessor::new()),
    };

    let scene = create_simple_test_scene(render_config, true);
    render_and_compare_output(scene, "oidn", 200, 100)
}

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
fn test_render_obj_with_default_material() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_obj_with_box(render_config, "tests/obj/", "box.obj");

    render_and_compare_output(scene, "obj_default", 200, 100);
}

#[test]
fn test_render_obj_with_diffuse_material() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_obj_with_box(render_config, "tests/obj/", "boxWithMat.obj");

    render_and_compare_output(scene, "obj_diffuse", 200, 100);
}

#[test]
fn test_render_uv_mapping() {
    let render_config = RenderConfig {
        samples_per_pixel: 5,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_uv_scene(render_config);

    render_and_compare_output(scene, "uv", 200, 200);
}

#[test]
fn test_abort_render_scene() {
    let render_config = RenderConfig {
        samples_per_pixel: 1000,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_test_scene(render_config);

    let (output_sender, output_receiver) = channel();
    let (abort_sender, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(200, 100, scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut progress_count = 0;
    for _ in output_receiver {
        progress_count += 1;
        abort_sender.send(true).unwrap();
    }
    assert!(progress_count < 1000, "Most likely it should be 1 or 2 depending on timing, but definitely less than 100 as rendering is aborted")
}

#[test]
fn test_render_scene_without_light() {
    let render_config = RenderConfig {
        samples_per_pixel: 100,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_simple_test_scene(render_config, false);

    let (output_sender, _) = channel();
    let (_, abort_receiver) = channel();

    let res = ray_trace(20, 10, scene, &output_sender, &abort_receiver);

    match res {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!("Scene should have at least one light", e.to_string()),
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

    actual_image
        .save(format!("tests/output/out_actual_{}.jpg", name))
        .unwrap();

    assert!(
        score > IMAGE_COMPARISON_SCORE_THRESHOLD,
        "Comparison score for {} is: {}",
        name,
        score
    )
}
