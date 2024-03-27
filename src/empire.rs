use bevy::prelude::*;

#[derive(Component)]
pub struct Empire {
    pub id: i32,
}

#[derive(Component)]
pub struct Inventory {
    pub wood: i32,
    pub stone: i32,
}
