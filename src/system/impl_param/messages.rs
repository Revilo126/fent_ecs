//! Messages allow simple communication between a World and it's Systems

use crate::{
    message::{Message, Messages},
    system::{
        access::{Access, AccessError},
        system_param::SystemParam,
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub struct MessageWriter<'w, M: Message> {
    messages: &'w mut Messages<M>,
}

impl<M: Message> MessageWriter<'_, M> {
    pub fn write(&mut self, message: M) {
        self.messages.write(message);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

impl<M: Message> SystemParam for MessageWriter<'_, M> {
    type State = ();
    type Item<'w> = MessageWriter<'w, M>;

    fn init_state(world: &mut World) -> Self::State {
        if world.get_resource::<Messages<M>>().is_none() {
            let mes: Messages<M> = Messages { queue: Vec::new() };
            world.insert_resource(mes);
        }
    }

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.write::<Messages<M>>()
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> Self::Item<'w> {
        let messages = unsafe { world.world_mut().get_mut_resource::<Messages<M>>() }
            .expect("Messages should have already existed");

        MessageWriter { messages }
    }
}

pub struct MessageReader<'w, M: Message> {
    messages: &'w Messages<M>,
    cursor: &'w mut usize,
}

impl<'w, M: Message> MessageReader<'w, M> {
    pub fn read(&mut self) -> impl Iterator<Item = &'w M> + '_ {
        let start = *self.cursor;
        *self.cursor = self.messages.queue.len();

        self.messages.iter().skip(start)
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl<M: Message> SystemParam for MessageReader<'_, M> {
    type State = usize;
    type Item<'w> = MessageReader<'w, M>;

    fn init_state(world: &mut World) -> Self::State {
        if world.get_resource::<Messages<M>>().is_none() {
            let mes: Messages<M> = Messages { queue: Vec::new() };

            world.insert_resource(mes);
        }

        0
    }

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        access.read::<Messages<M>>()
    }

    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> Self::Item<'w> {
        let messages = unsafe { world.world().get_resource::<Messages<M>>() }
            .expect("Messages should have already existed");

        MessageReader {
            messages,
            cursor: state,
        }
    }
}
