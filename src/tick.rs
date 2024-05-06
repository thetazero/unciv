use std::time::Duration;

use bevy::prelude::*;
use bevy::time::Stopwatch;

use crate::{
    actions, building, controls, empire, resource,
    tile::{self, TILE_SIZE},
    unit, utils, world_gen,
};

#[derive(Resource)]
pub struct TickState {
    stop_watch: Stopwatch,
}

pub fn init_tick(mut commands: Commands) {
    commands.insert_resource(TickState {
        stop_watch: Stopwatch::new(),
    })
}

fn tick_units(
    mut units: Query<(&mut Transform, &mut unit::Unit)>,
    world_state: &ResMut<world_gen::WorldState>,
    tiles: &Query<&tile::TileComponent>,
) {
    for res in units.iter_mut() {
        let (mut transform, mut unit) = res;
        if let Some(target) = &unit.target {
            if target == &unit.location {
                unit.target = None;
            } else {
                let delta = utils::Coordinates {
                    x: (target.x - unit.location.x).clamp(-1, 1),
                    y: (target.y - unit.location.y).clamp(-1, 1),
                };

                unit.location += delta;

                let (x, y) = utils::to_world_location(&unit.location);

                transform.translation.x = x;
                transform.translation.y = y;
            }
        }

        let tile_entity = world_state.tiles.get(&unit.location).unwrap();
        let tile = tiles.get(*tile_entity).unwrap();
        transform.translation.z = tile.tile.height + TILE_SIZE / 2.;
    }
}

pub fn execute_actions(
    mut action_reader: EventReader<ActionEvent>,
    mut commands: Commands,
    mut selector_state: ResMut<controls::SelectorState>,
    mut tile_query: Query<&mut tile::TileComponent>,
    building_resources: Res<building::BuildingResources>,
    unit_resources: Res<unit::UnitResources>,
) {
    for action_event in action_reader.read() {
        (tile_query, selector_state, commands) = actions::execute(
            action_event.action.clone(),
            tile_query,
            selector_state,
            commands,
            &building_resources,
            &unit_resources,
        );
    }
}

pub fn tick_world(
    mut tile_query: Query<&tile::TileComponent>,
    mut empire_query: Query<&mut empire::Empire>,
    time: ResMut<Time>,
    mut tick_state: ResMut<TickState>,
    world_state: ResMut<world_gen::WorldState>,
    units: Query<(&mut Transform, &mut unit::Unit)>,
) {
    tick_state.stop_watch.tick(time.delta());

    if tick_state.stop_watch.elapsed() < Duration::from_secs(1) {
        return;
    } else {
        tick_state.stop_watch.reset();
    }

    tick_units(units, &world_state, &tile_query);

    for tile in tile_query.iter_mut() {
        if let Some(owner) = tile.owner {
            let owner_entity = world_state.empires.get(&owner).unwrap();
            let mut empire: Mut<'_, empire::Empire> =
                empire_query.get_mut(owner_entity.clone()).unwrap();

            if let Some(building) = &tile.building {
                let production = building::building_production(building);
                for (resource, amount) in production {
                    empire = add_item(empire, resource, amount);
                }
            }

            match tile.tile.kind {
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
    let current_amount = empire.inventory.items.get(&item).unwrap_or(&0).clone();
    empire.inventory.items.insert(item, current_amount + amount);
    return empire;
}

#[derive(Event)]
pub struct ActionEvent {
    pub action: actions::Action,
}
