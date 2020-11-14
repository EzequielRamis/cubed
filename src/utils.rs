use bevy::prelude::{GlobalTransform, Mat4, Quat, Transform};
use std::f32::consts::SQRT_2;

pub trait NormalizedTransform {
    fn rotate_normalized(&mut self, quat: Quat);
    fn translate_normalized(&mut self);
}

impl NormalizedTransform for Transform {
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
        let rotation = Quat::from_rotation_mat4(&(Mat4::from_quat(rotation) * *self.value()));
        let rotation_normalized = Quat::from_xyzw(
            normalize(rotation.x()),
            normalize(rotation.y()),
            normalize(rotation.z()),
            normalize(rotation.w()),
        );
        self.set_rotation(rotation_normalized);
        self.translate_normalized();
    }

    fn translate_normalized(&mut self) {
        let translation_normalized = self.translation().round();
        self.set_translation(translation_normalized);
    }
}

impl NormalizedTransform for GlobalTransform {
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
        let rotation = Quat::from_rotation_mat4(&(Mat4::from_quat(rotation) * *self.value()));
        let rotation_normalized = Quat::from_xyzw(
            normalize(rotation.x()),
            normalize(rotation.y()),
            normalize(rotation.z()),
            normalize(rotation.w()),
        );
        self.set_rotation(rotation_normalized);
        self.translate_normalized();
    }

    fn translate_normalized(&mut self) {
        let translation_normalized = self.translation().round();
        self.set_translation(translation_normalized);
    }
}
