use bevy::prelude::*;

use crate::{actions, building, controls, empire, resource, tile, unit, world_gen};

fn tick_units(
    mut commands: Commands,
    mut units: Query<(Entity, &Transform, &mut unit::Unit)>,
    world_state: &Res<world_gen::WorldState>,
    time: &Res<Time>,
) {
    for res in units.iter_mut() {
        let (unit_entity, transform, mut unit) = res;

        if let Some(target) = &unit.target {
            if target == &unit.location {
                unit.target = None;
            } else {
                let next_location = unit::next_location(&unit, &world_state);

                (commands, unit) = unit::next_location_update(
                    commands,
                    unit,
                    time,
                    transform,
                    &unit_entity,
                    &world_state,
                    &next_location,
                );
            }
        }
        unit.moved = false;
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
    commands: Commands,
    mut tile_query: Query<&tile::TileComponent>,
    mut empire_query: Query<&mut empire::Empire>,
    mut end_turn_reader: EventReader<EndTurnEvent>,
    world_state: Res<world_gen::WorldState>,
    unit_query: Query<(Entity, &Transform, &mut unit::Unit)>,
    time: Res<Time>,
) {
    for _ in end_turn_reader.read() {
        tick_units(commands, unit_query, &world_state, &time);

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
