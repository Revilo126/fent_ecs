use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use fent_derive::Resource;
use fent_ecs::{schedule::Schedule, system::impl_param::ResMut, world::World};

#[derive(Resource)]
struct Counter(u64);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 = counter.0.wrapping_add(1);
}

fn benchmark_systems(c: &mut Criterion) {
    let mut world = World::default();
    let mut schedule = Schedule::default();
    schedule.initialize(&mut world).unwrap();

    let mut group = c.benchmark_group("systems");

    group.bench_function("empty_run", |b| {
        b.iter(|| {
            schedule.run(&mut world);
            black_box(&schedule);
        });
    });

    schedule.insert_system(increment);
    schedule.initialize(&mut world).unwrap();

    world.insert_resource(Counter(0));

    group.bench_function("single_system", |b| {
        b.iter(|| {
            schedule.run(&mut world);
            black_box(&schedule);
        })
    });
}

criterion_group!(benches, benchmark_systems);
criterion_main!(benches);
