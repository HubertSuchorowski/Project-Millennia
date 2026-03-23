use avian3d::prelude::*;
use bevy::prelude::*;

#[path = "game/player/player.rs"]
mod player;


use player::{spawn_player,};

struct ColorLight;

impl ColorLight{
    const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player::player_movement_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    
){ 
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(10.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(10.0, 0.1))),
        MeshMaterial3d(materials.add(Color::WHITE)),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            color: ColorLight::BLUE,
            ..default()
        },
        Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
    ));
}
