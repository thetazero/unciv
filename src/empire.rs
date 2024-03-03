use bevy::prelude::*;

#[derive(Component)]
struct Empire;

#[derive(Component)]
pub struct Inventory {
    pub wood: i32,
    pub stone: i32,
}
