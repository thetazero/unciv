use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_mod_picking::prelude::*;
use noise::{NoiseFn, Simplex};
use rand::seq::IteratorRandom;
use std::collections::HashMap;

use crate::config::CONFIG;
use crate::{building, colors, controls, empire, tile, ui, unit};

fn compute_tile_kind(height: f64, biome: f64) -> tile::TileKind {
    if height < -0.1 {
        return tile::TileKind::Water;
    } else if height < 0.5 {
        if biome < 0. {
            return tile::TileKind::Desert;
        } else {
            return tile::TileKind::Forest;
        }
    } else if height < 0.7 {
        return tile::TileKind::Mountain;
    } else {
        return tile::TileKind::SnowyMountain;
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

            let height = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.05);
            let biome = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.02);

            let kind: tile::TileKind = compute_tile_kind(height, biome);
            tiles.push(tile::Tile {
                x,
                y,
                kind: kind.clone(),
                owner: None,
                buildings: vec![],
            });
        }
    }
    tiles
}

fn add_empire_data(tile_data: &mut Vec<tile::Tile>, number_of_empires: i32) {
    let mut spawned_empires = 0;
    let mut rng = rand::thread_rng();

    let mut max_attempts = 1000;
    while spawned_empires < number_of_empires && max_attempts > 0 {
        let chosen_tile = tile_data.iter_mut().choose(&mut rng).unwrap();

        if tile::is_spawnable(&chosen_tile.kind) && chosen_tile.owner.is_none() {
            chosen_tile.owner = Some(spawned_empires);
            chosen_tile.buildings.push(building::Building::Capital(
                building::capital::Capital::default(),
            ));
            spawned_empires += 1;
        }

        max_attempts -= 1;
    }
    if max_attempts == 0 {
        panic!("Could not place all empires");
    }
}

#[derive(Resource)]
pub struct WorldState {
    pub tiles: HashMap<(i32, i32), Entity>,
    pub empires: HashMap<i32, Entity>,
}

pub fn spawn(
    mut commands: Commands,
    tile_resources: Res<tile::TileResources>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera: Query<&mut Transform, With<Camera2d>>,
    unit_resources: Res<unit::UnitResources>,
) {
    let mut world_state = WorldState {
        tiles: HashMap::new(),
        empires: HashMap::new(),
    };
    const NUMBER_OF_EMPIRES: i32 = 10;

    let (x_count, y_count) = CONFIG.world_size;

    let mut tile_data = spawn_tile_data(x_count, y_count);
    add_empire_data(&mut tile_data, NUMBER_OF_EMPIRES);

    let mut color_list = vec![];

    for i in 0..NUMBER_OF_EMPIRES {
        let color = materials.add(colors::bright_hue(i as f32 / NUMBER_OF_EMPIRES as f32));
        let empire_entity = commands
            .spawn((
                empire::Empire {
                    id: i as i32,
                    color: color.clone(),
                    inventory: empire::Inventory {
                        inv: HashMap::new(),
                    },
                },
                TransformBundle::default(),
                InheritedVisibility::default(),
            ))
            .id();

        world_state.empires.insert(i, empire_entity);

        color_list.push(color);
    }

    let mut camera_spawn_point = None;

    for tile in tile_data.iter() {
        let material = match tile.owner {
            Some(empire_id) => color_list.get(empire_id as usize).unwrap().clone(),
            None => tile::tile_material(&tile.kind, &tile_resources),
        };

        let tile_location = Transform::from_xyz(
            tile.x as f32 * (tile::TILE_SIZE + 1.),
            tile.y as f32 * (tile::TILE_SIZE + 1.),
            0.0,
        );

        if tile.owner == Some(0) {
            camera_spawn_point = Some(tile_location);
        }

        let tile_bundle = (
            tile.clone(),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(tile_resources.square.clone()),
                material,
                transform: tile_location,
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

    if let Some(camera_spawn_point) = camera_spawn_point {
        controls::move_camera_to(camera, camera_spawn_point);
    }

    let _commands = unit::spawn(
        commands,
        unit_resources,
        unit::Unit {
            kind: unit::UnitKind::Settler(unit::settler::Settler::default()),
            x: 0,
            y: 0,
            owner: Some(0),
        },
    );
}
