//! Commands are a way for systems to do actions like spawning and despawning [`Entity`]'s.

use crate::{
    component::{Component, resource::Resource},
    entity::Entity,
    system::{
        access::{Access, AccessError},
        system_param::SystemParam,
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait Command: Send + Sync + 'static {
    fn apply(self: Box<Self>, world: &mut World);
}

impl<F: FnOnce(&mut World) + Send + Sync + 'static> Command for F {
    fn apply(self: Box<Self>, world: &mut World) {
        (*self)(world)
    }
}

#[derive(Default)]
pub struct CommandQueue {
    commands: Vec<Box<dyn Command>>,
}
impl CommandQueue {
    pub fn push(&mut self, command: impl Command) {
        self.commands.push(Box::new(command));
    }

    pub fn apply(&mut self, world: &mut World) {
        for command in self.commands.drain(..) {
            command.apply(world);
        }
    }
}

pub struct Commands<'w> {
    queue: &'w mut CommandQueue,
}

impl<'w> Commands<'w> {
    pub fn spawn(
        &mut self,
        spawn_actions: impl FnOnce(Entity, &mut World) + Send + Sync + 'static,
    ) {
        self.queue.push(move |world: &mut World| {
            let entity = world.spawn();
            spawn_actions(entity, world);
        });
    }

    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push(move |world: &mut World| {
            world.despawn(entity);
        });
    }

    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.queue.push(move |world: &mut World| {
            world.insert_component(entity, component);
        });
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        self.queue.push(move |world: &mut World| {
            world.remove_component::<T>(entity);
        });
    }

    pub fn insert_resource<R: Resource + Send + Sync>(&mut self, resource: R) {
        self.queue.push(move |world: &mut World| {
            world.insert_resource(resource);
        });
    }

    pub fn remove_resource<R: Resource>(&mut self) {
        self.queue.push(move |world: &mut World| {
            world.remove_resource::<R>();
        })
    }
}

impl SystemParam for Commands<'_> {
    type State = CommandQueue;
    type Item<'w> = Commands<'w>;

    fn init_state(_world: &mut World) -> Self::State {
        CommandQueue::default()
    }

    fn access(_state: &Self::State, _access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        _world: UnsafeWorldCell<'w>,
    ) -> Commands<'w> {
        Commands { queue: state }
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        state.apply(world);
    }
}
