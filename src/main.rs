use std::{fmt::Debug, path::Path, path::PathBuf};

use bevy::math::*;
use bevy::prelude::*;
use movement_transform_tester::*;

mod movement_transform_tester;

#[derive(Default, Debug, Copy, Clone)]
struct State {
    velocity: Vec3,
    accel: f32,
    max_rotation: f32,
}

fn main() {
        App::build()
        .add_resource(Msaa { samples: 4 })
            .add_resource(PrintResource { path: "X:/Rust Projects/movement_tests".to_string(), ..Default::default()})
        .add_default_plugins()
        .run();

    let trans = Transform::default();
    let x = TestMarker {
        original_trans: trans,
        new_trans: trans,
        eval: Box::new(|original, new, state| Result::default()),
        result: Result::default(),
        state: Some(State::default()),
    };
    x.evaluate();
}
