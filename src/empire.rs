use bevy::prelude::*;

#[derive(Component)]
pub struct Empire {

}

#[derive(Component)]
pub struct Inventory {
    pub wood: i32,
    pub stone: i32,
}
