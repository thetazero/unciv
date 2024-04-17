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
        TileType::Forest => tile_resources.dark_green.clone(),
        TileType::Mountain => tile_resources.forest.clone(),
        TileType::Water => tile_resources.water.clone(),
    }
}

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub kind: TileType,
    pub neighbors: Vec<Entity>,
}

#[derive(Resource)]
pub struct TileResources {
    pub dark_green: Handle<ColorMaterial>,
    pub empire_red: Handle<ColorMaterial>,
    pub forest: Handle<ColorMaterial>,
    pub water: Handle<ColorMaterial>,
    pub square: Handle<Mesh>,
}

pub fn create_tile_resources(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> TileResources {
    let dark_green: Handle<ColorMaterial> = materials.add(dark_hue(0.4));
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let forest = materials.add(Color::hsl(0.3, 0.5, 0.3));
    let water = materials.add(Color::hsl(200.0, 0.3, 0.5));

    let square: Handle<Mesh> = meshes.add(Rectangle::new(50.0, 50.0));

    TileResources {
        dark_green,
        empire_red,
        forest,
        water,
        square,
    }
}

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

const X_EXTENT: f32 = 600.;

pub fn spawn(mut commands: Commands, tile_resources: Res<TileResources>) {
    let x_count = 10;
    let y_count = 10;

    for x in 0..x_count {
        for y in 0..y_count {
            let kind: TileType = rand::random();

            let tile_bundle = (
                Tile {
                    x,
                    y,
                    kind: kind.clone(),
                    neighbors: vec![],
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(tile_resources.square.clone()),
                    material: tile_material(&kind, &tile_resources),
                    transform: Transform::from_xyz(
                        distribute(x, x_count, X_EXTENT),
                        distribute(y, x_count, X_EXTENT),
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
