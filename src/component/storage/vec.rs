//! Contains the default [Component] Vec storages.

use crate::component::{Component, storage::ComponentStorage};

/// Default storage option for [Component]'s
pub struct VecStorage<T: Component> {
    /// The entity ID is used as the index into values
    pub(crate) values: Vec<Option<T>>,
}

impl<T: Component> ComponentStorage for VecStorage<T> {
    type Component = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index).and_then(Option::as_ref)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index).and_then(Option::as_mut)
    }

    fn insert(&mut self, index: usize, component: T) {
        if self.values.len() <= index {
            self.values.resize_with(index + 1, || None);
        }

        self.values[index] = Some(component);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        self.values.get_mut(index).and_then(Option::take)
    }
}

impl<T: Component> Default for VecStorage<T> {
    fn default() -> Self {
        Self { values: Vec::new() }
    }
}
