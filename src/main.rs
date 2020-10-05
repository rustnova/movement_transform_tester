use std::{fmt::Debug, path::Path, path::PathBuf};

use bevy::math::*;
use bevy::prelude::*;
use motion_test::MotionTest;
use movement_transform_tester::*;

mod movement_transform_tester;
mod equations_of_motion;
mod motion_test;

// create your own State struct to include things you care about in your testing
#[derive(Default, Debug, Copy, Clone)]
struct State {
    transform: Transform,
    velocity: Vec3,
    accel: f32,
    max_rotation: f32,
}
// impl your own evaluation function, comparing your original state to your final state
impl TestState for State {
    fn eval(&mut self, original_state: &Self) -> Result {
        Result {
            status: Status::Complete,
            ..Default::default()            
        }
    }
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        // initialize the printing resource with the correct path
        .add_resource(PrintResource {
            path: "E:/Rust/Projects/movement_transform_tester/printout.txt".to_string(),
            printed: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(MotionTest)
        .add_startup_system(setup.system())
        .add_system(evaluate.system())
        .run();
}

fn setup(mut commands: Commands) {
    let trans = Transform::default();
    let x = TestMarker {
        result: Result::default(),
        original_state: Some(State::default()),
        new_state: Some(State::default()),
    };
    // TODO add to ECS and get randomization happening
    // commands.spawn(SpriteComponents::default()).with(x);
}

fn evaluate(
    mut printer: ResMut<PrintResource>,
    time: Res<Time>,
    mut query: Query<(&mut TestMarker<State>, &Transform)>,
) {
    println!("time since startup: {}", time.seconds_since_startup);
    // counter to see how many ships are NOT complete
    let mut pending_ships = 0;
    // cycle through all marked ships
    for (mut marker, transform) in &mut query.iter() {
        // check if we've reached our timer
        if time.seconds_since_startup > 10. {
            // update "new state" to show current values
            marker.new_state = Some(State{ transform: *transform, ..Default::default()});
            // EVALUATE
            marker.evaluate(&mut printer);
        }
        // update counter if there are ships not yet completed
        if marker.result.status == Status::Pending {
            pending_ships += 1;
        }
    }
    // print only if you haven't printed before and if you have NO pending ships
    if printer.printed == false && pending_ships == 0 {
        println!("time to print!");
        printer.print();
    }
}
