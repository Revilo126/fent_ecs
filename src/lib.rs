//! A Fast ECS Library

// Storage of ECS object
pub mod schedule;
pub mod world;

pub mod component;
pub mod entity;
pub mod system;

#[cfg(test)]
mod test {
    use crate::{component::identification::component_id, entity::Entities};

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

    #[test]
    fn component_type() {
        struct Obj1;
        struct Obj2;

        let i = component_id::<Obj1>();
        assert_eq!(i, 0);
        let j = component_id::<Obj2>();
        assert_eq!(j, 1);

        let i = component_id::<Obj1>();
        assert_eq!(i, 0);
    }
}
