use solstrale::camera::CameraConfig;
use solstrale::geo::vec3::{Vec3, ZERO_VECTOR};
use solstrale::geo::Uv;
use solstrale::hittable::Bvh;
use solstrale::hittable::ConstantMedium;
use solstrale::hittable::Hittable;
use solstrale::hittable::HittableList;
use solstrale::hittable::Hittables::TriangleType;
use solstrale::hittable::Quad;
use solstrale::hittable::RotationY;
use solstrale::hittable::Sphere;
use solstrale::hittable::Translation;
use solstrale::hittable::Triangle;
use solstrale::hittable::{load_obj_model, load_obj_model_with_default_material};
use solstrale::material::texture::{ImageTexture, SolidColor};
use solstrale::material::{Dielectric, DiffuseLight, Lambertian};
use solstrale::renderer::{RenderConfig, Scene};

pub fn create_test_scene(render_config: RenderConfig) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        look_from: Vec3::new(-5., 3., 6.),
        look_at: Vec3::new(0.25, 1., 0.),
    };

    let mut world = HittableList::new();

    let image_tex = ImageTexture::load("resources/textures/tex.jpg").unwrap();

    let ground_material = Lambertian::new(image_tex, None);
    let glass_mat = Dielectric::new(SolidColor::new(1., 1., 1.), None, 1.5);
    let light_mat = DiffuseLight::new(10., 10., 10.);
    let red_mat = Lambertian::new(SolidColor::new(1., 0., 0.), None);

    world.add(Quad::new(
        Vec3::new(-5., 0., -15.),
        Vec3::new(20., 0., 0.),
        Vec3::new(0., 0., 20.),
        ground_material,
    ));
    world.add(Sphere::new(Vec3::new(-1., 1., 0.), 1., glass_mat));
    world.add(RotationY::new(
        Quad::new_box(
            Vec3::new(0., 0., -0.5),
            Vec3::new(1., 2., 0.5),
            red_mat.clone(),
        ),
        15.,
    ));
    world.add(ConstantMedium::new(
        Translation::new(
            Quad::new_box(
                Vec3::new(0., 0., -0.5),
                Vec3::new(1., 2., 0.5),
                red_mat.clone(),
            ),
            Vec3::new(0., 0., 1.),
        ),
        0.1,
        Vec3::new(1., 1., 1.),
    ));
    world.add(Quad::new_box(
        Vec3::new(-1., 2., 0.),
        Vec3::new(-0.5, 2.5, 0.5),
        red_mat.clone(),
    ));

    let mut balls = Vec::new();
    for ii in (0..10).step_by(2) {
        let i = ii as f64 * 0.1;
        for jj in (0..10).step_by(2) {
            let j = jj as f64 * 0.1;
            for kk in (0..10).step_by(2) {
                let k = kk as f64 * 0.1;
                if let TriangleType(t) = Triangle::new(
                    Vec3::new(i, j + 0.05, k + 0.8),
                    Vec3::new(i, j, k + 0.8),
                    Vec3::new(i, j + 0.05, k),
                    red_mat.clone(),
                ) {
                    balls.push(t)
                }
            }
        }
    }
    world.add(Bvh::new(balls));

    world.add(Triangle::new(
        Vec3::new(1., 0.1, 2.),
        Vec3::new(3., 0.1, 2.),
        Vec3::new(2., 0.1, 1.),
        red_mat,
    ));

    // Lights

    world.add(Sphere::new(Vec3::new(10., 5., 10.), 10., light_mat.clone()));
    world.add(Translation::new(
        RotationY::new(
            Quad::new(
                Vec3::new(0., 0., 0.),
                Vec3::new(2., 0., 0.),
                Vec3::new(0., 0., 2.),
                light_mat.clone(),
            ),
            45.,
        ),
        Vec3::new(-1., 10., -1.),
    ));
    world.add(Triangle::new(
        Vec3::new(-2., 1., -3.),
        Vec3::new(0., 1., -3.),
        Vec3::new(-1., 2., -3.),
        light_mat,
    ));

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

