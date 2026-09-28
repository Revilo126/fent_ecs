//! Contains the default [Component] Vec storages.

use crate::{
    component::{Component, storage::ComponentStorage},
    entity::Entity,
};

/// Default storage option for [Component]'s
pub struct VecStorage<T: Component> {
    /// The entity ID is used as the index into values
    pub(crate) values: Vec<Option<T>>,
}

impl<T: Component> ComponentStorage for VecStorage<T> {
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&T> {
        self.values.get(entity).and_then(Option::as_ref)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.values.get_mut(entity).and_then(Option::as_mut)
    }

    fn insert(&mut self, entity: Entity, component: T) {
        if self.values.len() <= entity {
            self.values.resize_with(entity + 1, || None);
        }

        self.values[entity] = Some(component);
    }

    fn remove(&mut self, entity: Entity) -> Option<T> {
        self.values.get_mut(entity).and_then(Option::take)
    }
}

impl<T: Component> Default for VecStorage<T> {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}
