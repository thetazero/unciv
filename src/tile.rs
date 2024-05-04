use bevy::{prelude::*, utils::HashMap};

use bevy_mod_picking::prelude::*;

use crate::{building, colors, controls, utils};

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
    Desert,
    Forest,
    Mountain,
    SnowyMountain,
    Shallows,
    Ocean,
    Beach,
}

pub fn tile_material(kind: &TileKind, tile_resources: &TileResources) -> Handle<StandardMaterial> {
    match kind {
        TileKind::Desert => tile_resources.materials.desert.clone(),
        TileKind::Forest => tile_resources.materials.forest.clone(),
        TileKind::Mountain => tile_resources.materials.mountain.clone(),
        TileKind::SnowyMountain => tile_resources.materials.snowy_mountain.clone(),
        TileKind::Shallows => tile_resources.materials.water.clone(),
        TileKind::Ocean => tile_resources.materials.ocean.clone(),
        TileKind::Beach => tile_resources.materials.beach.clone(),
    }
}

pub fn tile_string(kind: &TileKind) -> String {
    match kind {
        TileKind::Desert => "Desert".to_string(),
        TileKind::Forest => "Forest".to_string(),
        TileKind::Mountain => "Mountain".to_string(),
        TileKind::SnowyMountain => "Snowy Mountain".to_string(),
        TileKind::Shallows => "Water".to_string(),
        TileKind::Ocean => "Ocean".to_string(),
        TileKind::Beach => "Beach".to_string(),
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub location: utils::Coordinates,
    pub height: f32,
    pub kind: TileKind,
    pub owner: Option<i32>,
    pub building: Option<building::Building>,
}

struct TileMaterials {
    pub desert: Handle<StandardMaterial>,
    pub forest: Handle<StandardMaterial>,
    pub mountain: Handle<StandardMaterial>,
    pub snowy_mountain: Handle<StandardMaterial>,
    pub water: Handle<StandardMaterial>,
    pub ocean: Handle<StandardMaterial>,
    pub beach: Handle<StandardMaterial>,
    pub empire_colors: HashMap<i32, Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct TileResources {
    materials: TileMaterials,
    pub square: Handle<Mesh>,
}

pub const TILE_SIZE: f32 = 1.;

pub fn create_tile_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<StandardMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    TileResources,
    ResMut<'a, Assets<StandardMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let desert = materials.add(colors::plastic_material(47., 0.39, 0.39));
    let forest = materials.add(colors::plastic_material(0.4 * 360., 0.95, 0.2));
    let mountain = materials.add(colors::plastic_material(0.3, 0.1, 0.3));
    let snowy_mountain = materials.add(colors::plastic_material(0.3, 0.05, 0.8));
    let water = materials.add(colors::plastic_material(200.0, 0.3, 0.5));
    let ocean = materials.add(colors::plastic_material(200.0, 0.4, 0.3));
    let beach = materials.add(colors::plastic_material(47., 0.29, 0.49));

    let square: Handle<Mesh> = meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE));

    let mut empire_colors = HashMap::default();

    for i in 0..10 {
        empire_colors.insert(
            i,
            materials.add(colors::plastic_material(i as f32 * 36.0, 1.0, 0.3)),
        );
    }

    (
        TileResources {
            materials: TileMaterials {
                desert,
                forest,
                mountain,
                snowy_mountain,
                water,
                empire_colors,
                ocean,
                beach,
            },
            square,
        },
        materials,
        meshes,
    )
}

pub fn is_spawnable(kind: &TileKind) -> bool {
    match kind {
        TileKind::Forest | TileKind::Mountain => true,
        _ => false,
    }
}

pub fn is_land(kind: &TileKind) -> bool {
    match kind {
        TileKind::Ocean | TileKind::Shallows => false,
        _ => true,
    }
}

pub fn make_bundle(
    tile_resources: &Res<TileResources>,
    tile: &Tile,
) -> (
    Tile,
    MaterialMeshBundle<StandardMaterial>,
    PickableBundle,
    On<Pointer<Drag>>,
    On<Pointer<Click>>,
) {
    let material = match tile.owner {
        Some(empire_id) => tile_resources
            .materials
            .empire_colors
            .get(&(empire_id % 10))
            .unwrap()
            .clone(),
        None => tile_material(&tile.kind, &tile_resources),
    };

    let mut tile_location = utils::to_transform(&tile.location);

    tile_location.translation.z = tile.height;

    (
        tile.clone(),
        MaterialMeshBundle {
            mesh: tile_resources.square.clone(),
            material,
            transform: tile_location,
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Drag>>::send_event::<controls::DragEvent>(),
        On::<Pointer<Click>>::send_event::<controls::InspectTileEvent>(),
    )
}
