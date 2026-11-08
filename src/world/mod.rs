//! Worlds are the object in charge of storing entities and components.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use crate::{
    component::{
        Component,
        resource::{Resource, Resources},
        storage::{ComponentStorage, Components},
    },
    entity::{Entities, Entity},
    event::{
        Event, Events,
        handlers::{ErasedHandler, IntoEventHandler, TypedHandler},
        identification::event_id,
    },
    system::access::Access,
    world::change_detection::Tick,
};

pub mod change_detection;
pub mod unsafe_world_cell;

/// The World struct containing all [`Entities`], [`Components`], [`Resources`] and other owned
/// objects.
#[derive(Default)]
pub struct World {
    pub(crate) id: WorldId,
    pub(crate) entities: Entities,
    pub(crate) components: Components,
    pub(crate) resources: Resources,
    pub(crate) events: Events,
    pub(crate) change_tick: AtomicU64,
}

impl World {
    /// Returns a newly-created [`World`]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns this [`World`]'s [`WorldId`]
    #[inline]
    pub fn id(&self) -> WorldId {
        self.id
    }

    /// Retrieve the [`World]'s [`Entities`]
    #[inline]
    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    /// Retrieves a mutable borrow of this [`World`]'s [`Entities`]
    ///
    /// # Safety
    ///
    /// The caller must ensure no other mutable refrences to [`Entities`] exist.
    #[inline]
    pub unsafe fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    /// Retrieves the world's [`Components`]
    #[inline]
    pub fn components(&self) -> &Components {
        &self.components
    }

    /// Retrieves a mutable borrow of this [`World`]'s [`Components`]
    ///
    /// # Safety
    ///
    /// The caller must ensure no other mutable refrences to [`Components`] exist.
    #[inline]
    pub unsafe fn components_mut(&mut self) -> &mut Components {
        &mut self.components
    }

    /// Restrieves the world's [`Resources`]
    #[inline]
    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    /// Retrieves a mutable borrow of this [`World`]'s [`Resources`]
    ///
    /// # Safety
    ///
    /// The caller must ensure no other mutable refrences to [`Resources`] exist.
    #[inline]
    pub unsafe fn resources_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }

    /// Retrieves the world's [`Events`]
    #[inline]
    pub fn events(&self) -> &Events {
        &self.events
    }

    /// Retrieves a mutable borrow of this [`World`]'s [`Events`]
    ///
    /// # Safety
    ///
    /// The caller must ensure no other mutable refrence to [`Events`] exist.
    #[inline]
    pub unsafe fn events_mut(&mut self) -> &mut Events {
        &mut self.events
    }

    /// Retrieve the amount of [`Entities`] in the world.
    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.entity_allocated()
    }

    /// Spawn a new [`Entity`]
    #[inline]
    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }

    /// Despawn the provided [`Entity`]
    #[inline]
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.entities.despawn(entity) {
            return false;
        }

        let index = entity.index();

        for storage in self.components.storages.iter_mut().flatten() {
            storage.remove_untyped(index);
        }

        self.components.clear_all_ticks(index);

        true
    }

    /// Registers a [`Component`] to the [`Components`]
    #[inline]
    pub fn register_component<T: Component>(&mut self) {
        self.components.insert::<T::Storage>();
    }

    /// Inserts a [`Component`] for an [`Entity`]
    #[inline]
    pub fn insert_component<T>(&mut self, entity: Entity, component: T)
    where
        T: Component,
    {
        assert!(
            self.entities.is_alive(entity),
            "Entity {:?} is not alive!",
            entity
        );

        let tick = self.increment_change_tick();
        let index = entity.index();

        self.components
            .get_mut_or_insert::<T::Storage>()
            .unwrap()
            .insert(index, component);

        self.components.stamp_insert::<T>(index, tick);

        T::on_add(entity, self);
    }

    /// Returns a [`Component`] for an [`Entity`]
    #[inline]
    pub fn get_component<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component,
    {
        if !self.entities.is_alive(entity) {
            return None;
        }

        if let Some(storage) = self.components.get::<T::Storage>() {
            return storage.get(entity.index());
        }

        None
    }

    /// Returns a mutable [`Component`] for an [`Entity`]
    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        if !self.entities.is_alive(entity) {
            return None;
        }

        self.components
            .get_mut_or_insert::<T::Storage>()
            .unwrap()
            .get_mut(entity.index())
    }

    /// Removes a [`Component`] for an [`Entity`],
    ///
    /// This only sets to a None
    #[inline]
    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: Component,
    {
        if !self.entities.is_alive(entity) {
            return;
        }

        let index = entity.index();

        self.components
            .get_mut::<T::Storage>()
            .unwrap()
            .remove(index);

        self.components.clear_ticks::<T>(index);

        T::on_remove(entity, self);
    }

    /// Inserts a [`Resource`] into the current [`World`]
    #[inline]
    pub fn insert_resource<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        let tick = self.increment_change_tick();
        self.resources.insert(resource, tick);
    }

    /// Returns a [`Resource`]
    #[inline]
    pub fn get_resource<R>(&self) -> Option<&R>
    where
        R: Resource,
    {
        self.resources.get::<R>()
    }

    /// Returns a mutable [`Resource`]
    #[inline]
    pub fn get_mut_resource<R>(&mut self) -> Option<&mut R>
    where
        R: Resource,
    {
        self.resources.get_mut::<R>()
    }

    /// Removes a [`Resource`] from this world
    ///
    /// This only sets to a None
    #[inline]
    pub fn remove_resource<R>(&mut self)
    where
        R: Resource,
    {
        self.resources.remove::<R>();
    }

    /// Sets an [`crate::event::handlers::EventHandler`] to run on an [`Event`]
    pub fn on<E: Event, M: 'static>(&mut self, handler: impl IntoEventHandler<E, M>) {
        let mut typed = TypedHandler::<E> {
            handler: Box::new(handler.into_event_handler()),
        };
        typed.initialize(self);
        typed.access(&mut Access::default()).unwrap();
        self.events.register(event_id::<E>(), Box::new(typed));
    }

    /// Emit an [`Event`], running its [`crate::event::handlers::EventHandler`]'s
    pub fn emit<E: Event>(&mut self, event: E) {
        let id = event_id::<E>();
        let mut handlers = self.events.take(id);

        for handler in &mut handlers {
            handler.call(self, &event);
            handler.apply(self);
        }

        self.events.restore(id, handlers);
    }

    /// Returns the current change [`Tick`]
    pub fn change_tick(&self) -> Tick {
        Tick(self.change_tick.load(Ordering::Acquire))
    }

    pub(crate) fn increment_change_tick(&mut self) -> Tick {
        Tick(self.change_tick.fetch_add(1, Ordering::AcqRel) + 1)
    }
}

/// A unique identification given to every [`World`]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WorldId {
    pub(crate) inner: usize,
}

// Tracker of next [`WorldId`]'s avaliable
static NEXT_WORLD_ID: AtomicUsize = AtomicUsize::new(0);

impl Default for WorldId {
    fn default() -> Self {
        Self {
            inner: NEXT_WORLD_ID.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl WorldId {
    /// Returns the [`WorldId`]'s inner integer value
    #[inline]
    pub fn inner_value(&self) -> usize {
        self.inner
    }
}
