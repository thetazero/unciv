use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::{empire, tile, world_gen};

#[derive(Resource)]
pub struct TickState {
    stop_watch: Stopwatch,
}

pub fn init_tick(mut commands: Commands) {
    commands.insert_resource(TickState {
        stop_watch: Stopwatch::new(),
    })
}

pub fn tick_world(
    mut tile_query: Query<(Entity, &tile::Tile)>,
    mut empire_query: Query<(Entity, &mut empire::Empire)>,
    time: ResMut<Time>,
    mut tick_state: ResMut<TickState>,
    world_state: ResMut<world_gen::WorldState>,
) {
    tick_state.stop_watch.tick(time.delta());

    if tick_state.stop_watch.elapsed() < Duration::from_secs(1) {
        return;
    } else {
        tick_state.stop_watch.reset();
    }

    for (_entity, tile) in tile_query.iter_mut() {
        if let Some(owner) = tile.owner {
            let owner_entity = world_state.empires.get(&owner).unwrap();
            let (_, mut empire) = empire_query.get_mut(owner_entity.clone()).unwrap();

            match tile.kind {
                tile::TileKind::Forest => empire.inventory.wood += 1,
                tile::TileKind::Mountain => empire.inventory.stone += 1,
                _ => (),
            }
        }
    }
}
