// use bevy::prelude::*;

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

#[derive(PartialEq, Eq, Clone, Debug)]
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
        coordinates.x as f32 * (crate::tile::TILE_SIZE + 1.),
        coordinates.y as f32 * (crate::tile::TILE_SIZE + 1.),
    )
}
