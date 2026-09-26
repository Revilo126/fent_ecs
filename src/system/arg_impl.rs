//! This module contains many default system argument implementations such as Res an ResMut.

use std::ops::{Deref, DerefMut};

use crate::{
    component::resource::Resource,
    system::{
        access::{Access, AccessError},
        system_arg::SystemArg,
    },
    world::unsafe_world_cell::UnsafeWorldCell,
};

pub struct Res<R: Resource> {
    value: *const R,
    _marker: std::marker::PhantomData<R>,
}

impl<R: Resource> Res<R> {
    pub fn new(value: &R) -> Self {
        Self {
            value: value as *const R,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<R> SystemArg for Res<R>
where
    R: Resource + 'static,
{
    type Item<'world> = Res<R>;

    fn access(access: &mut Access) -> Result<(), AccessError> {
        access.read::<R>()
    }

    unsafe fn fetch<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world> {
        let resource = unsafe { world.world().get_resource::<R>().unwrap() };

        Res::new(resource)
    }
}

impl<R: Resource> Deref for Res<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

pub struct ResMut<R: Resource> {
    value: *mut R,
    _marker: std::marker::PhantomData<R>,
}

impl<R: Resource> ResMut<R> {
    pub fn new(value: &mut R) -> Self {
        Self {
            value: value as *mut R,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<R> SystemArg for ResMut<R>
where
    R: Resource + 'static,
{
    type Item<'world> = ResMut<R>;

    fn access(access: &mut Access) -> Result<(), AccessError> {
        access.write::<R>()
    }

    unsafe fn fetch<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world> {
        let resource = unsafe { world.world_mut().get_mut_resource::<R>().unwrap() };
        ResMut::new(resource)
    }
}

impl<R: Resource> Deref for ResMut<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value }
    }
}

impl<R: Resource> DerefMut for ResMut<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.value }
    }
}
