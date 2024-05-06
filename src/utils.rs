use bevy::{prelude::*, utils::HashMap};
use std::ops::AddAssign;

use crate::resource;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Copy, Component)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn to_world_location(coordinates: &Coordinates) -> (f32, f32) {
    (
        coordinates.x as f32 * (crate::tile::TILE_SIZE),
        coordinates.y as f32 * (crate::tile::TILE_SIZE),
    )
}

pub fn to_transform(coordinates: &Coordinates) -> Transform {
    let (x, y) = to_world_location(coordinates);

    Transform::from_translation(Vec3::new(x, y, 0.))
}

pub const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];


pub struct Inventory {
    pub items: HashMap<resource::Resource, i32>,
    pub capacity: i32
}
