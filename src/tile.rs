use bevy::prelude::*;

use crate::colors::dark_hue;

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
    Desert,
    Forest,
    Mountain,
    SnowyMountain,
    Water,
}

pub fn tile_material(kind: &TileKind, tile_resources: &TileResources) -> Handle<ColorMaterial> {
    match kind {
        TileKind::Desert => tile_resources.materials.desert.clone(),
        TileKind::Forest => tile_resources.materials.forest.clone(),
        TileKind::Mountain => tile_resources.materials.mountain.clone(),
        TileKind::SnowyMountain => tile_resources.materials.snowy_mountain.clone(),
        TileKind::Water => tile_resources.materials.water.clone(),
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub kind: TileKind,
    pub owner: Option<i32>,
}

struct TileMaterials {
    pub desert: Handle<ColorMaterial>,
    pub forest: Handle<ColorMaterial>,
    pub mountain: Handle<ColorMaterial>,
    pub snowy_mountain: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct TileResources {
    materials: TileMaterials,
    pub empire_red: Handle<ColorMaterial>,
    pub square: Handle<Mesh>,
}

pub const TILE_SIZE: f32 = 50.;

pub fn create_tile_resources(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> TileResources {
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let desert = materials.add(Color::hsl(47., 0.29, 0.49));
    let forest: Handle<ColorMaterial> = materials.add(dark_hue(0.4));
    let mountain = materials.add(Color::hsl(0.3, 0.1, 0.3));
    let snowy_mountain = materials.add(Color::hsl(0.3, 0.05, 0.8));
    let water = materials.add(Color::hsl(200.0, 0.3, 0.5));

    let square: Handle<Mesh> = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    TileResources {
        materials: TileMaterials {
            desert,
            forest,
            mountain,
            snowy_mountain,
            water,
        },
        empire_red,
        square,
    }
}

pub fn is_spawnable(kind: &TileKind) -> bool {
    match kind {
        TileKind::Forest | TileKind::Mountain => true,
        _ => false,
    }
}
