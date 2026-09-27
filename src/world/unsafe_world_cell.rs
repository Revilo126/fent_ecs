//! [`UnsafeWorldCell`]'s provide a way to access a [`World`] in a sketchy way.

use std::marker::PhantomData;

use crate::world::World;

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'world> {
    world: *mut World,
    marker: PhantomData<&'world mut World>,
}

impl<'world> UnsafeWorldCell<'world> {
    pub unsafe fn from_world(world: &'world mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    pub unsafe fn world(&self) -> &'world World {
        unsafe { &*self.world }
    }

    pub unsafe fn world_mut(&self) -> &'world mut World {
        unsafe { &mut *self.world }
    }
}
