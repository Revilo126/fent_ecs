//! Contains the objects used for storing and retrieving components

use std::any::Any;

use crate::{
    component::{Component, identification::component_id},
    world::change_detection::{ComponentTicks, Tick},
};

pub mod sparse_set;
pub mod vec;

/// The trait to be implemented by each component storage.
pub trait ComponentStorage: Send + Sync + 'static + Any + Default {
    type Component;

    fn get(&self, index: usize) -> Option<&Self::Component>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Component>;
    fn insert(&mut self, index: usize, component: Self::Component);
    fn remove(&mut self, index: usize) -> Option<Self::Component>;
}

pub trait ErasedComponentStorage: Send + Sync + Any {
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
    pub(crate) ticks: Vec<Vec<Option<ComponentTicks>>>,
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

    fn ticks_column(&mut self, id: usize) -> &mut Vec<Option<ComponentTicks>> {
        if self.ticks.len() <= id {
            self.ticks.resize_with(id + 1, Vec::new);
        }
        &mut self.ticks[id]
    }

    pub(crate) fn stamp_insert<C: Component>(&mut self, index: usize, tick: Tick) {
        let col = self.ticks_column(component_id::<C>());
        if col.len() <= index {
            col.resize_with(index + 1, || None);
        }
        match &mut col[index] {
            Some(t) => t.changed = tick,
            empty => *empty = Some(ComponentTicks::new(tick)),
        }
    }

    pub fn get_ticks<C: Component>(&self, index: usize) -> Option<&ComponentTicks> {
        self.ticks.get(component_id::<C>())?.get(index)?.as_ref()
    }

    pub fn get_ticks_mut<C: Component>(&mut self, index: usize) -> Option<&mut ComponentTicks> {
        self.ticks
            .get_mut(component_id::<C>())?
            .get_mut(index)?
            .as_mut()
    }

    pub(crate) fn clear_ticks<C: Component>(&mut self, index: usize) {
        if let Some(slot) = self
            .ticks
            .get_mut(component_id::<C>())
            .and_then(|c| c.get_mut(index))
        {
            *slot = None;
        }
    }

    pub(crate) fn clear_all_ticks(&mut self, index: usize) {
        for col in &mut self.ticks {
            if let Some(slot) = col.get_mut(index) {
                *slot = None;
            }
        }
    }
}
