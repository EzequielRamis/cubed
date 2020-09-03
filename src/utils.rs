use bevy::prelude::Vec3;
use std::f32::consts::{FRAC_PI_2, PI};

pub trait RotationState {
    fn normalize_rotation(&self) -> Self;
}

impl RotationState for Vec3 {
    fn normalize_rotation(&self) -> Self {
        let value = |x: f32| match x.round() as i8 {
            0 | 6 | -6 => 0.0,
            2 | -5 => FRAC_PI_2,
            -2 | 5 => -FRAC_PI_2,
            3 => PI,
            -3 => -PI,
            _ => 0.0,
        };
        let x = value(self.x());
        let y = value(self.y());
        let z = value(self.z());
        Vec3::new(x, y, z)
    }
}
