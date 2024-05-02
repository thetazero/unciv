use bevy::input::keyboard::KeyCode;

pub struct KeyBinds {
    pub quit: KeyCode,
    pub zoom_in: KeyCode,
    pub zoom_out: KeyCode,
    pub action: KeyCode,
}

pub struct CameraConfig {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub max_z: f32,
    pub min_z: f32,
}

pub struct Config {
    pub camera: CameraConfig,
    pub world_size: (i32, i32),
    pub keys: KeyBinds,
}

pub const fn default_config() -> Config {
    Config {
        camera: CameraConfig {
            pan_speed: 1.0,
            zoom_speed: 0.7,
            max_z: 60.,
            min_z: 8.,
        },
        world_size: (100, 100),
        keys: KeyBinds {
            quit: KeyCode::CapsLock,
            zoom_in: KeyCode::Equal,
            zoom_out: KeyCode::Minus,
            action: KeyCode::Enter,
        },
    }
}

pub static CONFIG: Config = default_config();
