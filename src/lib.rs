//! Fent ECS is a simple, fast and ergonomic entity component system.
//!
//! View the source code on [Github](https://github.com/Revilo126/fent_ecs).
//! And if you find anything that's worth fixing, send a pull request as all support and help is welcomed.
//!
//! # Examples
//!
//! Run the avaliable examples with the following commands:
//!
//! ```console
//! cargo run --example multiple_schedules
//! cargo run --example readme_example # The code seen in the [README.md](https://github.com/Revilo126/fent_ecs?tab=readme-ov-file)
//! ```

pub mod event;
pub mod message;
pub mod schedule;
pub mod world;

pub mod component;
pub mod entity;
pub mod system;

pub mod prelude;

#[cfg(test)]
mod test {
    use crate::prelude::*;

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

        schedule.add_system(test_system);

        schedule.run(&mut world);
    }
}
