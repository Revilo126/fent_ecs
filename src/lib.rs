//! A Fast ECS Library

// Storage of ECS object
pub mod schedule;
pub mod world;

pub mod component;
pub mod entity;
pub mod system;

#[cfg(test)]
mod test {
    // Entity Tests

    use crate::entity::Entities;
    #[test]
    fn entity_bulk_alloc_free() {
        let mut entities = Entities::default();

        let mut ids = Vec::new();

        for _ in 0..10000 {
            let e = entities.alloc();
            ids.push(e);
            assert!(entities.in_use[e]);
        }

        for &id in &ids {
            entities.free(id);
            assert!(!entities.in_use[id]);
        }
    }
}
