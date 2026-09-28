//! Components are objects stored in conjunction with entities,
//! providing data on said entities.

use crate::{component::storage::ComponentStorage, entity::Entity, world::World};

pub mod identification;
pub mod resource;
pub mod storage;

// Component trait
pub trait Component: Send + Sync + Sized + 'static {
    type Storage: ComponentStorage<Component = Self>;

    fn on_add(entity: Entity, world: &mut World);

    fn on_remove(entity: Entity, world: &mut World);
}
