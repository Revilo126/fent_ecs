//! Resources are shared objects inside a world,
//! Only one of a type can exist at once.
//!
//! They can be removed or inserted.

use std::any::Any;

use crate::{
    component::identification::resource_id,
    world::{
        World,
        change_detection::{ComponentTicks, Tick},
    },
};

// Resource trait
pub trait Resource: Send + Sync + Any {
    fn on_add(&self) -> Option<ResourceAction> {
        None
    }

    fn on_insert(&self) -> Option<ResourceAction> {
        None
    }

    fn on_remove(&self) -> Option<ResourceAction> {
        None
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Not working right now
type ResourceAction = fn(&mut dyn Resource, &mut World);

/// To store [`Resource`]'s, a seperate structure is used,
/// This simplifies the use and management of Resources.
#[derive(Default)]
pub struct Resources {
    pub(crate) resources: Vec<Option<Box<dyn Resource>>>,
    pub(crate) ticks: Vec<Option<ComponentTicks>>,
}

impl Resources {
    /// Insert a [`Resource`] into the storage.
    ///
    /// When a [`Resource`] of the same type already exists
    /// it does not do anything.
    pub fn insert<R>(&mut self, resource: R, tick: Tick)
    where
        R: Resource + 'static,
    {
        let id = resource_id::<R>();

        if self.resources.len() <= id {
            self.resources.resize_with(id + 1, || None);
            self.ticks.resize_with(id + 1, || None);
        }

        self.resources[id] = Some(Box::new(resource));

        match &mut self.ticks[id] {
            Some(t) => t.changed = tick,
            empty => *empty = Some(ComponentTicks::new(tick)),
        }
    }

    pub fn get<R>(&self) -> Option<&R>
    where
        R: Resource + 'static,
    {
        let id = resource_id::<R>();

        self.resources
            .get(id)?
            .as_ref()?
            .as_any()
            .downcast_ref::<R>()
    }

    pub fn get_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Resource + 'static,
    {
        let id = resource_id::<R>();

        self.resources
            .get_mut(id)?
            .as_mut()?
            .as_any_mut()
            .downcast_mut::<R>()
    }

    pub fn remove<R>(&mut self)
    where
        R: Resource + 'static,
    {
        let id = resource_id::<R>();
        if let Some(v) = self.resources.get_mut(id) {
            *v = None;
        }
        if let Some(t) = self.ticks.get_mut(id) {
            *t = None;
        }
    }

    pub(crate) fn get_mut_with_ticks<R: Resource>(
        &mut self,
    ) -> Option<(&mut R, &mut ComponentTicks)> {
        let id = resource_id::<R>();
        let Resources { resources, ticks } = self;
        let value = resources
            .get_mut(id)?
            .as_mut()?
            .as_any_mut()
            .downcast_mut::<R>()?;
        let ticks = ticks.get_mut(id)?.as_mut()?;
        Some((value, ticks))
    }

    pub(crate) fn get_with_ticks<R: Resource>(&self) -> Option<(&R, &ComponentTicks)> {
        let id = resource_id::<R>();
        Some((
            self.resources
                .get(id)?
                .as_ref()?
                .as_any()
                .downcast_ref::<R>()?,
            self.ticks.get(id)?.as_ref()?,
        ))
    }
}
