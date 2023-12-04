use bevy::{prelude::*, sprite:: MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub fn floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(280.0, 10.0),
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2 { x: 280.0, y: 10.0}))).into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(Vec3::new(0.0, -70.0, -1.0)),
            ..default()
        },
    ));
}
