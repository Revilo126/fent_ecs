# Fent ECS

## What is it?

Fent ECS is purpose built for my own needs, I will support any one else who uses it though.

## Why is it?

Why not?

# Example

```rust

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
```


# Acknowledgements

Many characteristics of this ECS library are inspired from other notorious ones,
This mainly includes bevy_ecs and hecs.

