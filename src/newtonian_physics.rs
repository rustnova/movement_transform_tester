use bevy::{math::vec3, math::Vec2, math::Vec3, prelude::Transform};

/*

2d:
F^2 = F_x^2 + F_y^2 (use Pythagorean theorem to split force into components)
F_x = m * a_x
F_y = m * a_y
s_x = s_o_x + v_x * t + a_x * t^2 / 2
s_y = s_o_y + v_y * t + a_y * t^2 / 2
v_x = v_o_x + a_x * t
v_y = v_o_y + a_y * t

F = force
a = accel
m = mass
s = current pos
s_o = original pos
v = velocity
t = current time (since starting)

*/

const UP: (f32, f32, f32) = (0.0, 1.0, 0.0);

pub trait NewtonStraightLines {
    fn force(&self, mass: &f32, accel: &f32) -> Vec3;
    fn position_at_velocity(&self, vel_future: Vec3, time: f32, accel: f32) -> Vec3;
    fn velocity_after_time(&self, vel_current: Vec3, accel: f32, time: f32) -> Vec3;
    fn velocity_at_position_and_time(&self, final_pos: Vec3, time: f32) -> Vec3;
}
impl NewtonStraightLines for Transform {
    /// Force = mass * accel
    fn force(&self, mass: &f32, accel: &f32) -> Vec3 {
        (*mass * *accel) * self.facing().normalize()
    }
    /// final_position = pos_init + final_velocity * time + accel * time^2 / 2
    fn position_at_velocity(&self, vel_future: Vec3, time: f32, accel: f32) -> Vec3 {
        self.translation()
            + (vel_future * time)
            + (self.facing().normalize() * accel) * time.powi(2) / 2.0
    }
    /// velocity = vel_init + accel * time
    fn velocity_after_time(&self, vel_current: Vec3, accel: f32, time: f32) -> Vec3 {
        vel_current + self.facing().normalize() * accel * time
    }
    /// velocity = (final_pos - current_pos) / time
    fn velocity_at_position_and_time(&self, final_pos: Vec3, time: f32) -> Vec3 {
        (final_pos - self.translation()) / time
    }
    
}

pub trait MathUpgrade {
    fn facing(&self) -> Vec3;
    fn distance(&self, other: Vec3) -> f32;
}

impl MathUpgrade for Transform {
    fn facing(&self) -> Vec3 {
        self.rotation().mul_vec3(Vec3::from(UP))
    }
    fn distance(&self, other: Vec3) -> f32 {
        (other - self.translation()).length()
    }
}
