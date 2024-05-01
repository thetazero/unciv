use bevy::prelude::*;

use crate::{resource, tile};

use super::BuildingTrait;

#[derive(Clone)]
pub struct Capital {
    level: i32,
}

impl Default for Capital {
    fn default() -> Self {
        Capital { level: 1 }
    }
}

impl BuildingTrait for Capital {
    fn production(&self) -> Vec<(resource::Resource, i32)> {
        vec![
            (resource::Resource::Wood, 1 * self.level),
            (resource::Resource::Stone, 1 * self.level),
        ]
    }

    fn name(&self) -> String {
        format!("Capital {}", self.level)
    }

    fn get_material(
        &self,
        building_resources: &Res<super::BuildingResources>,
    ) -> Handle<ColorMaterial> {
        building_resources.capital_material.clone()
    }

    fn get_mesh(
        &self,
        building_resources: &Res<super::BuildingResources>,
    ) -> bevy::prelude::Handle<bevy::prelude::Mesh> {
        building_resources.capital_mesh.clone()
    }

    fn load_mesh() -> Mesh {
        Rectangle::new(tile::TILE_SIZE as f32 / 1.5, tile::TILE_SIZE as f32 / 1.5).into()
    }

    fn load_material() -> ColorMaterial {
        ColorMaterial {
            color: Color::hsl(0.0, 0.0, 0.5),
            texture: None,
        }
    }
}
