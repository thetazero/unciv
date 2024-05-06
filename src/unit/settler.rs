use bevy::prelude::*;

use super::UnitTrait;
use crate::{actions, building, colors, tile};

#[derive(Clone, Debug)]
pub struct Settler {}

impl Default for Settler {
    fn default() -> Self {
        Settler {}
    }
}

pub struct SettlerResources {
    pub mesh: Handle<Mesh>,
}

pub fn init_resources<'a>(
    mut meshes: ResMut<'a, Assets<Mesh>>,
) -> (SettlerResources, ResMut<'a, Assets<Mesh>>) {
    let cube_mesh = Cuboid::new(
        tile::TILE_SIZE as f32 / 3.0,
        tile::TILE_SIZE as f32 / 3.0,
        tile::TILE_SIZE as f32 / 1.5,
    );
    let mesh = meshes.add(cube_mesh);

    (SettlerResources { mesh }, meshes)
}

impl UnitTrait for Settler {
    fn tile_action(
        &self,
        tile: Mut<tile::TileComponent>,
        unit_entity: Entity,
        tile_entity: Entity,
        acting_empire: i32,
    ) -> Vec<actions::Action> {
        if !tile::is_settleable(&tile.tile.kind) {
            println!("Can't build on this tile");

            return vec![];
        }
        match &tile.building {
            None => {
                let city = building::Building::City(default());

                vec![
                    actions::Action::Build(actions::Build {
                        building: city,
                        tile_entity: tile_entity,
                        owner: acting_empire,
                    }),
                    actions::Action::KillUnit(unit_entity),
                ]
            }
            Some(_) => {
                println!("Can't build over a building");

                vec![]
            }
        }
    }
}
