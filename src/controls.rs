use bevy::prelude::*;

use crate::config::CONFIG;

pub fn handle_keyboard(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(CONFIG.keys.quit) {
        app_exit_events.send(bevy::app::AppExit);
    }

    let mut delta_x = 0.;
    let mut delta_y = 0.;
    if keyboard_input.pressed(KeyCode::KeyW) {
        delta_y += CONFIG.camera.pan_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        delta_y -= CONFIG.camera.pan_speed;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        delta_x -= CONFIG.camera.pan_speed;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        delta_x += CONFIG.camera.pan_speed;
    }

    let mut delta_scale = 0.;
    if keyboard_input.pressed(CONFIG.keys.zoom_in) {
        delta_scale -= CONFIG.camera.zoom_speed;
    } else if keyboard_input.pressed(CONFIG.keys.zoom_out) {
        delta_scale += CONFIG.camera.zoom_speed;
    }

    for mut transform in camera.iter_mut() {
        transform.scale *= 1. + delta_scale * time.delta_seconds();
        transform.scale = transform
            .scale
            .clamp(CONFIG.camera.min_zoom, CONFIG.camera.max_zoom);

        let scale_magnitude = transform.scale.length();

        transform.translation.x += delta_x * time.delta_seconds() * scale_magnitude;
        transform.translation.y += delta_y * time.delta_seconds() * scale_magnitude;
    }
}
