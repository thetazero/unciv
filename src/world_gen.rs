use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_mod_picking::prelude::*;
use core::iter::zip;
use noise::{NoiseFn, Simplex};
use rand::seq::IteratorRandom;
use std::collections::HashMap;

use crate::config::CONFIG;
use crate::{colors, empire, tile, ui};

fn compute_tile_kind(height: f64, biome: f64) -> tile::TileKind {
    if height < -0.1 {
        return tile::TileKind::Water;
    } else if height < 0.5 {
        if biome < 0. {
            return tile::TileKind::Desert;
        } else {
            return tile::TileKind::Forest;
        }
    } else {
        return tile::TileKind::Mountain;
    }
}

fn scaled_simplex_2d(simplex: Simplex, x: f64, y: f64, scale: f64) -> f64 {
    simplex.get([x * scale, y * scale])
}

pub fn spawn_tile_data(x_count: i32, y_count: i32) -> Vec<tile::Tile> {
    let simplex_2d = Simplex::new(2);

    let mut tiles = Vec::new();
    for x in 0..x_count {
        for y in 0..y_count {
            let x_float = x as f64;
            let y_float = y as f64;

            let height = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.15);
            let biome = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.03);

            let kind: tile::TileKind = compute_tile_kind(height, biome);
            tiles.push(tile::Tile {
                x,
                y,
                kind: kind.clone(),
                owner: None,
            });
        }
    }
    tiles
}

#[derive(Resource)]
pub struct WorldState {
    pub tiles: HashMap<(i32, i32), Entity>,
}

pub fn spawn(mut commands: Commands, tile_resources: Res<tile::TileResources>) {
    let mut world_state = WorldState {
        tiles: HashMap::new(),
    };

    let (x_count, y_count) = CONFIG.world_size;

    let tile_data = spawn_tile_data(x_count, y_count);

    for tile in tile_data.iter() {
        let tile_bundle = (
            tile.clone(),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(tile_resources.square.clone()),
                material: tile::tile_material(&tile.kind, &tile_resources),
                transform: Transform::from_xyz(
                    tile.x as f32 * (tile::TILE_SIZE + 1.),
                    tile.y as f32 * (tile::TILE_SIZE + 1.),
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
        let tile_entity = commands.spawn(tile_bundle);
        world_state.tiles.insert((tile.x, tile.y), tile_entity.id());
    }

    commands.insert_resource(world_state);
}

pub fn spawn_empires(
    mut commands: Commands,
    mut query: Query<(Entity, &mut tile::Tile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const NUMBER_OF_EMPIRES: i64 = 10;

    let mut empire_list = vec![];

    for i in 0..NUMBER_OF_EMPIRES {
        let color = materials.add(colors::bright_hue(i as f32 / NUMBER_OF_EMPIRES as f32));
        let empire = commands
            .spawn((
                empire::Empire {
                    id: i as i32,
                    color: color.clone(),
                    inventory: empire::Inventory { wood: 0, stone: 0 },
                },
                TransformBundle::default(),
                InheritedVisibility::default(),
            ))
            .id();

        empire_list.push((empire, color));
    }

    let mut rng = rand::thread_rng();
    let spawn_tiles = query
        .iter_mut()
        .choose_multiple(&mut rng, NUMBER_OF_EMPIRES as usize);

    for ((entity, mut tile), (empire, color)) in zip(spawn_tiles, empire_list) {
        commands.entity(empire).push_children(&[entity]);
        commands.entity(entity).insert(color.clone());

        tile.owner = Some(empire);
    }
}
