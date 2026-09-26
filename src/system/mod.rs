//! Systems allow interactions inside an ECS application.
//!
//! In Fent ECS they are simply functions


use crate::{
    system::access::Access,
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub(crate) mod access;
pub mod arg_impl;
pub mod system_arg;

pub trait System: Send + Sync + 'static {
    fn run(&mut self, world: &mut World);

    fn access(&self) -> &Access;

    /// # Safety
    ///
    /// The caller must ensure that all accesses requested by this system are
    /// compatible with the accesses already active in the world. In particular,
    /// mutable accesses must not alias any other read or write access.
    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>);
}

pub trait IntoSystem<Marker>: Sized {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<T: System> IntoSystem<()> for T {
    type System = T;

    fn into_system(self) -> Self::System {
        self
    }
}
