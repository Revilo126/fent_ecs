//! Contains the default [`Component`] Vec storages.

use std::marker::PhantomData;

use crate::{
    component::{Component, storage::ComponentStorage},
    entity::Entity,
};

/// Default storage option for [`Component`]'s
pub struct VecStorage<T: Component> {
    /// The index of each Component is at it's [`ComponentId`]
    pub(crate) values: Vec<Option<T>>,
    _marker: PhantomData<T>,
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
