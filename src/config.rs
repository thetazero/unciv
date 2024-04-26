use bevy::input::keyboard::KeyCode;
use bevy::math::Vec3;

pub struct KeyBinds {
    pub quit: KeyCode,
    pub zoom_in: KeyCode,
    pub zoom_out: KeyCode,
}

pub struct CameraConfig {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub max_zoom: Vec3,
    pub min_zoom: Vec3,
}

pub struct Config {
    pub camera: CameraConfig,
    pub world_size: (i32, i32),
    pub keys: KeyBinds,
}

pub const fn default_config() -> Config {
    Config {
        camera: CameraConfig {
            pan_speed: 200.0,
            zoom_speed: 1.4,
            max_zoom: Vec3::new(6.0, 6.0, 6.0),
            min_zoom: Vec3::new(0.4, 0.4, 0.4),
        },
        world_size: (100, 100),
        keys: KeyBinds {
            quit: KeyCode::CapsLock,
            zoom_in: KeyCode::Equal,
            zoom_out: KeyCode::Minus,
        },
    }
}

pub static CONFIG: Config = default_config();
