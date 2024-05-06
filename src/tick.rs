use bevy::prelude::*;

use crate::{
    actions, building, controls, empire, resource,
    tile::{self, TILE_SIZE},
    unit, utils, world_gen,
};

fn tick_units(
    mut units: Query<(&mut Transform, &mut unit::Unit)>,
    world_state: &Res<world_gen::WorldState>,
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

        let tile_data = world_state.tile_data.get(&unit.location).unwrap();
        transform.translation.z = tile_data.height + TILE_SIZE / 2.;
    }
}

pub fn execute_actions(
    mut action_reader: EventReader<ActionEvent>,
    mut commands: Commands,
    mut selector_state: ResMut<controls::SelectorState>,
    mut tile_query: Query<&mut tile::TileComponent>,
    mut end_turn_writer: EventWriter<EndTurnEvent>,
    building_resources: Res<building::BuildingResources>,
    unit_resources: Res<unit::UnitResources>,
    world_state: Res<world_gen::WorldState>,
) {
    for action_event in action_reader.read() {
        (tile_query, selector_state, commands, end_turn_writer) = actions::execute(
            action_event.action.clone(),
            tile_query,
            selector_state,
            commands,
            end_turn_writer,
            &building_resources,
            &unit_resources,
            &world_state,
        );
    }
}

pub fn tick_world(
    mut tile_query: Query<&tile::TileComponent>,
    mut empire_query: Query<&mut empire::Empire>,
    world_state: Res<world_gen::WorldState>,
    units: Query<(&mut Transform, &mut unit::Unit)>,
    mut end_turn_reader: EventReader<EndTurnEvent>,
) {
    for _ in end_turn_reader.read() {
        tick_units(units, &world_state);

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
        break;
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

#[derive(Event)]
pub struct EndTurnEvent;
