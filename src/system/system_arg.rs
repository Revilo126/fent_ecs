//! System Args allow systems to only access what they need.

use crate::{
    system::access::{Access, AccessError},
    world::unsafe_world_cell::UnsafeWorldCell,
};

pub trait SystemArg {
    type Item<'world>;

    fn access(access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    ///
    /// The caller must ensure access inside of a system is correct.
    /// Otherwise undefined behaviour can occur.
    unsafe fn fetch<'world>(world: UnsafeWorldCell<'world>) -> Self::Item<'world>;
}
