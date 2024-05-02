use bevy::prelude::*;

use super::UnitTrait;
use crate::{actions, building, tile};

pub struct Settler {}

impl Default for Settler {
    fn default() -> Self {
        Settler {}
    }
}

pub struct SettlerResources {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
    pub selected_color: Handle<ColorMaterial>,
}

pub fn init_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<ColorMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    SettlerResources,
    ResMut<'a, Assets<ColorMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(Circle { radius: 20.0 }));

    let color = materials.add(Color::hsl(0., 0.8, 0.5));
    let selected_color = materials.add(Color::hsl(0., 0.8, 0.7));

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
    ) -> Handle<ColorMaterial> {
        return unit_resources.settler.color.clone();
    }

    fn get_selected_material(
        &self,
        unit_resources: &Res<super::UnitResources>,
    ) -> Handle<ColorMaterial> {
        return unit_resources.settler.selected_color.clone();
    }

    fn tile_action(
        &self,
        tile: Mut<tile::Tile>,
        unit_entity: Entity,
        tile_entity: Entity,
        acting_empire: i32,
    ) -> Vec<actions::Action> {
        match &tile.building {
            None => {
                let capital = building::Building::City(building::city::City::default());

                vec![
                    actions::Action::Build(actions::Build {
                        building: capital,
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
