use bevy::prelude::*;
use bevy::utils::HashMap;
use noise::{NoiseFn, Simplex};
use rand::seq::IteratorRandom;

use crate::config::CONFIG;
use crate::tile::TILE_SIZE;
use crate::{building, colors, controls, empire, tile, unit, utils};

const WATER_LEVEL: f32 = 0.2;

fn compute_tile_kind(height: f64, biome: f64) -> tile::TileKind {
    if height < -0.2 {
        return tile::TileKind::Ocean;
    } else if height <= WATER_LEVEL as f64 {
        return tile::TileKind::Shallows;
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

pub fn spawn_tile_data(x_count: i32, y_count: i32) -> Vec<tile::TileComponent> {
    let simplex_2d = Simplex::new(2);

    let mut tiles = HashMap::new();
    for x in 0..x_count {
        for y in 0..y_count {
            let x_float = x as f64;
            let y_float = y as f64;

            let height = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.05 / 2.);
            let biome = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.02 / 2.);

            let kind: tile::TileKind = compute_tile_kind(height, biome);

            let tile = tile::TileComponent {
                tile: tile::Tile {
                    location: utils::Coordinates { x, y },
                    height: (height as f32 * TILE_SIZE * 2.).max(WATER_LEVEL * TILE_SIZE * 2.),
                    kind: kind.clone(),
                },
                owner: None,
                building: None,
            };

            tiles.insert(tile.tile.location, tile);
        }
    }

    let keys = tiles.keys().cloned().collect::<Vec<utils::Coordinates>>();

    for loc in keys.iter() {
        let tile = tiles.get(loc).unwrap();

        if tile::is_land(&tile.tile.kind) {
            for (x, y) in utils::DIRECTIONS.iter() {
                if *x == 0 && *y == 0 {
                    continue;
                }

                let neighbor_loc = utils::Coordinates {
                    x: loc.x + x,
                    y: loc.y + y,
                };

                if let Some(neighbor) = tiles.get(&neighbor_loc) {
                    if !tile::is_land(&neighbor.tile.kind) {
                        let tile_mut = tiles.get_mut(loc).unwrap();
                        tile_mut.tile.kind = tile::TileKind::Beach;
                    }
                }
            }
        }
    }

    tiles.into_iter().map(|(_, tile)| tile).collect()
}

fn add_empire_data(tile_data: &mut Vec<tile::TileComponent>, number_of_empires: i32) {
    let mut spawned_empires = 0;
    let mut rng = rand::thread_rng();

    let mut max_attempts = 1000;
    while spawned_empires < number_of_empires && max_attempts > 0 {
        let chosen_tile = tile_data.iter_mut().choose(&mut rng).unwrap();

        if tile::is_spawnable(&chosen_tile.tile.kind) && chosen_tile.owner.is_none() {
            chosen_tile.owner = Some(spawned_empires);
            chosen_tile.building = Some(building::Building::Capital(default()));
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
    pub tile_entities: HashMap<utils::Coordinates, Entity>,
    pub tile_data: HashMap<utils::Coordinates, tile::Tile>,
    pub empires: HashMap<i32, Entity>,
}

pub fn spawn(
    mut commands: Commands,
    tile_resources: Res<tile::TileResources>,
    building_resources: Res<building::BuildingResources>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera: Query<&mut Transform, With<Camera3d>>,
    unit_resources: Res<unit::UnitResources>,
) {
    let mut world_state = WorldState {
        tile_entities: HashMap::new(),
        tile_data: HashMap::new(),
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
                    inventory: utils::Inventory {
                        items: HashMap::new(),
                        capacity: 100,
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
        world_state
            .tile_data
            .insert(tile.tile.location, tile.tile.clone());

        if let Some(empire_id) = tile.owner {
            let unit_bundle = unit::make_bundle(
                unit::Unit {
                    location: tile.tile.location,
                    owner: Some(empire_id),
                    ..Default::default()
                },
                &unit_resources,
                &world_state.tile_data,
            );
            commands.spawn(unit_bundle);
            if empire_id == 0 {
                camera_spawn_point = Some(utils::to_transform(&tile.tile.location));
            }
        }

        let tile_bundle = tile::make_bundle(&tile_resources, tile);
        let tile_entity = commands.spawn(tile_bundle);
        let tile_id = tile_entity.id();
        world_state
            .tile_entities
            .insert(tile.tile.location, tile_id);

        if let Some(building) = &tile.building {
            let building_bundle = building::make_bundle(building, &building_resources);
            let building_id = commands.spawn(building_bundle).id();
            commands.entity(tile_id).push_children(&[building_id]);
        }
    }

    commands.insert_resource(world_state);

    if let Some(camera_spawn_point) = camera_spawn_point {
        controls::move_camera_to(camera, camera_spawn_point);
    }
}
