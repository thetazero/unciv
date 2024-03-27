use bevy::prelude::*;
use crate::colors::dark_hue;

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}

#[derive(Resource)]
pub struct TileResources {
    pub dark_green: Handle<ColorMaterial>,
    pub empire_red: Handle<ColorMaterial>,
    pub square: Handle<Mesh>,
}

pub fn create_tile_resources(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> TileResources {
    let dark_green: Handle<ColorMaterial> = materials.add(dark_hue(0.4));
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let square: Handle<Mesh> = meshes.add(Rectangle::new(50.0, 50.0));

    TileResources {
        dark_green,
        empire_red,
        square,
    }
}
