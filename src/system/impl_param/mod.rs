//! Provides some basic [`crate::system::system_param::SystemParam`] implementations.

use std::ops::{Deref, DerefMut};

use crate::{
    component::resource::Resource,
    system::{
        access::{Access, AccessError},
        meta::SystemMeta,
        system_param::SystemParam,
    },
    world::{
        World,
        change_detection::{ComponentTicks, Tick},
        unsafe_world_cell::UnsafeWorldCell,
    },
};

pub mod command;
pub mod messages;
pub mod non_send;
pub mod queries;

pub struct Res<'w, T: 'static + Resource> {
    value: &'w T,
    ticks: &'w ComponentTicks,
    last_run: Tick,
}

pub struct ResMut<'w, T: 'static + Resource> {
    value: &'w mut T,
    ticks: &'w mut ComponentTicks,
    last_run: Tick,
    this_run: Tick,
}

impl<T: 'static + Resource> Res<'_, T> {
    pub fn is_added(&self) -> bool {
        self.ticks.added.is_newer_than(self.last_run)
    }
    pub fn is_changed(&self) -> bool {
        self.ticks.changed.is_newer_than(self.last_run)
    }
}

impl<T: 'static + Resource> ResMut<'_, T> {
    pub fn is_added(&self) -> bool {
        self.ticks.added.is_newer_than(self.last_run)
    }
    pub fn is_changed(&self) -> bool {
        self.ticks.changed.is_newer_than(self.last_run)
    }

    pub fn set_changed(&mut self) {
        self.ticks.changed = self.this_run;
    }

    pub fn bypass_change_detection(&mut self) -> &mut T {
        self.value
    }
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
        self.ticks.changed = self.this_run;
        self.value
    }
}

impl<T: 'static + Resource> SystemParam for Res<'_, T> {
    type State = ();
    type Item<'w> = Res<'w, T>;

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.read::<T>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        meta: &SystemMeta,
        world: UnsafeWorldCell<'w>,
        _this_run: Tick,
    ) -> Res<'w, T> {
        let (value, ticks) = unsafe { world.world().resources().get_with_ticks::<T>() }
            .unwrap_or_else(|| {
                panic!(
                    "Resource not initialised yet! ({})",
                    std::any::type_name::<T>()
                )
            });
        Res {
            value,
            ticks,
            last_run: meta.last_run(),
        }
    }
}

impl<T: 'static + Resource> SystemParam for ResMut<'_, T> {
    type State = ();
    type Item<'w> = ResMut<'w, T>;

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.write::<T>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        meta: &SystemMeta,
        world: UnsafeWorldCell<'w>,
        this_run: Tick,
    ) -> ResMut<'w, T> {
        // SAFETY: write access to T was validated.
        let (value, ticks) = unsafe { world.world_mut().resources_mut().get_mut_with_ticks::<T>() }
            .unwrap_or_else(|| {
                panic!(
                    "Resource not initialised yet! ({})",
                    std::any::type_name::<T>()
                )
            });
        ResMut {
            value,
            ticks,
            last_run: meta.last_run(),
            this_run,
        }
    }
}

impl<T: 'static + Resource> SystemParam for Option<Res<'_, T>> {
    type State = ();
    type Item<'w> = Option<Res<'w, T>>;

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.read::<T>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        meta: &SystemMeta,
        world: UnsafeWorldCell<'w>,
        _this_run: Tick,
    ) -> Self::Item<'w> {
        unsafe { world.world().resources().get_with_ticks::<T>() }.map(|(value, ticks)| Res {
            value,
            ticks,
            last_run: meta.last_run(),
        })
    }
}

impl<T: 'static + Resource> SystemParam for Option<ResMut<'_, T>> {
    type State = ();
    type Item<'w> = Option<ResMut<'w, T>>;

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.read::<T>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        meta: &SystemMeta,
        world: UnsafeWorldCell<'w>,
        this_run: Tick,
    ) -> Self::Item<'w> {
        unsafe { world.world_mut().resources_mut().get_mut_with_ticks::<T>() }.map(
            |(value, ticks)| ResMut {
                value,
                ticks,
                last_run: meta.last_run(),
                this_run,
            },
        )
    }
}

/// Allows systems to store values, these variables are stored per-system.
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

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {
        T::default()
    }

    fn access(_state: &Self::State, _access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        _meta: &SystemMeta,
        _world: UnsafeWorldCell<'w>,
        _this_run: Tick,
    ) -> Local<'w, T> {
        Local { value: state }
    }
}
