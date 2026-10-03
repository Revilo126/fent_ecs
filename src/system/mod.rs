//! Systems allow interactions inside an ECS application.
//!
//! In Fent ECS they are simply functions

use crate::{
    system::access::{Access, AccessError},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub mod access;
pub mod function_system;
pub mod impl_param;
pub mod system_param;

pub trait System: Send + Sync + 'static {
    fn initialize(&mut self, world: &mut World);

    fn run(&mut self, world: &mut World);

    fn access(&self, access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    ///
    /// The caller must ensure that all accesses requested by this system are
    /// compatible with the accesses already active in the world. In particular,
    /// mutable accesses must not alias any other read or write access.
    unsafe fn unsafe_run(&mut self, world: UnsafeWorldCell<'_>);

    /// Runs after the system is finished, useful for applying actions after completion.
    fn apply(&mut self, world: &mut World);
}

pub trait IntoSystem<Marker = ()> {
    type System: System;

    fn into_system(self) -> Self::System;
}
