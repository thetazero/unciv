use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::{empire, resource, tile, world_gen};

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
    mut empire_query: Query<&mut empire::Empire>,
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
            let empire: Mut<'_, empire::Empire> =
                empire_query.get_mut(owner_entity.clone()).unwrap();

            match tile.kind {
                tile::TileKind::Forest => {
                    add_item(empire, resource::Resource::Wood, 1);
                }
                tile::TileKind::Mountain => {
                    add_item(empire, resource::Resource::Stone, 1);
                }
                _ => (),
            }
        }
    }
}

fn add_item<'a>(
    mut empire: Mut<'a, empire::Empire>,
    item: resource::Resource,
    amount: i32,
) -> Mut<'a, empire::Empire> {
    let current_amount = empire.inventory.inv.get(&item).unwrap_or(&0).clone();
    empire.inventory.inv.insert(item, current_amount + amount);
    return empire;
}
