use bevy::input::keyboard::KeyCode;
pub struct Config {
    pub camera_speed: f32,
    pub quit_key: KeyCode,
    pub world_size: (i32, i32),
}

pub const fn default_config() -> Config {
    Config {
        camera_speed: 200.,
        quit_key: KeyCode::CapsLock,
        world_size: (30, 30),
    }
}

pub static CONFIG: Config = default_config();
