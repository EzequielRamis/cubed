use crate::entity::Piece;
use bevy::input::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn keyboard_input(input: Res<Input<KeyCode>>, mut query: Query<(&Piece, &mut Transform)>) {
    for (_, mut transform) in &mut query.iter() {
        if input.just_pressed(KeyCode::W) {
            transform.rotate(Quat::from_rotation_x(-FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::A) {
            transform.rotate(Quat::from_rotation_z(FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::S) {
            transform.rotate(Quat::from_rotation_x(FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::D) {
            transform.rotate(Quat::from_rotation_z(-FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::Q) {
            transform.rotate(Quat::from_rotation_y(FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::E) {
            transform.rotate(Quat::from_rotation_y(-FRAC_PI_2));
        }
        if input.just_pressed(KeyCode::I) {
            transform.translate(-Vec3::unit_z());
        }
        if input.just_pressed(KeyCode::K) {
            transform.translate(Vec3::unit_z());
        }
        if input.just_pressed(KeyCode::J) {
            transform.translate(-Vec3::unit_x());
        }
        if input.just_pressed(KeyCode::L) {
            transform.translate(Vec3::unit_x());
        }
    }
}
