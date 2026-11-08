//! [`UnsafeWorldCell`]'s provide a way to access a [`World`] in a sketchy way.

use std::marker::PhantomData;

use crate::world::World;

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'world> {
    world: *mut World,
    marker: PhantomData<&'world mut World>,
}

impl<'world> UnsafeWorldCell<'world> {
    #[inline]
    pub unsafe fn from_world(world: &'world mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn world(&self) -> &'world World {
        unsafe { &*self.world }
    }

    #[inline]
    pub unsafe fn world_mut(&self) -> &'world mut World {
        unsafe { &mut *self.world }
    }
}

// SAFETY: The scheduler only shares this handle among systems in a stage
// whose declared accesses have been proven non-conflicting. Systems can
// access world data only through APIs that enforce those declarations.
// The world remains alive and is not otherwise accessed until the stage joins.
unsafe impl Send for UnsafeWorldCell<'_> {}
unsafe impl Sync for UnsafeWorldCell<'_> {}
