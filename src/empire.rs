use bevy::prelude::*;

use crate::utils;

#[derive(Component)]
pub struct Empire {
    pub id: i32,
    pub color: Handle<ColorMaterial>,
    pub inventory: utils::Inventory,
}
