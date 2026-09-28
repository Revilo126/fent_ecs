//! System parameters are the tuple in which is given to systems when running.

use crate::{
    system::access::{Access, AccessError},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait SystemParam {
    type State: Send + Sync + 'static;
    type Item<'w>: SystemParam<State = Self::State>;

    fn init_state(world: &mut World) -> Self::State;
    fn access(state: &Self::State, access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    /// Access must have been validated, and no conflicting access may be live.
    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> Self::Item<'w>;
}

pub type ParamItem<'w, P> = <P as SystemParam>::Item<'w>;
