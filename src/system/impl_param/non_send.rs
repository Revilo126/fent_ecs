//! Allows the marking and use of non thread safe objects

use crate::{
    system::{
        access::{Access, AccessError},
        meta::SystemMeta,
        system_param::SystemParam,
    },
    world::World,
};

pub struct NonSendMarker;

impl SystemParam for NonSendMarker {
    type State = ();
    type Item<'w> = NonSendMarker;

    fn init_state(_world: &mut World, meta: &mut SystemMeta) -> Self::State {
        meta.set_main_thread_only();
    }

    fn access(_state: &Self::State, _access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>,
    ) -> Self::Item<'w> {
        NonSendMarker
    }
}
