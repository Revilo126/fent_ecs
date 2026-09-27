//! A Fast ECS Library

// Storage of ECS object
pub mod schedule;
pub mod world;

pub mod component;
pub mod entity;
pub mod system;

#[cfg(test)]
mod test {
    use crate::{
        entity::Entities,
        system::{IntoSystem, System},
        world::{World, unsafe_world_cell::UnsafeWorldCell},
    };

    #[test]
    fn entity_bulk_alloc_free() {
        let mut entities = Entities::default();

        let mut ids = Vec::new();

        for _ in 0..1000000 {
            let e = entities.alloc();
            ids.push(e);
            assert!(entities.in_use[e]);
        }

        for &id in &ids {
            entities.free(id);
            assert!(!entities.in_use[id]);
        }
    }

    #[test]
    fn world_idk_test() {
        let mut world = World::default();

        let mut ids = Vec::new();

        for _ in 0..1000000 {
            let e = world.spawn();
            ids.push(e);
            assert!(world.entities.in_use[e]);
        }

        for &id in &ids {
            world.despawn(id);
            assert!(!world.entities.in_use[id]);
        }
    }

    #[should_panic(expected = "assertion `left == right`")]
    #[test]
    fn system_running() {
        fn test_system(_world: &mut World) {
            assert_eq!(1, 2);
        }

        let mut world = World::default();

        let unsafe_world = unsafe { UnsafeWorldCell::from_world(&mut world) };

        unsafe { test_system.into_system().unsafe_run(unsafe_world) };
    }
}
