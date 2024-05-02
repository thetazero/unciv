use bevy::prelude::*;

// use crate::{empire, world_gen};

// pub fn get_empire_from_id<'a>(
//     empire_id: i32,
//     empire_query:  Query<(Entity, & empire::Empire)>,
//     world_state: Res<world_gen::WorldState>,
// ) -> Option<&'a empire::Empire> {
//     let empire_entity = world_state.empires.get(&empire_id);

//     if empire_entity.is_none() {
//         return None;
//     }

//     let (_, empire) = empire_query.get(*empire_entity.unwrap()).unwrap();

//     return Some(empire);
// }

use std::ops::AddAssign;

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
