#[cfg(test)]
mod test {
    use fent_derive::{Component, Resource};
    use fent_ecs::{
        component::identification::{component_id, resource_id},
        world::World,
    };

    #[test]
    fn component_ids() {
        struct Obj1;
        struct Obj2;

        let aid = component_id::<Obj1>();
        let bid = component_id::<Obj2>();

        let i = component_id::<Obj1>();
        assert_eq!(i, aid);
        let j = component_id::<Obj2>();
        assert_eq!(j, bid);

        let i = component_id::<Obj1>();
        assert_eq!(i, aid);
    }

    #[test]
    fn resource_ids() {
        struct Obj1;
        struct Obj2;

        let aid = resource_id::<Obj1>();
        let bid = resource_id::<Obj2>();

        let i = resource_id::<Obj1>();
        assert_eq!(i, aid);
        let j = resource_id::<Obj2>();
        assert_eq!(j, bid);

        let i = resource_id::<Obj1>();
        assert_eq!(i, aid);
    }

    #[test]
    fn component_storage() {
        #[derive(Component)]
        pub struct Component1 {
            value: u32,
        }

        let mut world = World::default();

        world.register_component::<Component1>();

        let e = world.spawn();

        let comp = Component1 { value: 5 };

        world.insert_component(e, comp);

        let comp = world.get_component::<Component1>(e);

        if let Some(comp) = comp {
            assert_eq!(comp.value, 5);
        } else {
            panic!("Failed to retrieve comp!");
        }
    }

    #[test]
    fn resource_storage() {
        #[derive(Resource)]
        pub struct ResType {
            a: f32,
            b: usize,
        }

        let mut world = World::default();

        let resource = ResType { a: 6.4, b: 6345 };

        world.insert_resource(resource);

        if let Some(resource) = world.get_resource::<ResType>() {
            assert_eq!(resource.a, 6.4);
            assert_eq!(resource.b, 6345);
        } else {
            panic!("Failed to retrieve Resource!");
        }
    }

    // #[test]
    // #[should_panic(expected = "WriteWriteConflict")]
    // fn access_system_error() {}

    #[test]
    fn system_mutability() {}

    #[test]
    fn component_storage_mutability() {
        #[derive(Component)]
        struct Comp {
            a: f32,
        }

        let mut world = World::default();
        world.register_component::<Comp>();

        let e = world.spawn();

        world.insert_component(e, Comp { a: 0.5 });

        let Some(comp) = world.get_mut_component::<Comp>(e) else {
            panic!("Failed to retrieve Component!");
        };

        comp.a = 0.7;

        let _ = comp;

        let Some(comp) = world.get_component::<Comp>(e) else {
            panic!("Failed to retrieve Component!");
        };

        assert_eq!(comp.a, 0.7);
    }
}
