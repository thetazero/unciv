use bevy::{prelude::*, utils::HashMap};

use bevy_mod_picking::prelude::*;

use crate::{building, colors::dark_hue, controls, utils};

#[derive(Clone, Copy, PartialEq)]
pub enum TileKind {
    Desert,
    Forest,
    Mountain,
    SnowyMountain,
    Water,
}

pub fn tile_material(kind: &TileKind, tile_resources: &TileResources) -> Handle<StandardMaterial> {
    match kind {
        TileKind::Desert => tile_resources.materials.desert.clone(),
        TileKind::Forest => tile_resources.materials.forest.clone(),
        TileKind::Mountain => tile_resources.materials.mountain.clone(),
        TileKind::SnowyMountain => tile_resources.materials.snowy_mountain.clone(),
        TileKind::Water => tile_resources.materials.water.clone(),
    }
}

pub fn tile_string(kind: &TileKind) -> String {
    match kind {
        TileKind::Desert => "Desert".to_string(),
        TileKind::Forest => "Forest".to_string(),
        TileKind::Mountain => "Mountain".to_string(),
        TileKind::SnowyMountain => "Snowy Mountain".to_string(),
        TileKind::Water => "Water".to_string(),
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
    pub empire_colors: HashMap<i32, Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct TileResources {
    materials: TileMaterials,
    pub empire_red: Handle<StandardMaterial>,
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
    let empire_red = materials.add(Color::hsl(0.0, 1.0, 0.5));
    let desert = materials.add(Color::hsl(47., 0.29, 0.49));
    let forest = materials.add(dark_hue(0.4));
    let mountain = materials.add(Color::hsl(0.3, 0.1, 0.3));
    let snowy_mountain = materials.add(Color::hsl(0.3, 0.05, 0.8));
    let water = materials.add(Color::hsl(200.0, 0.3, 0.5));

    let square: Handle<Mesh> = meshes.add(Cuboid::new(TILE_SIZE, TILE_SIZE, TILE_SIZE));

    let mut empire_colors = HashMap::default();

    for i in 0..10 {
        empire_colors.insert(i, materials.add(Color::hsl(i as f32 * 36.0, 0.5, 0.5)));
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
            },
            empire_red,
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

pub fn make_bundle(
    tile_resources: &Res<TileResources>,
    tile: &Tile,
) -> (
    Tile,
    MaterialMeshBundle<StandardMaterial>,
    PickableBundle,
    // On<Pointer<Drag>>,
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
        // On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //     transform.translation.x += drag.delta.x; // Make the square follow the mouse
        //     transform.translation.y -= drag.delta.y;
        // }),
        On::<Pointer<Click>>::send_event::<controls::InspectTileEvent>(),
    )
}
