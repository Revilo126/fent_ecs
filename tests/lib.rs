#[cfg(test)]
mod test {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    use fent_derive::{Component, Event, Resource};
    use fent_ecs::{
        component::{
            identification::{component_id, resource_id},
            storage::sparse_set::SparseStorage,
        },
        schedule::Schedule,
        system::IntoSystem,
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
    fn component_ids_are_unique() {
        struct Obj1;
        struct Obj2;

        let obj1_id = component_id::<Obj1>();
        let obj2_id = component_id::<Obj2>();

        assert_eq!(component_id::<Obj1>(), obj1_id);
        assert_eq!(component_id::<Obj2>(), obj2_id);
        assert_ne!(obj1_id, obj2_id);
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
    fn resource_ids_are_unique() {
        struct Obj1;
        struct Obj2;

        let obj1_id = resource_id::<Obj1>();
        let obj2_id = resource_id::<Obj2>();

        assert_eq!(resource_id::<Obj1>(), obj1_id);
        assert_eq!(resource_id::<Obj2>(), obj2_id);
        assert_ne!(obj1_id, obj2_id);
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
    fn component_sparse_set_storage() {
        #[derive(Component)]
        #[storage(SparseStorage)]
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

    // #[test]
    // fn system_mutability() {}

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

    #[test]
    fn event_handler_called() {
        #[derive(Event)]
        struct TestEvent {
            value: usize,
        }

        let mut world = World::default();

        let total = Arc::new(AtomicUsize::new(0));
        let total_for_handler = Arc::clone(&total);

        world.on::<TestEvent>(move |_world, event| {
            total_for_handler.fetch_add(event.value, Ordering::SeqCst);
        });

        world.emit(TestEvent { value: 5 });

        assert_eq!(total.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn readme_example() {
        let mut world = World::default();

        #[derive(Resource)]
        struct Origin {
            x: u32,
            y: u32,
        }

        fn system(world: &mut World) {
            let Some(o) = world.get_mut_resource::<Origin>() else {
                panic!("Failed to retrieve \"Origin\" resource!");
            };

            if o.x != 0 || o.y != 0 {
                o.x = 0;
                o.y = 0;
            }
        }

        world.insert_resource(Origin { x: 1, y: 4 });

        let mut schedule = Schedule::default();
        schedule.insert_system(system.into_system());

        schedule.run(&mut world);

        let Some(o) = world.get_resource::<Origin>() else {
            panic!("Failed to retrieve \"Origin\" resource!");
        };

        assert_eq!(o.x, 0);
        assert_eq!(o.y, 0);
    }
}
