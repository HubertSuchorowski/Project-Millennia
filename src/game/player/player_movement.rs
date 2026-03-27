use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use crate::player::{Player, Stats};
use crate::camera_settings::CameraSensitivity;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    
    mut player_query: Query<(&mut LinearVelocity, &Stats), With<Player>>,
    
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
) {
   
    let Ok((mut linear_velocity, stats)) = player_query.single_mut() else { return };
    let Ok(camera_transform) = camera_query.single() else { return };

    let speed = stats.speed as f32; 
    let jump_force = 5.0; 

  
    let forward = camera_transform.forward().as_vec3();
    let right = camera_transform.right().as_vec3();


    let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let flat_right = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

  
    let mut move_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        move_direction += flat_forward;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        move_direction -= flat_forward;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        move_direction += flat_right;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        move_direction -= flat_right;
    }

    
    move_direction = move_direction.normalize_or_zero();


    linear_velocity.x = move_direction.x * speed;
    linear_velocity.z = move_direction.z * speed;


    if keyboard_input.just_pressed(KeyCode::Space) {
        linear_velocity.y = jump_force; 
    }
}

pub fn player_camera_system(
    acumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Camera3d>>,
){
    let (mut transform, camera_sensitivity) = player.into_inner();
    let delta = acumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        
        let delta_pitch = -delta.y * camera_sensitivity.y;
        let delta_yaw = -delta.x * camera_sensitivity.x; 
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = 1.5; 
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        
    }
}