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

impl QueryData for Entity {
    type Item<'w> = Entity;

    fn access(_access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn fetch<'w>(_world: UnsafeWorldCell<'w>, entity: Entity) -> Option<Entity> {
        Some(entity)
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

pub trait QueryFilter {
    fn access(access: &mut Access) -> Result<(), AccessError>;

    /// # Safety
    /// Access must have been validated.
    unsafe fn matches(world: UnsafeWorldCell<'_>, entity: Entity) -> bool;
}

pub struct With<T>(PhantomData<T>);

impl<T: Component> QueryFilter for With<T> {
    fn access(_access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn matches(world: UnsafeWorldCell<'_>, entity: Entity) -> bool {
        unsafe { world.world().get_component::<T>(entity).is_some() }
    }
}

pub struct Without<T>(PhantomData<T>);

impl<T: Component> QueryFilter for Without<T> {
    fn access(_access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn matches(world: UnsafeWorldCell<'_>, entity: Entity) -> bool {
        unsafe { world.world().get_component::<T>(entity).is_none() }
    }
}

impl QueryFilter for () {
    fn access(_access: &mut Access) -> Result<(), AccessError> {
        Ok(())
    }

    unsafe fn matches(_world: UnsafeWorldCell<'_>, _entity: Entity) -> bool {
        true
    }
}

macro_rules! impl_query_filter_tuple {
    ($($F:ident),*) => {
        impl<$($F: QueryFilter),*> QueryFilter for ($($F,)*) {
            fn access(access: &mut Access) -> Result<(), AccessError> {
                $($F::access(access)?;)*
                Ok(())
            }

            unsafe fn matches(world: UnsafeWorldCell<'_>, entity: Entity) -> bool {
                true $(&& unsafe { $F::matches(world, entity) })*
            }
        }
    };
}
impl_query_filter_tuple!(F0);
impl_query_filter_tuple!(F0, F1);
impl_query_filter_tuple!(F0, F1, F2);

pub struct Query<'w, D: QueryData, F: QueryFilter = ()> {
    world: UnsafeWorldCell<'w>,
    marker: PhantomData<fn() -> (D, F)>,
}

impl<'w, D: QueryData, F: QueryFilter> Query<'w, D, F> {
    pub fn iter(&mut self) -> impl Iterator<Item = D::Item<'_>> {
        let world = self.world;
        let entities: Vec<Entity> = unsafe { world.world().entities().iter().collect() };

        entities
            .into_iter()
            .filter(move |&entity| unsafe { F::matches(world, entity) })
            .filter_map(move |entity| unsafe { D::fetch(world, entity) })
    }

    pub fn get(&mut self, entity: Entity) -> Option<D::Item<'_>> {
        if !unsafe { F::matches(self.world, entity) } {
            return None;
        }
        unsafe { D::fetch(self.world, entity) }
    }
}

impl<D: QueryData + 'static, F: QueryFilter + 'static> SystemParam for Query<'_, D, F> {
    type State = ();
    type Item<'w> = Query<'w, D, F>;

    fn init_state(_world: &mut World) -> Self::State {}

    fn access(_state: &Self::State, access: &mut Access) -> Result<(), AccessError> {
        D::access(access)?;
        F::access(access)?;
        Ok(())
    }

    unsafe fn get_param<'w>(
        _state: &'w mut Self::State,
        world: UnsafeWorldCell<'w>,
    ) -> Query<'w, D, F> {
        Query {
            world,
            marker: PhantomData,
        }
    }
}
