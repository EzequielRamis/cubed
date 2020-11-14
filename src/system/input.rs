use crate::entity::Cube;
use crate::entity::Piece;
use crate::utils::NormalizedTransform;
use bevy::input::*;
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_2;

pub fn keyboard_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Piece, &mut Transform)>,
    mut cubes: Query<(&Cube, &mut GlobalTransform)>,
) {
    for (_, mut transform) in &mut query.iter() {
        if input.just_pressed(KeyCode::W) {
            transform.rotate_normalized(Quat::from_rotation_x(-FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::S) {
            transform.rotate_normalized(Quat::from_rotation_x(FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::E) {
            transform.rotate_normalized(Quat::from_rotation_y(-FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::Q) {
            transform.rotate_normalized(Quat::from_rotation_y(FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::D) {
            transform.rotate_normalized(Quat::from_rotation_z(-FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::A) {
            transform.rotate_normalized(Quat::from_rotation_z(FRAC_PI_2));
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::J) {
            transform.translate(-Vec3::unit_x());
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::L) {
            transform.translate(Vec3::unit_x());
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::I) {
            transform.translate(-Vec3::unit_z());
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
        if input.just_pressed(KeyCode::K) {
            transform.translate(Vec3::unit_z());
            for (_, mut cube) in &mut cubes.iter() {
                cube.translate_normalized();
            }
        }
    }
}
