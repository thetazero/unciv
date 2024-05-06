use bevy::prelude::*;

use super::BuildingTrait;
use crate::{resource, tile};

#[derive(Clone, Debug)]
pub struct House {}

impl Default for House {
    fn default() -> Self {
        House {}
    }
}

impl BuildingTrait for House {
    fn production(&self) -> Vec<(resource::Resource, i32)> {
        vec![
            (resource::Resource::Wood, 1),
            (resource::Resource::Stone, 1),
        ]
    }

    fn name(&self) -> String {
        "House".to_string()
    }

    fn get_material(
        &self,
        building_resources: &Res<super::BuildingResources>,
    ) -> Handle<StandardMaterial> {
        building_resources.city_material.clone()
    }

    fn get_mesh(&self, building_resources: &Res<super::BuildingResources>) -> Handle<Scene> {
        building_resources.city_mesh.clone()
    }

    fn load_material() -> StandardMaterial {
        Color::hsl(30.0, 0.3, 0.5).into()
    }

    fn load_mesh() -> Mesh {
        Cuboid::new(
            tile::TILE_SIZE as f32 / 1.5,
            tile::TILE_SIZE as f32 / 1.5,
            tile::TILE_SIZE as f32 / 1.5,
        )
        .into()
    }
}
