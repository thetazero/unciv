use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use bevy_mod_picking::prelude::*;

use crate::colors::dark_hue;

use std::collections::HashMap;

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub neighbors: Vec<Entity>,
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

fn distribute(i: i32, count: i32, extent: f32) -> f32 {
    -extent / 2. + i as f32 / (count - 1) as f32 * extent
}

const X_EXTENT: f32 = 600.;

pub fn spawn(mut commands: Commands, tile_resources: Res<TileResources>) {
    let x_count = 10;
    let y_count = 10;

    for x in 0..x_count {
        for y in 0..y_count {
            let tile_bundle = (
                Tile {
                    x,
                    y,
                    neighbors: vec![],
                },
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(tile_resources.square.clone()),
                    material: tile_resources.dark_green.clone(),
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
