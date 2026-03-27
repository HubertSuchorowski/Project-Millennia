use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};


#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Default)]
enum GameMode {
    #[default]
    Playing,
    Freecam,
}

#[path = "game/player/player.rs"]
mod player;

#[path = "game/player/player_movement.rs"]
mod player_movement;

#[path = "game/debug_tools/freecam.rs"]
mod freecam;

#[path = "game/settings/camera_settings.rs"]
mod camera_settings;

use player::{spawn_player,};

struct ColorLight;

impl ColorLight{
    const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, capture_cursor) 
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement::player_movement_system.run_if(in_state(GameMode::Playing)))
        .add_systems(Update, player_movement::player_camera_system.run_if(in_state(GameMode::Playing)))
        .add_systems(Update, freecam::freecam_system.run_if(in_state(GameMode::Freecam)))
        .init_state::<GameMode>()
        .add_systems(Update, toggle_mode)
        .add_systems(Update, toggle_cursor)
        .add_systems(Update, freecam::debug_camera_system.run_if(in_state(GameMode::Freecam)))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _keyboard_input: Res<ButtonInput<KeyCode>>,
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

fn toggle_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameMode>>,
    state: Res<State<GameMode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyT) {
        match state.get() {
            GameMode::Playing => next_state.set(GameMode::Freecam),
            GameMode::Freecam => next_state.set(GameMode::Playing),
        }
    }
}

// --- SEKTYCJA CURSORA  ---

fn capture_cursor(
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn toggle_cursor(
    mut cursor: Single<&mut CursorOptions, With<PrimaryWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        if cursor.grab_mode == CursorGrabMode::None {
            cursor.grab_mode = CursorGrabMode::Locked;
            cursor.visible = false;
        } else {
            cursor.grab_mode = CursorGrabMode::None;
            cursor.visible = true;
        }
    }
}