#[allow(dead_code)]
pub fn new_bvh_test_scene(render_config: RenderConfig, use_bvh: bool, num_triangles: u32) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        look_from: Vec3::new(-0.5, 0., 4.),
        look_at: Vec3::new(-0.5, 0., 0.),
    };

    let mut world = HittableList::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.), None);
    let light = DiffuseLight::new(10., 10., 10.);
    world.add(Sphere::new(Vec3::new(0., 4., 10.), 4., light));

    let mut triangles = Vec::new();
    for x in 0..num_triangles {
        let cx = x as f64 - num_triangles as f64 / 2.;
        let t = Triangle::new(
            Vec3::new(cx, -0.5, 0.),
            Vec3::new(cx + 1., -0.5, 0.),
            Vec3::new(cx + 0.5, 0.5, 0.),
            yellow.clone(),
        );
        if use_bvh {
            if let TriangleType(tri) = t {
                triangles.push(tri);
            }
        } else {
            world.add(t);
        }
    }

    if use_bvh {
        world.add(Bvh::new(triangles))
    }

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

#[allow(dead_code)]
pub fn create_simple_test_scene(render_config: RenderConfig, add_light: bool) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        look_from: Vec3::new(0., 0., 4.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = HittableList::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.), None);
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

#[allow(dead_code)]
pub fn create_uv_scene(render_config: RenderConfig) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.,
        look_from: Vec3::new(0., 1., 5.),
        look_at: Vec3::new(0., 1., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(10., 10., 10.);

    world.add(Sphere::new(Vec3::new(50., 50., 50.), 20., light));

    let tex = ImageTexture::load("resources/textures/checker.jpg").unwrap();
    let checker_mat = Lambertian::new(tex, None);

    world.add(Triangle::new_with_tex_coords(
        Vec3::new(-1., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 2., 0.),
        Uv::new(-1., -1.),
        Uv::new(2., -1.),
        Uv::new(0., 2.),
        checker_mat,
    ));

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

#[allow(dead_code)]
pub fn create_normal_mapping_scene(
    render_config: RenderConfig,
    light_pos: Vec3,
    normal_mapping_enabled: bool,
) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 40.,
        aperture_size: 0.,
        look_from: Vec3::new(0., 0., 2.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(5., 5., 5.);

    world.add(Sphere::new(light_pos, 30., light));

    let albedo_tex = ImageTexture::load("resources/textures/wall_color.png").unwrap();
    let normal_tex = if normal_mapping_enabled {
        Some(ImageTexture::load("resources/textures/wall_n.png").unwrap())
    } else {
        None
    };
    let mat = Lambertian::new(albedo_tex, normal_tex);

    world.add(Quad::new(
        Vec3::new(-1., -1., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        mat,
    ));

    Scene {
        world,
        camera,
        background_color: Vec3::new(0., 0., 0.),
        render_config,
    }
}

#[allow(dead_code)]
pub fn create_obj_scene(render_config: RenderConfig) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 30.,
        aperture_size: 20.,
        look_from: Vec3::new(-250., 30., 150.),
        look_at: Vec3::new(-50., 0., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(15., 15., 15.);

    world.add(Sphere::new(Vec3::new(-100., 100., 40.), 35., light));
    let model = load_obj_model("resources/spider/", "spider.obj", 1., ZERO_VECTOR).unwrap();
    world.add(model);

    let image_tex = ImageTexture::load("resources/textures/tex.jpg").unwrap();
    let ground_material = Lambertian::new(image_tex, None);
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

#[allow(dead_code)]
pub fn create_obj_with_box(render_config: RenderConfig, path: &str, filename: &str) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 30.,
        aperture_size: 0.,
        look_from: Vec3::new(2., 1., 3.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(15., 15., 15.);
    let red = Lambertian::new(SolidColor::new(1., 0., 0.), None);

    world.add(Sphere::new(Vec3::new(-100., 100., 40.), 35., light));
    world.add(load_obj_model_with_default_material(path, filename, 1., ZERO_VECTOR, red).unwrap());

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

#[allow(dead_code)]
pub fn create_obj_with_triangle(render_config: RenderConfig, path: &str, filename: &str) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 30.,
        aperture_size: 0.,
        look_from: Vec3::new(0., 0., 2.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = HittableList::new();
    let light = DiffuseLight::new(15., 15., 15.);

    world.add(Sphere::new(Vec3::new(100., 0., 100.), 35., light));
    world.add(load_obj_model(path, filename, 1., ZERO_VECTOR).unwrap());

    Scene {
        world,
        camera,
        background_color: Vec3::new(0., 0., 0.),
        render_config,
    }
}
