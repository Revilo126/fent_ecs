//! A Fast ECS Library
//!
//! # Examples
//!
//! Run fent_ecs' examples with:
//!
//! ```text
//! cargo run --example multiple_schedules
//! ```

// Storage of ECS object
pub mod event;
pub mod schedule;
pub mod world;

pub mod component;
pub mod entity;
pub mod system;

#[cfg(test)]
mod test {
    use crate::{entity::Entities, schedule::Schedule, world::World};

    #[test]
    fn entity_bulk_alloc_free() {
        let mut entities = Entities::default();

        let mut ids = Vec::new();

        for _ in 0..1000000 {
            let e = entities.spawn();
            ids.push(e);
            assert!(entities.in_use[e.index()]);
        }

        for &id in &ids {
            entities.despawn(id);
            assert!(!entities.in_use[id.index()]);
        }
    }

    #[test]
    fn world_idk_test() {
        let mut world = World::default();

        let mut ids = Vec::new();

        for _ in 0..1000000 {
            let e = world.spawn();
            ids.push(e);
            assert!(world.entities.in_use[e.index()]);
        }

        for &id in &ids {
            world.despawn(id);
            assert!(!world.entities.in_use[id.index()]);
        }
    }

    #[should_panic(expected = "assertion `left == right`")]
    #[test]
    fn system_running() {
        fn test_system() {
            assert_eq!(1, 2);
        }

        let mut world = World::default();

        let mut schedule = Schedule::default();

        schedule.insert_system(test_system);

        schedule.initialize(&mut world).unwrap();

        schedule.run(&mut world);
    }
}
