use bevy::input::keyboard::KeyCode;
pub struct Config {
    pub camera_speed: f32,
    pub quit_key: KeyCode,
}

pub const fn default_config() -> Config {
    Config {
        camera_speed: 200.,
        quit_key: KeyCode::CapsLock,
    }
}

pub static CONFIG: Config = default_config();
