use bevy::prelude::{Mat4, Quat, Transform};
use std::f32::consts::SQRT_2;

pub trait NormalizedRotation {
    fn rotate_normalized(&mut self, quat: Quat);
}

impl NormalizedRotation for Transform {
    fn rotate_normalized(&mut self, rotation: Quat) {
        let normalize = |value: f32| -> f32 {
            match (value.abs() * 10.0) as u8 {
                0 => 0.0f32,
                9 | 10 => 1.0f32,
                4 | 5 => 0.5f32,
                7 => SQRT_2 / 2.0,
                _ => 0.0f32,
            }
            .copysign(value)
        };
        let rotation =
            Quat::from_rotation_mat4(&(Mat4::from_quat(rotation) * *self.value())).normalize();
        let normalized = Quat::from_xyzw(
            normalize(rotation.x()),
            normalize(rotation.y()),
            normalize(rotation.z()),
            normalize(rotation.w()),
        );
        self.set_rotation(normalized);
    }
}
