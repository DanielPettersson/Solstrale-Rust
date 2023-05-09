use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use crate::scenes::{create_test_scene, new_bvh_test_scene};
use derive_more::{Constructor, Display};
use solstrale::ray_trace;
use solstrale::renderer::shader::PathTracingShader;
use solstrale::renderer::RenderConfig;

#[path = "../tests/scenes.rs"]
mod scenes;

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
        group.sample_size(25);
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
                        new_bvh_test_scene(
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

pub fn scene_benchmark(c: &mut Criterion) {
    c.bench_function("scene_benchmark", |b| {
        b.iter_with_setup(
            || {
                let render_config = RenderConfig {
                    samples_per_pixel: 1,
                    shader: PathTracingShader::new(50),
                    post_processor: None,
                };
                create_test_scene(render_config)
            },
            |scene| {
                let (output_sender, output_receiver) = channel();
                let (_, abort_receiver) = channel();

                thread::spawn(move || {
                    ray_trace(
                        black_box(100),
                        black_box(50),
                        scene,
                        &output_sender,
                        &abort_receiver,
                    )
                    .unwrap();
                });

                for _ in output_receiver {}
            },
        )
    });
}

#[derive(Constructor, Display)]
#[display(fmt = "{} {}", num_triangles, use_bvh)]
struct BvhInput {
    num_triangles: u32,
    use_bvh: bool,
}

criterion_group!(benches, bvh_benchmark, scene_benchmark);
criterion_main!(benches);
