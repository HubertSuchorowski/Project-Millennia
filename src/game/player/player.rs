use bevy::prelude::*;

#[derive(Component)] //marker
pub struct Player; 

#[derive(Component)] //staty
pub struct Stats {
    pub health: i32,
    pub strength: i32,
    pub defense: i32,
}

#[derive(Component)] //velocity
pub struct Velocity {
    pub dx: f32,
    pub dz: f32,
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
        Velocity { dx: 1.0, dz: 1.0 },

        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),

        Transform::from_xyz(0.0, 0.5, 0.0),
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
    mut query: Query <(&mut Transform, &Velocity)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time : Res<Time>,
){
    for(mut transfrom, velocity) in &mut query{
        if keyboard_input.pressed(KeyCode::KeyW){
            transfrom.translation.z -= velocity.dz * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            transfrom.translation.z += velocity.dz * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            transfrom.translation.x -= velocity.dx * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            transfrom.translation.x += velocity.dx * time.delta_secs();
        }
        if keyboard_input.pressed(KeyCode::Space){
            transfrom.translation.y += velocity.dz * time.delta_secs();
        }
    }
}
    




