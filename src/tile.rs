use bevy::{
    prelude::*,
};

use crate::colors::dark_hue;

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
    Forest,
    Desert,
    Mountain,
    Water,
}

pub fn tile_material(kind: &TileKind, tile_resources: &TileResources) -> Handle<ColorMaterial> {
    match kind {
        TileKind::Forest => tile_resources.forest.clone(),
        TileKind::Mountain => tile_resources.mountain.clone(),
        TileKind::Water => tile_resources.water.clone(),
        TileKind::Desert => tile_resources.desert.clone(),
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub kind: TileKind,
    pub owner: Option<i32>,
}

#[derive(Resource)]
pub struct TileResources {
    pub forest: Handle<ColorMaterial>,
    pub desert: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
    pub mountain: Handle<ColorMaterial>,
    pub empire_red: Handle<ColorMaterial>,
    pub square: Handle<Mesh>,
}

pub const TILE_SIZE: f32 = 50.;

pub fn create_tile_resources(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> TileResources {
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let forest: Handle<ColorMaterial> = materials.add(dark_hue(0.4));
    let desert = materials.add(Color::hsl(47., 0.29, 0.49));
    let mountain = materials.add(Color::hsl(0.3, 0.1, 0.3));
    let water = materials.add(Color::hsl(200.0, 0.3, 0.5));

    let square: Handle<Mesh> = meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE));

    TileResources {
        forest,
        desert,
        water,
        mountain,
        empire_red,
        square,
    }
}

fn is_spawnable(kind: &TileKind) -> bool {
    match kind {
        TileKind::Forest | TileKind::Mountain => true,
        _ => false,
    }
}
