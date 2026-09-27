use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use fent_derive::Component;
use fent_ecs::component::storage::{ComponentStorage, sparse_set::SparseStorage, vec::VecStorage};

#[allow(dead_code)]
#[derive(Clone, Component)]
struct Position {
    x: f32,
    y: f32,
}

const ENTITY_COUNT: usize = 10_000;

fn benchmark_component_storage(c: &mut Criterion) {
    let mut group = c.benchmark_group("component_storage");

    group.bench_function("vec_insert", |b| {
        b.iter(|| {
            let mut storage = VecStorage::<Position>::default();

            for entity in 0..ENTITY_COUNT {
                storage.insert(black_box(entity), Position { x: 1.0, y: 2.0 });
            }

            black_box(storage);
        });
    });

    group.bench_function("sparse_insert", |b| {
        b.iter(|| {
            let mut storage = SparseStorage::<Position>::default();

            for entity in 0..ENTITY_COUNT {
                storage.insert(black_box(entity), Position { x: 1.0, y: 2.0 });
            }

            black_box(storage);
        });
    });

    group.bench_function("vec_remove", |b| {
        b.iter_batched(
            || {
                let mut storage = VecStorage::<Position>::default();

                for entity in 0..ENTITY_COUNT {
                    storage.insert(entity, Position { x: 1.0, y: 2.0 });
                }

                storage
            },
            |mut storage| {
                for entity in 0..ENTITY_COUNT {
                    black_box(storage.remove(black_box(entity)));
                }

                black_box(storage);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sparse_remove", |b| {
        b.iter_batched(
            || {
                let mut storage = SparseStorage::<Position>::default();

                for entity in 0..ENTITY_COUNT {
                    storage.insert(entity, Position { x: 1.0, y: 2.0 });
                }

                storage
            },
            |mut storage| {
                for entity in 0..ENTITY_COUNT {
                    black_box(storage.remove(black_box(entity)));
                }

                black_box(storage);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("vec_query", |b| {
        b.iter_batched(
            || {
                let mut storage = VecStorage::<Position>::default();

                for entity in 0..ENTITY_COUNT {
                    storage.insert(entity, Position { x: 1.0, y: 2.0 });
                }

                storage
            },
            |storage| {
                let mut total = 0.0_f32;

                for entity in 0..ENTITY_COUNT {
                    if let Some(position) = storage.get(black_box(entity)) {
                        total += position.x + position.y;
                    }
                }

                black_box(total);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sparse_query", |b| {
        b.iter_batched(
            || {
                let mut storage = SparseStorage::<Position>::default();

                for entity in 0..ENTITY_COUNT {
                    storage.insert(entity, Position { x: 1.0, y: 2.0 });
                }

                storage
            },
            |storage| {
                let mut total = 0.0_f32;

                for entity in 0..ENTITY_COUNT {
                    if let Some(position) = storage.get(black_box(entity)) {
                        total += position.x + position.y;
                    }
                }

                black_box(total);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, benchmark_component_storage);
criterion_main!(benches);
