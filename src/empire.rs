use std::collections::HashMap;

use bevy::prelude::*;

use crate::resource;

#[derive(Component)]
pub struct Empire {
    pub id: i32,
    pub color: Handle<ColorMaterial>,
    pub inventory: Inventory,
}

#[derive(Component)]
pub struct Inventory {
    pub inv: HashMap<resource::Resource, i32>,
}
