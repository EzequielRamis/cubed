use crate::entity::Piece;
use crate::utils::*;
use bevy::input::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn keyboard_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Piece, &mut Rotation, &mut Translation)>,
) {
    for (_, mut rotation, mut translation) in &mut query.iter() {
        if input.just_pressed(KeyCode::W) {}
        if input.just_pressed(KeyCode::A) {}
        if input.just_pressed(KeyCode::S) {}
        if input.just_pressed(KeyCode::D) {}
        if input.just_pressed(KeyCode::Q) {}
        if input.just_pressed(KeyCode::E) {}
        if input.just_pressed(KeyCode::I) {
            translation.0 -= Vec3::unit_z();
        }
        if input.just_pressed(KeyCode::K) {
            translation.0 += Vec3::unit_z();
        }
        if input.just_pressed(KeyCode::J) {
            translation.0 -= Vec3::unit_x();
        }
        if input.just_pressed(KeyCode::L) {
            translation.0 += Vec3::unit_x();
        }
    }
}
