use std::{fmt::Debug, path::Path, path::PathBuf};

use bevy::math::*;
use bevy::prelude::*;
use equations_of_motion::Momentum;
use motion_test::MotionTest;
use movement_transform_tester::*;

mod equations_of_motion;
mod motion_test;
mod movement_transform_tester;

#[derive(Default, Debug, Copy, Clone)]
struct State {
    transform: Transform,
    motion_values: Momentum,
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
        write!(f, "{:?} {:?}", self.motion_values, self.transform)
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
