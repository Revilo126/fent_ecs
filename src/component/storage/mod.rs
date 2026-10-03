//! Contains the objects used for storing and retrieving components

use std::any::Any;

use crate::component::identification::component_id;

pub mod sparse_set;
pub mod vec;

/// The trait to be implemented by each component storage.
pub trait ComponentStorage: 'static + Any + Default {
    type Component;

    fn get(&self, index: usize) -> Option<&Self::Component>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Component>;
    fn insert(&mut self, index: usize, component: Self::Component);
    fn remove(&mut self, index: usize) -> Option<Self::Component>;
}

pub trait ErasedComponentStorage: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove_untyped(&mut self, index: usize);
}

impl<T> ErasedComponentStorage for T
where
    T: ComponentStorage,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_untyped(&mut self, index: usize) {
        self.remove(index);
    }
}

/// Contains and handles all storages for components,
/// Such as adding the storages when needed.
///
/// Storages are stored in a Vec of all storages.
/// The ID for each Compoennt is the ComponentId of said object.
#[derive(Default)]
pub struct Components {
    pub(crate) storages: Vec<Option<Box<dyn ErasedComponentStorage>>>,
}

impl Components {
    fn ensure_storage<S>(&mut self) -> usize
    where
        S: ComponentStorage + Default,
    {
        let id = component_id::<S::Component>();

        if self.storages.len() <= id {
            self.storages.resize_with(id + 1, || None);
        }

        if self.storages[id].is_none() {
            self.storages[id] = Some(Box::new(S::default()));
        }

        id
    }

    pub fn insert<S>(&mut self)
    where
        S: ComponentStorage + Default,
    {
        let _ = self.ensure_storage::<S>();
    }

    pub fn get_or_insert<S>(&mut self) -> Option<&S>
    where
        S: ComponentStorage + Default,
    {
        let id = self.ensure_storage::<S>();

        self.storages
            .get(id)?
            .as_ref()?
            .as_any()
            .downcast_ref::<S>()
    }

    pub fn get_mut_or_insert<S>(&mut self) -> Option<&mut S>
    where
        S: ComponentStorage + Default,
    {
        let id = self.ensure_storage::<S>();

        self.storages
            .get_mut(id)?
            .as_mut()?
            .as_any_mut()
            .downcast_mut::<S>()
    }

    pub fn get<S>(&self) -> Option<&S>
    where
        S: ComponentStorage,
    {
        let id = component_id::<S::Component>();

        self.storages
            .get(id)?
            .as_ref()?
            .as_any()
            .downcast_ref::<S>()
    }

    pub fn get_mut<S>(&mut self) -> Option<&mut S>
    where
        S: ComponentStorage,
    {
        let id = component_id::<S::Component>();

        self.storages
            .get_mut(id)?
            .as_mut()?
            .as_any_mut()
            .downcast_mut::<S>()
    }
}
