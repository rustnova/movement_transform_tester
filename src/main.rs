use std::{fmt::Debug, path::Path, path::PathBuf};

use bevy::math::*;
use bevy::prelude::*;
use equations_of_motion::Momentum;
use motion_test::MotionTest;
use movement_transform_tester::*;

mod equations_of_motion;
mod motion_test;
mod movement_transform_tester;
mod newtonian_physics;

#[derive(Default, Debug, Copy, Clone)]
struct State {
    transform: Transform,
    destination: Vec3,
    motion_values: Momentum,
    tick_counter: u32,
    tick_goal: u32,
}
impl TestState for State {
    // so this evaluate is pretty fucking useless.... i guess we could provide a some tests but we would need to formalize momentum to that effect
    fn eval(&mut self, original_state: &Self) -> Result {
        Result {
            status: Status::Complete,
            ..Default::default()
        }
    }
}
impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:?} \n{:?} \n{:?}", self.motion_values, self.transform, self.destination)
    }
}

fn main() {
    App::build()
        .add_resource(PrintResource {
            path: "test_results/".to_string(),
            printed: false,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(MotionTest)
        .run();
}
