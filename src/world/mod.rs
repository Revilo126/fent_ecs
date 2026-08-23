//! Worlds are the object in charge of storing entities and components.

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{
    component::{
        Component,
        resource::{Resource, Resources},
        storage::{ComponentStorage, ComponentStorages},
    },
    entity::{Entities, Entity},
};

/// The World Struct
pub struct World {
    /// To be able to identify a [`World`] within mass storages
    pub(crate) id: WorldId,
    pub(crate) entities: Entities,
    pub(crate) components: ComponentStorages,
    pub(crate) resources: Resources,
}

impl Default for World {
    /// Returns a newly-created [`World`]
    fn default() -> Self {
        World {
            id: WorldId::default(),
            entities: Entities::default(),
            components: ComponentStorages::default(),
            resources: Resources::default(),
        }
    }
}

impl World {
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

    /// Retrieves the world's [`ComponentStorages`]
    #[inline]
    pub fn components(&self) -> &ComponentStorages {
        &self.components
    }

    /// Retrieves a mutable borrow of this [`World`]'s [`ComponentStorages`]
    ///
    /// # Safety
    ///
    /// The caller must ensure no other mutable refrences to [`ComponentStorages`] exist.
    #[inline]
    pub unsafe fn components_mut(&mut self) -> &mut ComponentStorages {
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

    /// Retrieve the amount of [`Entities`] in the world.
    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.entity_allocated()
    }

    /// Spawn a new [`Entity`]
    #[inline]
    pub fn spawn(&mut self) -> Entity {
        self.entities.alloc()
    }

    /// Despawn the provided [`Entity`]
    #[inline]
    pub fn despawn(&mut self, e: Entity) {
        self.entities.free(e);
    }

    /// Registers a [`Component`] to the [`ComponentStorages`]
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
        self.components
            .get_mut_or_insert::<T::Storage>()
            .unwrap()
            .insert(entity, component);
    }

    /// Returns a [`Component`] for an [`Entity`]
    #[inline]
    pub fn get_component<T>(&self, entity: Entity) -> Option<&T>
    where
        T: Component,
    {
        self.components.get::<T::Storage>().unwrap().get(entity)
    }

    /// Returns a mutable [`Component`] for an [`Entity`]
    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: Entity) -> Option<&mut T>
    where
        T: Component,
    {
        self.components
            .get_mut::<T::Storage>()
            .unwrap()
            .get_mut(entity)
    }

    /// Removes a [`Component`] for an [`Entity`],
    ///
    /// This only sets to a None
    #[inline]
    pub fn remove_component<T>(&mut self, entity: Entity)
    where
        T: Component,
    {
        self.components
            .get_mut::<T::Storage>()
            .unwrap()
            .remove(entity);
    }

    /// Inserts a [`Resource`] into the current [`World`]
    #[inline]
    pub fn insert_resource<R>(&mut self, resource: R)
    where
        R: Resource,
    {
        self.resources.insert(resource);
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
}

/// A unique identification given to every [`World`]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WorldId {
    pub(crate) inner: usize,
}

/// Tracker of next [`WorldId`]'s avaliable
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
    pub fn inner_value(&self) -> usize {
        self.inner
    }
}
