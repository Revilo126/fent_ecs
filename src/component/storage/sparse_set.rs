//! Contains the [`ComponentStorage`] implementation with a sparse set.

use crate::component::{Component, storage::ComponentStorage};

const ABSENT: usize = usize::MAX;

pub struct SparseStorage<T: Component> {
    sparse: Vec<usize>,
    entities: Vec<usize>,
    values: Vec<T>,
}

impl<T: Component> SparseStorage<T> {
    fn ensure_sparse_capacity(&mut self, index: usize) {
        if self.sparse.len() <= index {
            self.sparse.resize(index + 1, ABSENT);
        }
    }

    #[inline]
    fn index_of(&self, index: usize) -> Option<usize> {
        match self.sparse.get(index) {
            Some(&index) if index != ABSENT => Some(index),
            _ => None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.entities.iter().copied().zip(self.values.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.entities.iter().copied().zip(self.values.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl<T: Component> ComponentStorage for SparseStorage<T> {
    type Component = T;

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        let index = self.index_of(index)?;
        self.values.get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let index = self.index_of(index)?;
        self.values.get_mut(index)
    }

    fn insert(&mut self, index: usize, component: T) {
        self.ensure_sparse_capacity(index);

        let slot = self.sparse[index];
        if slot != ABSENT {
            self.values[slot] = component;
            return;
        }

        self.sparse[index] = self.values.len();
        self.entities.push(index);
        self.values.push(component);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        let removed_index = self.index_of(index)?;
        let last_index = self.values.len() - 1;

        self.sparse[index] = ABSENT;

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
