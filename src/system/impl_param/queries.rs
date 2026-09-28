//! Queries allow accessing components, this is by only acquiring the ones needed.

use std::marker::PhantomData;

use crate::{
    component::Component,
    entity::Entity,
    system::{
        access::{Access, AccessError},
        system_param::SystemParam,
    },
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

pub trait QueryData {
    type Item<'w>;

    fn access(access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    /// Access must have been validated.
    unsafe fn fetch<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Option<Self::Item<'w>>;
}

impl<T: Component> QueryData for &T {
    type Item<'w> = &'w T;

    fn access(access: &mut Access) -> Result<(), AccessError> {
        access.read::<T>()
    }

    unsafe fn fetch<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Option<&'w T> {
        unsafe { world.world().get_component::<T>(entity) }
    }
}

impl<T: Component> QueryData for &mut T {
    type Item<'w> = &'w mut T;

    fn access(access: &mut Access) -> Result<(), AccessError> {
        access.write::<T>()
    }

    unsafe fn fetch<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Option<&'w mut T> {
        unsafe { world.world_mut().get_mut_component::<T>(entity) }
    }
}

macro_rules! impl_query_data_tuple {
    ($($D:ident),*) => {
        impl<$($D: QueryData),*> QueryData for ($($D,)*) {
            type Item<'w> = ($($D::Item<'w>,)*);

            fn access(access: &mut Access) -> Result<(), AccessError> {
                $($D::access(access)?;)*
                Ok(())
            }

            unsafe fn fetch<'w>(world: UnsafeWorldCell<'w>, entity: Entity) -> Option<Self::Item<'w>> {
                Some(($(unsafe { $D::fetch(world, entity)? },)*))
            }
        }
    };
}
impl_query_data_tuple!(D0);
impl_query_data_tuple!(D0, D1);
impl_query_data_tuple!(D0, D1, D2);
impl_query_data_tuple!(D0, D1, D2, D3);
impl_query_data_tuple!(D0, D1, D2, D3, D4);
impl_query_data_tuple!(D0, D1, D2, D3, D4, D5);

pub struct Query<'w, D: QueryData> {
    world: UnsafeWorldCell<'w>,
    marker: PhantomData<fn() -> D>,
}

impl<'w, D: QueryData> Query<'w, D> {
    pub fn iter(&mut self) -> impl Iterator<Item = D::Item<'_>> {
        let world = self.world;
        // SAFETY: access is already verified.
        let entities = unsafe { world.world().entities() };
        entities
            .iter()
            .filter_map(move |entity| unsafe { D::fetch(world, entity) })
    }

    pub fn get(&mut self, entity: Entity) -> Option<D::Item<'_>> {
        unsafe { D::fetch(self.world, entity) }
    }
}

impl<D: QueryData + 'static> SystemParam for Query<'_, D> {
    type State = ();
    type Item<'w> = Query<'w, D>;

    fn init_state(_world: &mut World) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        D::access(access)
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> Query<'w, D> {
        Query {
            world,
            marker: PhantomData,
        }
    }
}
