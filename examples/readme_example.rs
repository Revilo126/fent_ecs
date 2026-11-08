//! This example is the one shown in the README.md
//!
//! Can be run using the following command:
//! - cargo run -p fent_ecs --example readme_example

use fent_ecs::prelude::*;

fn main() {
    let mut world = World::default();

    #[derive(Resource)]
    struct Origin {
        x: u32,
        y: u32,
    }

    fn reset_origin(mut o: ResMut<Origin>) {
        if o.x != 0 || o.y != 0 {
            o.x = 0;
            o.y = 0;
        }
    }

    world.insert_resource(Origin { x: 1, y: 4 });

    let mut schedule = Schedule::default();
    schedule.add_system(reset_origin);

    schedule.run(&mut world);

    let o = world
        .get_resource::<Origin>()
        .expect("Failed to retrieve \"Origin\" resource");

    assert_eq!(o.x, 0);
    assert_eq!(o.y, 0);
}
