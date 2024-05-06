use bevy::prelude::*;

use super::UnitTrait;
use crate::{actions, tile};

#[derive(Clone, Debug)]
pub struct Caravan {}

impl Default for Caravan {
    fn default() -> Self {
        Caravan {}
    }
}

pub struct CaravanResources {
    pub mesh: Handle<Mesh>,
}

pub fn init_resources<'a, 'b>(
    materials: ResMut<'a, Assets<StandardMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    CaravanResources,
    ResMut<'a, Assets<StandardMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let cube_mesh = Cuboid::new(
        tile::TILE_SIZE as f32 / 5.0,
        tile::TILE_SIZE as f32 / 5.0,
        tile::TILE_SIZE as f32 / 2.0,
    );
    let mesh = meshes.add(cube_mesh);

    (CaravanResources { mesh }, materials, meshes)
}

impl UnitTrait for Caravan {
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
        _: Mut<tile::TileComponent>,
        _: Entity,
        _: Entity,
        _: i32,
    ) -> Vec<actions::Action> {
        vec![]
    }
}
