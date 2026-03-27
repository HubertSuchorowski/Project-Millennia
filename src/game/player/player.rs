use avian3d::prelude::*;
use bevy::prelude::*;
use crate::camera_settings::CameraSensitivity;


#[derive(Component)] 
pub struct Player; 

#[derive(Component)] 
pub struct Stats {
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
    pub speed: i32,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
    .spawn((
        Player,
        Stats {
            health: 100,
            strength: 10,
            defense: 5,
            speed: 10,
        },
    RigidBody::Dynamic,
    LinearVelocity::default(),
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    Collider::cuboid(1.0, 1.0, 1.0),
    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    Transform::from_xyz(0.0, 5.0, 0.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            CameraSensitivity::default(),
            Transform {
                translation: Vec3::new(0.0, 2.0, 3.0),
                rotation: Quat::from_rotation_x(-0.5),
                ..default()
            },
        ));
    });
}






