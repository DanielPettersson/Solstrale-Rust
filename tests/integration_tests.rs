use std::collections::HashMap;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;

use image::imageops::FilterType;
use image::RgbImage;
use image_compare::Algorithm::RootMeanSquared;
use solstrale::geo::vec3::{Vec3, ZERO_VECTOR};
use solstrale::post::{BloomPostProcessor, OidnPostProcessor, PostProcessor};

use solstrale::ray_trace;
use solstrale::renderer::shader::{PathTracingShader, Shaders, SimpleShader};
use solstrale::renderer::{RenderConfig, Scene};
use solstrale::util::rgb_color::rgb_to_vec3;

use crate::scenes::{
    create_light_attenuation_scene, create_normal_mapping_scene, create_obj_scene,
    create_obj_with_box, create_obj_with_triangle, create_simple_test_scene, create_test_scene,
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
            post_processors: Vec::new(),
        };
        let scene = create_test_scene(render_config);

        render_and_compare_output(scene, shader_name, 200, 100)
    }
}

#[test]
#[cfg(feature = "oidn-postprocessor")]
fn test_render_scene_with_oidn() {
    let render_config = RenderConfig {
        samples_per_pixel: 20,
        shader: PathTracingShader::new(50),
        post_processors: vec![OidnPostProcessor::new()],
    };

    let scene = create_simple_test_scene(render_config, true);
    render_and_compare_output(scene, "oidn", 200, 100)
}

#[test]
fn test_render_obj_with_textures() {
    let render_config = RenderConfig {
        samples_per_pixel: 20,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_obj_scene(render_config);

    render_and_compare_output(scene, "obj", 200, 100);
}

#[test]
fn test_render_obj_with_default_material() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_obj_with_box(render_config, "resources/obj/", "box.obj");

    render_and_compare_output(scene, "obj_default", 200, 100);
}

#[test]
fn test_render_obj_with_diffuse_material() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_obj_with_box(render_config, "resources/obj/", "boxWithMat.obj");

    render_and_compare_output(scene, "obj_diffuse", 200, 100);
}

#[test]
fn test_render_uv_mapping() {
    let render_config = RenderConfig {
        samples_per_pixel: 5,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_uv_scene(render_config);

    render_and_compare_output(scene, "uv", 200, 200);
}

#[test]
fn test_render_normal_mapping_disabled() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: vec![OidnPostProcessor::new()],
    };

    let scene = create_normal_mapping_scene(render_config, Vec3::new(30., 30., 30.), false);
    render_and_compare_output(scene, "normal_mapping_disabled", 400, 400);
}

#[test]
fn test_render_normal_mapping_1() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: vec![OidnPostProcessor::new()],
    };

    let scene = create_normal_mapping_scene(render_config, Vec3::new(30., 30., 30.), true);
    render_and_compare_output(scene, "normal_mapping_1", 400, 400);
}

#[test]
fn test_render_normal_mapping_2() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: vec![OidnPostProcessor::new()],
    };

    let scene = create_normal_mapping_scene(render_config, Vec3::new(-30., 30., 30.), true);
    render_and_compare_output(scene, "normal_mapping_2", 400, 400);
}

#[test]
fn test_abort_render_scene() {
    let render_config = RenderConfig {
        samples_per_pixel: 1000,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_test_scene(render_config);

    let (output_sender, output_receiver) = channel();
    let (abort_sender, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(400, 200, scene, &output_sender, &abort_receiver).unwrap();
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
        post_processors: Vec::new(),
    };
    let scene = create_simple_test_scene(render_config, false);

    let (output_sender, _) = channel();
    let (_, abort_receiver) = channel();

    let res = ray_trace(20, 10, scene, &output_sender, &abort_receiver);

    match res {
        Ok(_) => panic!("There should be an error"),
        Err(e) => assert_eq!("Scene should have at least one light", e.to_string()),
    }
}

#[test]
fn test_render_obj_with_normal_map() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_obj_with_triangle(render_config, "resources/obj/", "triWithNormalMap.obj");

    render_and_compare_output(scene, "obj_normal_map", 500, 500);
}

#[test]
fn test_render_obj_with_height_map() {
    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processors: Vec::new(),
    };
    let scene = create_obj_with_triangle(render_config, "resources/obj/", "triWithHeightMap.obj");

    render_and_compare_output(scene, "obj_height_map", 500, 500);
}

#[test]
fn test_render_light_attenuation() {
    for attenuation_half_length in [Some(0.1), Some(0.8), None] {
        let render_config = RenderConfig {
            samples_per_pixel: 50,
            shader: PathTracingShader::new(50),
            post_processors: Vec::new(),
        };
        let scene = create_light_attenuation_scene(render_config, attenuation_half_length);

        render_and_compare_output(
            scene,
            &format!(
                "light_attenuation_{}",
                attenuation_half_length.map_or(-1., |a| a)
            ),
            300,
            300,
        );
    }
}

#[test]
fn test_bloom() -> Result<(), Box<dyn Error>> {
    let post = BloomPostProcessor::new(0.2, None, None)?;
    let bloom_image = image::open("resources/textures/bloom.png").unwrap().into_rgb8();
    let w = bloom_image.width();
    let h = bloom_image.height();
    let pixel_colors = image_to_vec3(bloom_image);

    let res = post.post_process(
        &pixel_colors,
        &[ZERO_VECTOR; 0],
        &[ZERO_VECTOR; 0],
        w,
        h,
        1
    )?;
    
    compare_output("bloom", &res);

    Ok(())
}

fn image_to_vec3(image: RgbImage) -> Vec<Vec3> {
    let mut ret = Vec::with_capacity((image.width() * image.height()) as usize);
    for y in 0..image.height() {
        for x in 0..image.width() {
            ret.push(rgb_to_vec3(image.get_pixel(x, y)));
        }
    }
    ret
}


fn render_and_compare_output(scene: Scene, name: &str, width: u32, height: u32) {
    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(width, height, scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut image = RgbImage::new(width, height);
    for render_output in output_receiver {
        image = render_output.render_image
    }

    compare_output(name, &image);
}

fn compare_output(name: &str, actual_image: &RgbImage) {
    actual_image
        .save(format!("tests/output/out_actual_{}.jpg", name))
        .unwrap();

    let expected_image_path = format!("tests/output/out_expected_{}.jpg", name);
    let expected_image = image::open(&expected_image_path)
        .unwrap_or_else(|_| panic!("Could not load {}", &expected_image_path))
        .into_rgb8();

    let sized_actual = image::imageops::resize(actual_image, 100, 50, FilterType::Gaussian);
    let sized_expected = image::imageops::resize(&expected_image, 100, 50, FilterType::Gaussian);

    let score =
        image_compare::rgb_similarity_structure(&RootMeanSquared, &sized_expected, &sized_actual)
            .expect("Failed to compare images")
            .score;

    assert!(
        score > IMAGE_COMPARISON_SCORE_THRESHOLD,
        "Comparison score for {} is: {}",
        name,
        score
    )
}
