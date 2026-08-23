//! Resources are shared objects inside a world,
//! Only one of a type can exist at once.
//!
//! They can be removed or inserted.

use std::any::Any;

use crate::{component::identification::resource_id, world::World};

// Resource trait
pub trait Resource: Any {
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
}

impl Resources {
    pub fn insert<R>(&mut self, resource: R)
    where
        R: Resource + 'static,
    {
        let id = resource_id::<R>();

        if self.resources.len() <= id {
            self.resources.resize_with(id + 1, || None);
        }

        if self.resources[id].is_none() {
            self.resources[id] = Some(Box::new(resource));
            return;
        }

        log::error!("Tried to insert a resource when already existing!");
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

        if let Some(resource) = self.resources.get_mut(id) {
            *resource = None;
        } else {
            log::warn!("Tried removing a non-existent resource!");
        }
    }
}
