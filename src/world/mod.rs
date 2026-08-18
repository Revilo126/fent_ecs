//! Worlds are the object in charge of storing entities and components.

use std::sync::atomic::{AtomicUsize, Ordering};

use crate::entity::{Entities, Entity};

// The World Struct
pub struct World {
    // To be able to identify worlds within mass storages
    id: WorldId,
    pub(crate) entities: Entities,
}

impl Default for World {
    /// Returns a newly-created [`World`]
    fn default() -> Self {
        World {
            id: WorldId::new()
                .expect("How in the actual fuck did you overflow the world counter?!"),
            entities: Entities::default(),
        }
    }
}

impl World {
    /// Returns this [`World`]'s id
    #[inline]
    pub fn id(&self) -> WorldId {
        self.id
    }

    /// Retrieve the world's [`Entities`]
    #[inline]
    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    /// Retrieve the amount of [`Entities`] in the world.
    #[inline]
    pub fn entity_count(&self) -> usize {
        self.entities.entity_allocated()
    }

    /// Spawn a new [`Entity`]
    pub fn spawn(&mut self) -> Entity {
        self.entities.alloc()
    }

    /// Frees/Kills the provided [`Entity`]
    pub fn free(&mut self, e: Entity) {
        self.entities.free(e);
    }
}

/// A unique identification given to every [`World`]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WorldId(usize);

/// Tracker of next [`WorldId`]'s avaliable
static NEXT_WORLD_ID: AtomicUsize = AtomicUsize::new(0);

impl WorldId {
    /// Return's the next avaliable [`WorldId`]
    pub fn new() -> Option<Self> {
        NEXT_WORLD_ID
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .map(WorldId)
            .ok()
    }
}
