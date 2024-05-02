use bevy::prelude::*;

use crate::{building, controls, tile};
pub enum Action {
    Build(Build),
    KillUnit(Entity),
}

pub struct Build {
    pub building: building::Building,
    pub tile_entity: Entity,
    pub owner: i32,
}

pub fn execute<'a, 'b, 'c, 'd, 'e, 'f, 'g>(
    action: Action,
    mut tile_query: Query<'a, 'b, &'c mut tile::Tile>,
    mut selector_state: ResMut<'d, controls::SelectorState>,
    building_resources: &Res<'e, building::BuildingResources>,
    mut commands: Commands<'f, 'g>,
) -> (
    Query<'a, 'b, &'c mut tile::Tile>,
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
    }

    (tile_query, selector_state, commands)
}
