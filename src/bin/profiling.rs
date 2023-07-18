use std::sync::mpsc::channel;
use std::{env, thread};

use image::RgbImage;

use solstrale::camera::CameraConfig;
use solstrale::geo::transformation::NopTransformer;
use solstrale::geo::vec3::Vec3;
use solstrale::hittable::Hittable;
use solstrale::hittable::HittableList;
use solstrale::hittable::Sphere;
use solstrale::loader::obj::Obj;
use solstrale::loader::Loader;
use solstrale::material::DiffuseLight;
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::{RenderConfig, Scene};

fn main() {
    let obj_path = &env::args().nth(1).expect("Object path argument required");

    let render_config = RenderConfig {
        samples_per_pixel: 50,
        shader: PathTracingShader::new(50),
        post_processor: None,
    };
    let scene = create_obj_scene(render_config, obj_path);

    let (output_sender, output_receiver) = channel();
    let (_, abort_receiver) = channel();

    thread::spawn(move || {
        ray_trace(800, 400, scene, &output_sender, &abort_receiver).unwrap();
    });

    let mut image = RgbImage::new(800, 400);
    for render_output in output_receiver {
        image = render_output.render_image
    }

    image.save("out.jpg").unwrap();
}

fn create_obj_scene(render_config: RenderConfig, obj_path: &str) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 30.,
        aperture_size: 0.,
        look_from: Vec3::new(1., 0.05, 0.),
        look_at: Vec3::new(0., 0.05, 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(15., 15., 15., None);

    world.add(Sphere::new(Vec3::new(100., 100., 100.), 35., light));
    world.add(
        Obj::new("", obj_path)
            .load(&NopTransformer(), None)
            .unwrap(),
    );

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}
