use solstrale::camera::CameraConfig;
use solstrale::geo::transformation::{NopTransformer, RotationY, Transformations, Translation};
use solstrale::geo::vec3::Vec3;
use solstrale::geo::Uv;
use solstrale::hittable::ConstantMedium;
use solstrale::hittable::Sphere;
use solstrale::hittable::Triangle;
use solstrale::hittable::{Bvh, Quad};
use solstrale::loader::obj::Obj;
use solstrale::loader::Loader;
use solstrale::material::texture::{ImageMap, SolidColor};
use solstrale::material::{Dielectric, DiffuseLight, Lambertian};
use solstrale::renderer::{RenderConfig, Scene};

pub fn create_test_scene(render_config: RenderConfig) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        look_from: Vec3::new(-5., 3., 6.),
        look_at: Vec3::new(0.25, 1., 0.),
    };

    let mut world = Vec::new();

    let image_tex = ImageMap::load("resources/textures/tex.jpg").unwrap();

    let ground_material = Lambertian::new(image_tex, None);
    let glass_mat = Dielectric::new(SolidColor::new(1., 1., 1.), None, 1.5);
    let light_mat = DiffuseLight::new(10., 10., 10., None);
    let red_mat = Lambertian::new(SolidColor::new(1., 0., 0.), None);

    world.push(Quad::new(
        Vec3::new(-5., 0., -15.),
        Vec3::new(20., 0., 0.),
        Vec3::new(0., 0., 20.),
        ground_material,
        &NopTransformer(),
    ));
    world.push(Sphere::new(Vec3::new(-1., 1., 0.), 1., glass_mat));
    world.append(&mut Quad::new_box(
        Vec3::new(0., 0., -0.5),
        Vec3::new(1., 2., 0.5),
        red_mat.clone(),
        &RotationY::new(15.),
    ));
    world.push(ConstantMedium::new(
        Bvh::new(Quad::new_box(
            Vec3::new(0., 0., -0.5),
            Vec3::new(1., 2., 0.5),
            red_mat.clone(),
            &Translation::new(Vec3::new(0., 0., 1.)),
        )),
        0.1,
        Vec3::new(1., 1., 1.),
    ));
    world.append(&mut Quad::new_box(
        Vec3::new(-1., 2., 0.),
        Vec3::new(-0.5, 2.5, 0.5),
        red_mat.clone(),
        &NopTransformer(),
    ));

    let nop_transformer = NopTransformer();

    let mut balls = Vec::new();
    for ii in (0..10).step_by(2) {
        let i = ii as f64 * 0.1;
        for jj in (0..10).step_by(2) {
            let j = jj as f64 * 0.1;
            for kk in (0..10).step_by(2) {
                let k = kk as f64 * 0.1;
                balls.push(Triangle::new(
                    Vec3::new(i, j + 0.05, k + 0.8),
                    Vec3::new(i, j, k + 0.8),
                    Vec3::new(i, j + 0.05, k),
                    red_mat.clone(),
                    &nop_transformer,
                ));
            }
        }
    }
    world.push(Bvh::new(balls));

    world.push(Triangle::new(
        Vec3::new(1., 0.1, 2.),
        Vec3::new(3., 0.1, 2.),
        Vec3::new(2., 0.1, 1.),
        red_mat,
        &nop_transformer,
    ));

    // Lights

    world.push(Sphere::new(Vec3::new(10., 5., 10.), 10., light_mat.clone()));
    world.push(Quad::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 0., 2.),
        light_mat.clone(),
        &Transformations::new(vec![
            Box::new(RotationY::new(45.)),
            Box::new(Translation::new(Vec3::new(-1., 10., -1.))),
        ]),
    ));
    world.push(Triangle::new(
        Vec3::new(-2., 1., -3.),
        Vec3::new(0., 1., -3.),
        Vec3::new(-1., 2., -3.),
        light_mat,
        &nop_transformer,
    ));

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.), None);
    let light = DiffuseLight::new(10., 10., 10., None);
    world.push(Sphere::new(Vec3::new(0., 4., 10.), 4., light));

    let nop_transformer = NopTransformer();
    let mut triangles = Vec::new();
    for x in 0..num_triangles {
        let cx = x as f64 - num_triangles as f64 / 2.;
        let t = Triangle::new(
            Vec3::new(cx, -0.5, 0.),
            Vec3::new(cx + 1., -0.5, 0.),
            Vec3::new(cx + 0.5, 0.5, 0.),
            yellow.clone(),
            &nop_transformer,
        );
        if use_bvh {
            triangles.push(t);
        } else {
            world.push(t);
        }
    }

    if use_bvh {
        world.push(Bvh::new(triangles))
    }

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.), None);
    let light = DiffuseLight::new(10., 10., 10., None);
    if add_light {
        world.push(Sphere::new(Vec3::new(0., 100., 0.), 20., light))
    }
    world.push(Sphere::new(Vec3::new(0., 0., 0.), 0.5, yellow));

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let light = DiffuseLight::new(10., 10., 10., None);

    world.push(Sphere::new(Vec3::new(50., 50., 50.), 20., light));

    let tex = ImageMap::load("resources/textures/checker.jpg").unwrap();
    let checker_mat = Lambertian::new(tex, None);

    world.push(Triangle::new_with_tex_coords(
        Vec3::new(-1., 0., 0.),
        Vec3::new(1., 0., 0.),
        Vec3::new(0., 2., 0.),
        Uv::new(-1., -1.),
        Uv::new(2., -1.),
        Uv::new(0., 2.),
        checker_mat,
        &NopTransformer(),
    ));

    Scene {
        world: Bvh::new(world),
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
        look_from: Vec3::new(0.2, 0.2, 2.),
        look_at: Vec3::new(0., 0., 0.),
    };

    let mut world = Vec::new();
    let light = DiffuseLight::new(45., 45., 45., None);

    world.push(Sphere::new(light_pos, 5., light));

    let albedo_tex = ImageMap::load("resources/textures/wall_color.png").unwrap();
    let normal_tex = if normal_mapping_enabled {
        Some(ImageMap::load("resources/textures/wall_n.png").unwrap())
    } else {
        None
    };
    let mat = Lambertian::new(albedo_tex, normal_tex);
    let red = Lambertian::new(SolidColor::new(1., 0., 0.), None);

    world.append(&mut Quad::new_box(
        Vec3::new(-0.1, -0.1, 0.),
        Vec3::new(0.1, 0.1, 1.),
        red,
        &NopTransformer(),
    ));

    world.push(Quad::new(
        Vec3::new(-1., -1., 0.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        mat,
        &NopTransformer(),
    ));

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let light = DiffuseLight::new(15., 15., 15., None);

    world.push(Sphere::new(Vec3::new(-100., 100., 40.), 35., light));
    let model = Obj::new("resources/spider/", "spider.obj")
        .load(&NopTransformer(), None)
        .unwrap();
    world.push(model);

    let image_tex = ImageMap::load("resources/textures/tex.jpg").unwrap();
    let ground_material = Lambertian::new(image_tex, None);
    world.push(Quad::new(
        Vec3::new(-200., -30., -200.),
        Vec3::new(400., 0., 0.),
        Vec3::new(0., 0., 400.),
        ground_material,
        &NopTransformer(),
    ));

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let light = DiffuseLight::new(15., 15., 15., None);
    let red = Lambertian::new(SolidColor::new(1., 0., 0.), None);

    world.push(Sphere::new(Vec3::new(-100., 100., 40.), 35., light));
    world.push(
        Obj::new(path, filename)
            .load(&NopTransformer(), Some(red))
            .unwrap(),
    );

    Scene {
        world: Bvh::new(world),
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

    let mut world = Vec::new();
    let light = DiffuseLight::new(15., 15., 15., None);

    world.push(Sphere::new(Vec3::new(100., 0., 100.), 35., light));
    world.push(
        Obj::new(path, filename)
            .load(&NopTransformer(), None)
            .unwrap(),
    );

    Scene {
        world: Bvh::new(world),
        camera,
        background_color: Vec3::new(0., 0., 0.),
        render_config,
    }
}

#[allow(dead_code)]
pub fn create_light_attenuation_scene(
    render_config: RenderConfig,
    attenuation_half_length: Option<f64>,
) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.,
        look_from: Vec3::new(0., 1., 2.),
        look_at: Vec3::new(0., 0.2, 0.),
    };

    let mut world = Vec::new();
    let light = DiffuseLight::new(25., 25., 25., attenuation_half_length);
    let red = Lambertian::new(SolidColor::new(1., 0., 0.), None);
    let green = Lambertian::new(SolidColor::new(0., 1., 0.), None);
    let blue = Lambertian::new(SolidColor::new(0., 0., 1.), None);
    let glass = Dielectric::new(SolidColor::new(0.8, 0.8, 0.8), None, 1.5);

    world.push(Sphere::new(Vec3::new(0., 0.2, 0.), 0.03, light));
    world.push(Sphere::new(Vec3::new(0.25, 0.1, 0.25), 0.1, green));
    world.push(Sphere::new(Vec3::new(0.25, 0.1, -0.5), 0.1, blue));
    world.push(Sphere::new(Vec3::new(-0.1, 0.1, -0.1), 0.1, glass));
    world.push(Quad::new(
        Vec3::new(-1., 0., -1.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 0., 2.),
        red,
        &NopTransformer(),
    ));

    Scene {
        world: Bvh::new(world),
        camera,
        background_color: Vec3::new(0., 0., 0.),
        render_config,
    }
}
