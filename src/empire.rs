use bevy::prelude::*;

use crate::utils;

#[derive(Component)]
pub struct Empire {
    pub id: i32,
    pub color: Handle<ColorMaterial>,
    pub inventory: utils::Inventory,
}

pub fn id_to_hue(empire_id: i32) -> f32 {
    empire_id as f32 * 36.0
}
