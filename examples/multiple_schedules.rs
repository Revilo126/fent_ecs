use std::io::{self, Write};

use fent_derive::Resource;
use fent_ecs::{
    schedule::Schedule,
    system::impl_param::{Res, ResMut},
    world::World,
};

const PASSWORD: &str = "Hello12";

/// This example demonstrates that the use of 2 or more shedules can be useful.
///
/// Multiple schedules can be used to make sure certain tasks happen at the right time, here it's
/// used so the systems happen in this order: print_text, get_input, check_guess.
///
/// # Note
///
/// Yes the test is stupid.
fn main() {
    let mut world = World::default();

    world.insert_resource(Guess("".into()));
    world.insert_resource(PasswordCorrect(false));

    let mut print_schedule = Schedule::default();
    print_schedule.insert_system(print_text);

    let mut input_schedule = Schedule::default();
    input_schedule.insert_system(get_input);

    let mut check_schedule = Schedule::default();
    check_schedule.insert_system(check_guess);

    loop {
        print_schedule.run(&mut world);
        input_schedule.run(&mut world);
        check_schedule.run(&mut world);

        let correct = world
            .get_resource::<PasswordCorrect>()
            .map(|value| value.0)
            .unwrap_or(false);

        if correct {
            break;
        }

        println!("Incorrect password.");
    }

    println!("Correct password!");
}

fn print_text() {
    print!("What is the password? ");
    io::stdout().flush().unwrap();
}

#[derive(Resource)]
struct Guess(String);

#[derive(Resource)]
struct PasswordCorrect(bool);

fn get_input(mut guess: ResMut<Guess>) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let password = input.trim().to_owned();

    guess.0 = password;
}

fn check_guess(guess: Res<Guess>, mut correct: ResMut<PasswordCorrect>) {
    let is_correct = guess.0 == PASSWORD;

    correct.0 = is_correct;
}
