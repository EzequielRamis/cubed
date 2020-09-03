use crate::component::{Tetracube, TetracubeShape};
use crate::entity::{Cube, Piece};
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(0.1, 8.0),
                flip: false,
            })),
            material: materials.add(Color::GREEN.into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(0.1, 8.0),
                flip: false,
            })),
            material: materials.add(Color::BLUE.into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            rotation: Rotation::from_rotation_x(-90f32.to_radians()),
            ..Default::default()
        })
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(0.1, 8.0),
                flip: false,
            })),
            material: materials.add(Color::RED.into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            rotation: Rotation::from_rotation_z(-90f32.to_radians()),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::new_sync_disabled(Mat4::face_toward(
                Vec3::new(10.0, 10.0, 10.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.0 })),
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .with(Piece)
        .with_children(|piece| {
            let tetra = Tetracube::from(TetracubeShape::T);
            for cube in &tetra.position {
                piece
                    .spawn(PbrComponents {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        material: materials.add(Color::GREEN.into()),
                        translation: Translation::new(cube.x(), cube.y(), cube.z()),
                        ..Default::default()
                    })
                    .with(Cube);
            }
        });
}
