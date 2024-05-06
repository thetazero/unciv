use bevy::prelude::*;

use crate::{building, controls, tile, unit, utils, world_gen};

#[derive(Clone, Component, Debug)]
pub enum Action {
    Build(Build),
    KillUnit(Entity),
    _Spawn(Spawn),
    SpawnBasedOnSelectorState(unit::UnitKind),
    Noop,
}

#[derive(Clone, Debug)]
pub struct Spawn {
    pub location: utils::Coordinates,
    pub unit: unit::Unit,
}

#[derive(Clone, Debug)]
pub struct Build {
    pub building: building::Building,
    pub tile_entity: Entity,
    pub owner: i32,
}

pub fn execute<'a, 'b, 'c, 'd, 'f, 'g>(
    action: Action,
    mut tile_query: Query<'a, 'b, &'c mut tile::TileComponent>,
    mut selector_state: ResMut<'d, controls::SelectorState>,
    mut commands: Commands<'f, 'g>,
    building_resources: &Res<building::BuildingResources>,
    unit_resources: &Res<unit::UnitResources>,
    world_state: &Res<world_gen::WorldState>,
) -> (
    Query<'a, 'b, &'c mut tile::TileComponent>,
    ResMut<'d, controls::SelectorState>,
    Commands<'f, 'g>,
) {
    match action {
        Action::Build(build) => {
            let mut tile = tile_query.get_mut(build.tile_entity).unwrap();

            tile.owner = Some(build.owner);

            tile.building = Some(build.building.clone());

            let building_bundle = building::make_bundle(&build.building, &building_resources);
            let building_id = commands.spawn(building_bundle).id();
            commands
                .entity(build.tile_entity)
                .push_children(&[building_id]);
        }
        Action::KillUnit(unit_entity) => {
            selector_state.selected_unit = None;

            commands.entity(unit_entity).despawn();
        }
        Action::_Spawn(spawn) => {
            let _unit_bundle = unit::make_bundle(spawn.unit, &unit_resources, &world_state);
            println!("Not implemented");
        }
        Action::SpawnBasedOnSelectorState(unit_kind) => match selector_state.selected_tile {
            Some(tile_entity) => {
                let tile = tile_query.get(tile_entity).unwrap();

                let location = tile.tile.location.clone();
                let unit = unit::Unit {
                    location,
                    owner: Some(0),
                    target: None,
                    kind: unit_kind,
                };
                let unit_bundle = unit::make_bundle(unit, &unit_resources, &world_state);

                commands.spawn(unit_bundle);
            }
            None => {
                println!("No tile selected")
            }
        },
        Action::Noop => {
            println!("Noop")
        }
    }

    (tile_query, selector_state, commands)
}
