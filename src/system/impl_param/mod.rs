//! Provides some basic [`crate::system::system_param::SystemParam`] implementations.

use std::ops::{Deref, DerefMut};

use crate::{
    component::resource::Resource,
    system::{
        access::{Access, AccessError},
        system_param::SystemParam,
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub mod queries;

pub struct Res<'w, T: 'static + Resource> {
    value: &'w T,
}

pub struct ResMut<'w, T: 'static + Resource> {
    value: &'w mut T,
}

impl<T: 'static + Resource> Deref for Res<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}
impl<T: 'static + Resource> Deref for ResMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}
impl<T: 'static + Resource> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: 'static + Resource> SystemParam for Res<'_, T> {
    type State = ();
    type Item<'w> = Res<'w, T>;

    fn init_state(_world: &mut World) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.read::<T>()
    }

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: UnsafeWorldCell<'w>) -> Res<'w, T> {
        // SAFETY: access was validated, so no live mutable access to T exists.
        let value = unsafe { world.world().get_resource::<T>() }.unwrap_or_else(|| {
            panic!(
                "Resource not initialised yet! ({})",
                std::any::type_name::<T>()
            )
        });
        Res { value }
    }
}

impl<T: 'static + Resource> SystemParam for ResMut<'_, T> {
    type State = ();
    type Item<'w> = ResMut<'w, T>;

    fn init_state(_world: &mut World) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.write::<T>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> ResMut<'w, T> {
        // SAFETY: access was validated to T
        let value = unsafe { world.world_mut().get_mut_resource::<T>() }.unwrap_or_else(|| {
            panic!(
                "Resource not initialised yet! ({})",
                std::any::type_name::<T>()
            )
        });
        ResMut { value }
    }
}

/// [`Local`] allows systems to store values, these variables are stored per-system.
pub struct Local<'s, T: Default + Send + Sync + 'static> {
    value: &'s mut T,
}

impl<T: Default + Send + Sync + 'static> Deref for Local<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}

impl<T: Default + Send + Sync + 'static> DerefMut for Local<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<T: Default + Send + Sync + 'static> SystemParam for Local<'_, T> {
    type State = T;
    type Item<'w> = Local<'w, T>;

    fn init_state(_world: &mut World) -> Self::State {
        T::default()
    }

    fn access(_state: &Self::State, _access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        _world: UnsafeWorldCell<'w>,
    ) -> Local<'w, T> {
        Local { value: state }
    }
}
