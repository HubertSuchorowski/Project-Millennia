use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(Component)] 
pub struct Player; 

#[derive(Component)] 
pub struct Stats {
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
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
        },
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED, 
    LinearVelocity::default(),
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    Collider::cuboid(1.0, 1.0, 1.0),
    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    Transform::from_xyz(0.0, 5.0, 0.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform {
                translation: Vec3::new(0.0, 2.0, 3.0),
                rotation: Quat::from_rotation_x(-0.5),
                ..default()
            },
        ));
    });
}

pub fn player_movement_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
    time: Res<Time>,
){
    for mut linear_velocity in &mut query{
        linear_velocity.x = 0.0;
        linear_velocity.z = 0.0;
        {
            if keyboard_input.pressed(KeyCode::KeyW){
            linear_velocity.z -= 1000.0 * time.delta_secs(); 
            }
            if keyboard_input.pressed(KeyCode::KeyS){
            linear_velocity.z += 1000.0 * time.delta_secs(); 
            }
            if keyboard_input.pressed(KeyCode::KeyA){
            linear_velocity.x -= 1000.0 * time.delta_secs(); 
            }
            if keyboard_input.pressed(KeyCode::KeyD){
            linear_velocity.x += 1000.0 * time.delta_secs(); 
            }
        }
    }
}




