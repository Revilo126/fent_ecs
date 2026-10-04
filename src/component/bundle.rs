use crate::{component::Component, entity::Entity, world::World};

pub trait Bundle: Send + Sync + 'static {
    fn insert_into(self, entity: Entity, world: &mut World);
}

impl<T: Component> Bundle for T {
    fn insert_into(self, entity: Entity, world: &mut World) {
        world.insert_component(entity, self);
    }
}

macro_rules! impl_bundle_tuple {
    ($($B:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($B: Bundle),*> Bundle for ($($B,)*) {
            fn insert_into(self, entity: Entity, world: &mut World) {
                let ($($B,)*) = self;
                $($B::insert_into($B, entity, world);)*
            }
        }
    };
}
impl_bundle_tuple!(B0);
impl_bundle_tuple!(B0, B1);
impl_bundle_tuple!(B0, B1, B2);
impl_bundle_tuple!(B0, B1, B2, B3);
impl_bundle_tuple!(B0, B1, B2, B3, B4);
impl_bundle_tuple!(B0, B1, B2, B3, B4, B5);
