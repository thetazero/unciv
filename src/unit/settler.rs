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
    pub color: Handle<StandardMaterial>,
    pub selected_color: Handle<StandardMaterial>,
}

pub fn init_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<StandardMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    SettlerResources,
    ResMut<'a, Assets<StandardMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let cube_mesh = Cuboid::new(
        tile::TILE_SIZE as f32 / 3.0,
        tile::TILE_SIZE as f32 / 3.0,
        tile::TILE_SIZE as f32 / 1.5,
    );
    let mesh = meshes.add(cube_mesh);

    let color = materials.add(colors::plastic_material(0., 0.8, 0.5));
    let selected_color = materials.add(colors::plastic_material(0., 0.8, 0.7));

    (
        SettlerResources {
            mesh,
            color,
            selected_color,
        },
        materials,
        meshes,
    )
}

impl UnitTrait for Settler {
    fn get_normal_material(
        &self,
        unit_resources: &Res<super::UnitResources>,
    ) -> Handle<StandardMaterial> {
        return unit_resources.settler.color.clone();
    }

    fn get_selected_material(
        &self,
        unit_resources: &Res<super::UnitResources>,
    ) -> Handle<StandardMaterial> {
        return unit_resources.settler.selected_color.clone();
    }

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
