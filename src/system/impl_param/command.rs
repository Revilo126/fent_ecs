//! Commands are a way for systems to do actions like spawning and despawning [`Entity`]'s.

use crate::{
    component::{Component, bundle::Bundle, resource::Resource},
    entity::Entity,
    event::{Event, handlers::IntoEventHandler},
    system::{
        access::{Access, AccessError},
        meta::SystemMeta,
        system_param::SystemParam,
    },
    world::{World, change_detection::Tick, unsafe_world_cell::UnsafeWorldCell},
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
    #[inline]
    pub fn spawn(&mut self, bundle: impl Bundle) {
        self.queue.push(move |world: &mut World| {
            let entity = world.spawn();
            bundle.insert_into(entity, world);
        });
    }

    #[inline]
    pub fn despawn(&mut self, entity: Entity) {
        self.queue.push(move |world: &mut World| {
            world.despawn(entity);
        });
    }

    #[inline]
    pub fn insert_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.queue.push(move |world: &mut World| {
            world.insert_component(entity, component);
        });
    }

    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        self.queue.push(move |world: &mut World| {
            world.remove_component::<T>(entity);
        });
    }

    #[inline]
    pub fn insert_resource<R: Resource + Send + Sync>(&mut self, resource: R) {
        self.queue.push(move |world: &mut World| {
            world.insert_resource(resource);
        });
    }

    #[inline]
    pub fn remove_resource<R: Resource>(&mut self) {
        self.queue.push(move |world: &mut World| {
            world.remove_resource::<R>();
        })
    }

    #[inline]
    pub fn on<E, M>(&mut self, handler: impl IntoEventHandler<E, M> + Send + Sync + 'static)
    where
        E: Event,
        M: 'static,
    {
        self.queue.push(move |world: &mut World| {
            world.on(handler);
        });
    }

    #[inline]
    pub fn emit<E: Event>(&mut self, event: E) {
        self.queue.push(|world: &mut World| {
            world.emit(event);
        });
    }
}

impl SystemParam for Commands<'_> {
    type State = CommandQueue;
    type Item<'w> = Commands<'w>;

    fn init_state(_world: &mut World, _meta: &mut SystemMeta) -> Self::State {
        CommandQueue::default()
    }

    fn access(_state: &Self::State, _access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn get_param<'w>(
        state: &'w mut Self::State,
        _meta: &SystemMeta,
        _world: UnsafeWorldCell<'w>,
        _this_run: Tick,
    ) -> Commands<'w> {
        Commands { queue: state }
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        state.apply(world);
    }
}
