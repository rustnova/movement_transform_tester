use bevy::{math::vec2, math::vec3, prelude::*};
use rand::Rng;

use crate::{
    equations_of_motion::{Destination, EquationsOfMotion, Momentum},
    movement_transform_tester::{PrintResource, Result, Status, TestMarker},
    State,
};
pub struct MotionTest;

impl Plugin for MotionTest {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(start.system())
            .add_system(movement.system())
            .add_system(evaluate.system());
    }
}

fn evaluate(
    mut printer: ResMut<PrintResource>,
    time: Res<Time>,
    mut query: Query<(&mut TestMarker<State>, &Transform)>,
) {
    let mut pending = false;
    for (mut marker, transform) in &mut query.iter() {
        if time.seconds_since_startup > 10. && marker.result.status == Status::Pending {
            marker.new_state = Some(State {
                transform: *transform,
                ..Default::default()
            });
            marker.evaluate(&mut printer);
        }
        if marker.result.status == Status::Pending {
            pending = true
        }
    }
    if printer.printed == false && pending == false {
        println!("time to print!");
        printer.print();
    }
}
fn start(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let trans = Transform::from_translation_rotation(
            vec3(
                rng.gen_range(-500., 500.),
                rng.gen_range(-500., 500.),
                rng.gen_range(-500., 500.),
            ),
            Quat::from_rotation_z(rng.gen_range(0.001, 6.28)),
        );
        let momentum = Momentum {
            thrust: rng.gen_range(0.001, 0.1),
            max_rotation: rng.gen_range(0.01, 0.1),
            inertia: Vec2::default(),
        };
        let original_state = State {
            transform: trans,
            motion_values: momentum,
            ..Default::default()
        };
        let testmarker = TestMarker {
            original_state: Some(original_state),
            new_state: None,
            result: Result::default(),
        };

        commands
            .spawn((trans,))
            .with(momentum)
            .with(Destination {
                d: Vec3::new(2000., 10000., 0.0),
            })
            .with(testmarker);
    }
}
fn movement(mut query: Query<(&mut Momentum, &Destination, &mut Transform)>) {
    for (mut momentum, destination, mut transform) in &mut query.iter() {
        let mut pos = transform.translation();
        let null = Vec3::new(0.0, 0.0, 0.0);
        let mask = pos.cmpeq(null);
        if mask.all() {
            pos.set_x(1.0);
        }

        let facing = (transform.rotation().mul_vec3(Vec3::unit_y())).normalize();
        let vector_to_dest = (destination.d - pos).normalize();

        let (axis, angle) = momentum.turn_to(facing, vector_to_dest);
        let s = (momentum.max_rotation() / angle).abs();
        let look_at = Quat::default_to_vec3(vector_to_dest);

        let rot = transform.rotation().normalize();

        if rot.dot(look_at) < 0.0 {
            transform.set_rotation(rot.slerp(-look_at, s.min(1.0)));
        } else {
            transform.set_rotation(rot.slerp(look_at, s.min(1.0)));
        }

        // let s = momentum.max_rotation() / facing.angle_between(vector_to_dest);
        // let dot = facing.dot(vector_to_dest);
        // // let final_angle = Vec3::unit_y().angle_between(vector_to_dest);
        // // let facing_angle = Vec3::unit_y().angle_between(facing);
        // // let turn_angle = final_angle - facing_angle;
        // // let final_angle = momentum.turn_to(facing, vector_to_dest);
        // // if final_angle > 0.00001 {
        // //     transform.set_rotation(Quat::from_rotation_z(facing_angle+turn_angle));
        // // }
        //     let (axis, angle) = momentum.turn_to(facing, vector_to_dest);
        //     transform.rotate(Quat::from_axis_angle(axis, angle).normalize());

        let thrust = momentum.thrust();
        // momentum.inertia += vec2(0.0, 0.1);
        momentum.inertia += vec2(
            facing.x().sin() * thrust,
            (facing.y().cos() * thrust).copysign(facing.y()),
        );

        transform.translate(momentum.inertia().extend(0.0));

        // println!(
        //     "a {:<13?} d {:<13?} t {:<13?} f {:<13?} ax {:<13?}",
        //     angle,
        //     //  0,0
        //     // angle_to_turn,
        //     momentum.inertia,
        //     transform.translation(),
        //     // transform.rotation().to_axis_angle(),
        //     facing,
        //     axis
        // );
    }
}

trait QuatMath {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat;
    fn default_to_vec3(to: Vec3) -> Quat;
}

impl QuatMath for Quat {
    fn from_to_vec3(from: Vec3, to: Vec3) -> Quat {
        let from_vec = from.normalize();
        let to_vec = to.normalize();
        let dot = from_vec.dot(to_vec);
        if dot >= 1.0 {
            return Quat::identity();
        }
        if dot < 1e-6_f32 - 1.0 {
            let mut axis = Vec3::unit_x().cross(from);
            if axis.length() == 0.0 {
                axis = Vec3::unit_y().cross(from);
            }
            return Quat::from_axis_angle(axis.normalize(), std::f32::consts::PI);
        }
        let angle = dot.acos();
        Quat::from_axis_angle(from_vec.cross(to_vec).normalize(), angle).normalize()
    }
    fn default_to_vec3(forward: Vec3) -> Quat {
        Quat::from_to_vec3(Vec3::unit_y(), forward)
    }
}
