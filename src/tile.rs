use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use crate::colors::dark_hue;
use crate::ui;

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TileType {
    Forest,
    Mountain,
    Water,
}

impl Distribution<TileType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        match rng.gen_range(0..3) {
            0 => TileType::Forest,
            1 => TileType::Mountain,
            _ => TileType::Water,
        }
    }
}

fn tile_material(kind: &TileType, tile_resources: &TileResources) -> Handle<ColorMaterial> {
    match kind {
        TileType::Forest => tile_resources.forest.clone(),
        TileType::Mountain => tile_resources.mountain.clone(),
        TileType::Water => tile_resources.water.clone(),
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub kind: TileType,
    pub neighbors: Vec<Entity>,
}

#[derive(Resource)]
pub struct TileResources {
    pub forest: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
    pub mountain: Handle<ColorMaterial>,
    pub empire_red: Handle<ColorMaterial>,
    pub square: Handle<Mesh>,
}

pub fn create_tile_resources(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> TileResources {
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let forest: Handle<ColorMaterial> = materials.add(dark_hue(0.4));
    let mountain = materials.add(Color::hsl(0.3, 0.1, 0.3));
    let water = materials.add(Color::hsl(200.0, 0.3, 0.5));

    let square: Handle<Mesh> = meshes.add(Rectangle::new(50.0, 50.0));

    TileResources {
        forest,
        water,
        mountain,
        empire_red,
        square,
    }
}

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

const X_EXTENT: f32 = 600.;

fn spawn_tile_data(x_count: i32, y_count: i32) -> Vec<Tile> {
    let mut tiles = Vec::new();
    for x in 0..x_count {
        for y in 0..y_count {
            let kind: TileType = rand::random();
            tiles.push(Tile {
                x,
                y,
                kind: kind.clone(),
                neighbors: vec![],
            });
        }
    }
    tiles
}

pub fn spawn(mut commands: Commands, tile_resources: Res<TileResources>) {
    let x_count = 10;
    let y_count = 10;

    let tile_data = spawn_tile_data(x_count, y_count);

    for tile in tile_data.iter() {
        let tile_bundle = (
            tile.clone(),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(tile_resources.square.clone()),
                material: tile_material(&tile.kind, &tile_resources),
                transform: Transform::from_xyz(
                    distribute(tile.x, x_count, X_EXTENT),
                    distribute(tile.y, x_count, X_EXTENT),
                    0.0,
                ),
                ..default()
            },
            PickableBundle::default(),
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x; // Make the square follow the mouse
                transform.translation.y -= drag.delta.y;
            }),
            On::<Pointer<Click>>::send_event::<ui::InspectEvent>(),
        );
        commands.spawn(tile_bundle);
    }
}

pub fn link(mut query: Query<(Entity, &mut Tile)>) {
    let mut tile_ids: HashMap<(i32, i32), Entity> = HashMap::new();
    for (entity, tile) in query.iter_mut() {
        tile_ids.insert((tile.x, tile.y), entity);
    }

    for (_, mut tile) in query.iter_mut() {
        let neighbors = [
            (tile.x - 1, tile.y),
            (tile.x + 1, tile.y),
            (tile.x, tile.y - 1),
            (tile.x, tile.y + 1),
        ];

        for (x, y) in neighbors.iter() {
            if let Some(neighbor) = tile_ids.get(&(*x, *y)) {
                tile.neighbors.push(*neighbor);
            }
        }
    }
}
