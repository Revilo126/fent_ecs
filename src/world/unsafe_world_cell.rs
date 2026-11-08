//! [`UnsafeWorldCell`]'s provide a way to access a [`World`] in a sketchy way.

use std::marker::PhantomData;

use crate::world::World;

#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'world> {
    world: *mut World,
    marker: PhantomData<&'world mut World>,
}

impl<'world> UnsafeWorldCell<'world> {
    /// # Safety
    ///
    /// This method does not do anything 'unsafe' by itself, but ensure all uses of the
    /// [`UnsafeWorldCell`] are verified.
    #[inline]
    pub unsafe fn from_world(world: &'world mut World) -> Self {
        Self {
            world,
            marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// Ensure there is no Read-Write conficts that will exist once calling this,
    /// If not done so unexplainable behaviour may occur.
    #[inline]
    pub unsafe fn world(&self) -> &'world World {
        unsafe { &*self.world }
    }

    /// # Safety
    ///
    /// Ensure the use of the mutable [`World`] is correct within the assessed capabilities.
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
