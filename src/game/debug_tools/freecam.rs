use bevy::prelude::*;
use crate::camera_settings::CameraSensitivity;
use bevy::input::mouse::AccumulatedMouseMotion;

pub fn freecam_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut translation: Query<&mut Transform, With<Camera3d>>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    time: Res<Time>,
){
    {
        let Ok(camera_transform) = camera_query.single() else { return };
        let Ok(mut transform) = translation.single_mut() else { return };

        let speed = 5.0;
        let forward = camera_transform.forward().as_vec3();
        let right = camera_transform.right().as_vec3();
        let mut move_direction = Vec3::ZERO;

        let forward = transform.forward().as_vec3();
        let right = transform.right().as_vec3();
        let up = transform.up().as_vec3();

        if keyboard_input.pressed(KeyCode::KeyW){
            move_direction += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            move_direction -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            move_direction -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            move_direction += right;
        }
        if keyboard_input.pressed(KeyCode::Space){
            move_direction.y += up.y;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft){
            move_direction.y -= up.y;
        }

        transform.translation.x += move_direction.x * speed * time.delta_secs();
        transform.translation.z += move_direction.z * speed * time.delta_secs();  
        transform.translation.y += move_direction.y * speed * time.delta_secs();
    }
}

pub fn debug_camera_system(
    acumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Camera3d>>,
){
    let (mut transform, camera_sensitivity) = player.into_inner();
    let delta = acumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = 1.5; 
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
