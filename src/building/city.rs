use bevy::prelude::*;

use super::BuildingTrait;
use crate::{resource, tile};

#[derive(Clone)]
pub struct City {}

impl Default for City {
    fn default() -> Self {
        City {}
    }
}

impl BuildingTrait for City {
    fn production(&self) -> Vec<(resource::Resource, i32)> {
        vec![
            (resource::Resource::Wood, 1),
            (resource::Resource::Stone, 1),
        ]
    }

    fn name(&self) -> String {
        "City".to_string()
    }

    fn get_material(
        &self,
        building_resources: Res<super::BuildingResources>,
    ) -> Handle<ColorMaterial> {
        building_resources.city_material.clone()
    }

    fn get_mesh(&self, building_resources: Res<super::BuildingResources>) -> Handle<Mesh> {
        building_resources.city_mesh.clone()
    }

    fn load_material() -> ColorMaterial {
        ColorMaterial {
            color: Color::hsl(0.0, 0.0, 0.5),
            texture: None,
        }
    }

    fn load_mesh() -> Mesh {
        Rectangle::new(tile::TILE_SIZE, tile::TILE_SIZE).into()
    }
}
