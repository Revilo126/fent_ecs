//! Components are objects stored in conjunction with entities,
//! providing data on said entities.

use crate::component::storage::ComponentStorage;

pub mod identification;
pub mod storage;

// Component trait
pub trait Component: Send + Sync + Sized + 'static {
    type Storage: ComponentStorage<Component = Self>;

    fn on_add() -> Option<ComponentAction> {
        None
    }

    fn on_insert() -> Option<ComponentAction> {
        None
    }

    fn on_remove() -> Option<ComponentAction> {
        None
    }

    fn on_despawn() -> Option<ComponentAction> {
        None
    }
}

// Not working now
type ComponentAction = fn();
