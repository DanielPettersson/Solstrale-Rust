use solstrale::camera::CameraConfig;
use solstrale::geo::vec3::Vec3;
use solstrale::hittable::hittable_list::HittableList;
use solstrale::hittable::obj_model::new_obj_model;
use solstrale::hittable::quad::Quad;
use solstrale::hittable::sphere::Sphere;
use solstrale::hittable::Hittable;
use solstrale::material::texture::{ImageTexture, SolidColor};
use solstrale::material::{DiffuseLight, Lambertian};
use solstrale::renderer::{RenderConfig, Scene};

pub fn simple_test_scene(render_config: RenderConfig, add_light: bool) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        focus_distance: 10.,
        look_from: Vec3::new(0., 0., 4.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = HittableList::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.));
    let light = DiffuseLight::new(10., 10., 10.);
    if add_light {
        world.add(Sphere::new(Vec3::new(0., 100., 0.), 20., light))
    }
    world.add(Sphere::new(Vec3::new(0., 0., 0.), 0.5, yellow));

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

pub fn create_obj_scene(render_config: RenderConfig) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 30.,
        aperture_size: 20.,
        focus_distance: 260.,
        look_from: Vec3::new(-250., 30., 150.),
        look_at: Vec3::new(-50., 0., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(15., 15., 15.);

    world.add(Sphere::new(Vec3::new(-100., 100., 40.), 35., light));
    let model = new_obj_model("tests/spider/", "spider.obj", 1.).unwrap();
    world.add(model);

    let image_tex = ImageTexture::load("tests/textures/tex.jpg").unwrap();
    let ground_material = Lambertian::new(image_tex);
    world.add(Quad::new(
        Vec3::new(-200., -30., -200.),
        Vec3::new(400., 0., 0.),
        Vec3::new(0., 0., 400.),
        ground_material,
    ));

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}
