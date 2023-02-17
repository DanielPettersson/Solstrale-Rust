extern crate derive_more;

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use derive_more::{Constructor, Display};
use solstrale::camera::CameraConfig;
use solstrale::geo::vec3::Vec3;
use solstrale::hittable::bvh::Bvh;
use solstrale::hittable::hittable_list::HittableList;
use solstrale::hittable::sphere::Sphere;
use solstrale::hittable::triangle::Triangle;
use solstrale::hittable::{Hittable, Hittables};
use solstrale::material::texture::SolidColor;
use solstrale::material::{DiffuseLight, Lambertian};
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::{RenderConfig, Scene};

pub fn bvh_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bvh_benchmark");
    for input_param in [
        BvhInput::new(10, true),
        BvhInput::new(10, false),
        BvhInput::new(100, true),
        BvhInput::new(100, false),
        BvhInput::new(1000, true),
        BvhInput::new(1000, false),
        BvhInput::new(10000, true),
        BvhInput::new(10000, false),
    ]
    .iter()
    {
        group.throughput(Throughput::Bytes(input_param.num_triangles as u64));
        group.sample_size(50);
        group.warm_up_time(Duration::from_secs(5));
        group.measurement_time(Duration::from_secs(10));
        group.bench_with_input(
            BenchmarkId::from_parameter(input_param),
            input_param,
            |b, bvh_input| {
                b.iter_with_setup(
                    || {
                        let render_config = RenderConfig {
                            samples_per_pixel: 1,
                            shader: PathTracingShader::new(50),
                            post_processor: None,
                        };
                        create_bvh_test_scene(
                            render_config,
                            bvh_input.use_bvh,
                            bvh_input.num_triangles,
                        )
                    },
                    |scene| {
                        let (output_sender, output_receiver) = channel();
                        let (_, abort_receiver) = channel();

                        thread::spawn(move || {
                            ray_trace(
                                black_box(20),
                                black_box(10),
                                scene,
                                &output_sender,
                                &abort_receiver,
                            )
                            .unwrap();
                        });

                        for _ in output_receiver {}
                    },
                );
            },
        );
    }
    group.finish();
}

#[derive(Constructor, Display)]
#[display(fmt = "{} {}", num_triangles, use_bvh)]
struct BvhInput {
    num_triangles: u32,
    use_bvh: bool,
}

fn create_bvh_test_scene(render_config: RenderConfig, use_bvh: bool, num_triangles: u32) -> Scene {
    let camera = CameraConfig {
        vertical_fov_degrees: 20.,
        aperture_size: 0.1,
        focus_distance: 10.,
        look_from: Vec3::new(-0.5, 0., 4.),
        look_at: Vec3::new(-0.5, 0., 0.),
    };

    let mut world = HittableList::new();
    let yellow = Lambertian::new(SolidColor::new(1., 1., 0.));
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
            if let Hittables::Triangle(tri) = t {
                triangles.push(tri);
            }
        } else {
            world.add(t);
        }
    }

    if use_bvh {
        world.add(Bvh::new(triangles.as_mut_slice()))
    }

    Scene {
        world,
        camera,
        background_color: Vec3::new(0.2, 0.3, 0.5),
        render_config,
    }
}

criterion_group!(benches, bvh_benchmark);
criterion_main!(benches);
