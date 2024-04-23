use bevy::prelude::*;

use crate::{empire, world_gen};

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
