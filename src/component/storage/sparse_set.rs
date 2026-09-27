//! Contains the [`ComponentStorage`] implementation with a sparse set.

use crate::{
    component::{Component, storage::ComponentStorage},
    entity::Entity,
};

const ABSENT: usize = usize::MAX;

pub struct SparseStorage<T: Component> {
    sparse: Vec<usize>,
    entities: Vec<Entity>,
    values: Vec<T>,
}

impl<T: Component> SparseStorage<T> {
    fn ensure_sparse_capacity(&mut self, entity: Entity) {
        if self.sparse.len() <= entity {
            self.sparse.resize(entity + 1, ABSENT);
        }
    }

    fn index_of(&self, entity: Entity) -> Option<usize> {
        let index = *self.sparse.get(entity)?;

        if index == ABSENT {
            return None;
        }

        if self.entities.get(index).copied() != Some(entity) {
            return None;
        }

        Some(index)
    }
}

impl<T: Component> ComponentStorage for SparseStorage<T> {
    type Component = T;

    fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.index_of(entity)?;
        self.values.get(index)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.index_of(entity)?;
        self.values.get_mut(index)
    }

    fn insert(&mut self, entity: Entity, component: T) {
        self.ensure_sparse_capacity(entity);

        if let Some(index) = self.index_of(entity) {
            self.values[index] = component;
            return;
        }

        let index = self.values.len();

        self.sparse[entity] = index;
        self.entities.push(entity);
        self.values.push(component);
    }

    fn remove(&mut self, entity: Entity) -> Option<T> {
        let removed_index = self.index_of(entity)?;

        let last_index = self.values.len() - 1;

        self.sparse[entity] = ABSENT;

        self.entities.swap_remove(removed_index);
        let removed_component = self.values.swap_remove(removed_index);

        if removed_index != last_index {
            let moved_entity = self.entities[removed_index];
            self.sparse[moved_entity] = removed_index;
        }

        Some(removed_component)
    }
}

impl<T: Component> Default for SparseStorage<T> {
    fn default() -> Self {
        Self {
            sparse: Vec::new(),
            entities: Vec::new(),
            values: Vec::new(),
        }
    }
}
