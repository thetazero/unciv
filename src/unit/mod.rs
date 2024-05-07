use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::prelude::*;

use crate::{
    actions, animation, colors, controls, empire,
    tile::{self, TILE_SIZE},
    unit, utils, world_gen,
};

pub mod caravan;
pub mod settler;

#[derive(Component, Clone, Debug)]
pub struct Unit {
    pub kind: UnitKind,
    pub location: utils::Coordinates,
    pub target: Option<utils::Coordinates>,
    pub owner: Option<i32>,
    pub moved: bool,
}

#[derive(Clone, Debug)]
pub enum UnitKind {
    Settler(settler::Settler),
    Caravan(caravan::Caravan),
}

impl Default for Unit {
    fn default() -> Self {
        Unit {
            kind: UnitKind::Settler(settler::Settler::default()),
            location: utils::Coordinates { x: 0, y: 0 },
            target: None,
            owner: None,
            moved: false,
        }
    }
}

#[derive(Resource)]
pub struct UnitResources {
    pub settler: settler::SettlerResources,
    pub caravan: caravan::CaravanResources,
    pub default_materials: Vec<Handle<StandardMaterial>>,
    pub selected_materials: Vec<Handle<StandardMaterial>>,
}

pub fn create_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<StandardMaterial>>,
    meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    UnitResources,
    ResMut<'a, Assets<StandardMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let (settler, meshes) = settler::init_resources(meshes);
    let (caravan, meshes) = caravan::init_resources(meshes);

    let mut default_materials = Vec::new();
    let mut selected_materials = Vec::new();

    for i in 0..10 {
        let empire_hue = empire::id_to_hue(i as i32);
        let default = materials.add(colors::plastic_material(empire_hue, 0.5, 0.5));
        let selected = materials.add(colors::plastic_material(empire_hue, 0.5, 0.3));

        default_materials.push(default);
        selected_materials.push(selected);
    }

    (
        UnitResources {
            settler,
            caravan,
            default_materials,
            selected_materials,
        },
        materials,
        meshes,
    )
}

pub fn make_bundle(
    unit: Unit,
    unit_resources: &Res<UnitResources>,
    tile_data: &HashMap<utils::Coordinates, tile::Tile>,
) -> (
    MaterialMeshBundle<StandardMaterial>,
    Unit,
    On<Pointer<Click>>,
) {
    let (x, y) = utils::to_world_location(&unit.location);

    let z = tile_data.get(&unit.location).unwrap().height + TILE_SIZE as f32 / 2.;

    let default_material = get_normal_material(&unit, unit_resources);

    (
        MaterialMeshBundle {
            mesh: unit_resources.settler.mesh.clone(),
            material: default_material,
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        unit,
        On::<Pointer<Click>>::send_event::<controls::SelectUnit>(),
    )
}

trait UnitTrait {
    fn tile_action(
        &self,
        tile: Mut<tile::TileComponent>,
        unit_entity: Entity,
        tile_entity: Entity,
        acting_empire: i32,
    ) -> Vec<actions::Action>;
}

pub fn get_selected_material(
    unit: &Unit,
    unit_resources: &Res<UnitResources>,
) -> Handle<StandardMaterial> {
    let owner = unit.owner.unwrap_or(0);

    unit_resources
        .selected_materials
        .get(owner as usize)
        .unwrap()
        .clone()
}

pub fn get_normal_material(
    unit: &Unit,
    unit_resources: &Res<UnitResources>,
) -> Handle<StandardMaterial> {
    let owner = unit.owner.unwrap_or(0);

    unit_resources
        .default_materials
        .get(owner as usize)
        .unwrap()
        .clone()
}

pub fn tile_action(
    unit: &Unit,
    tile: Mut<tile::TileComponent>,
    unit_entity: Entity,
    tile_entity: Entity,
    acting_empire: i32,
) -> Vec<actions::Action> {
    match &unit.kind {
        UnitKind::Settler(settler) => {
            settler.tile_action(tile, unit_entity, tile_entity, acting_empire)
        }
        UnitKind::Caravan(caravan) => {
            caravan.tile_action(tile, unit_entity, tile_entity, acting_empire)
        }
    }
}

pub fn unit_height(
    world_state: &Res<world_gen::WorldState>,
    coordinates: &utils::Coordinates,
) -> f32 {
    return world_state.tile_data.get(coordinates).unwrap().height + TILE_SIZE as f32 / 2.;
}

// TODO: Do not walk on water
pub fn next_location(unit: &Unit, _world_state: &Res<world_gen::WorldState>) -> utils::Coordinates {
    match (unit.target, unit.moved) {
        (Some(target), false) => {
            let delta = utils::Coordinates {
                x: (target.x - unit.location.x).clamp(-1, 1),
                y: (target.y - unit.location.y).clamp(-1, 1),
            };
            unit.location + delta
        }
        (_, true) => unit.location,
        (None, _) => unit.location,
    }
}

pub fn next_location_update<'a, 'b, 'c>(
    mut commands: Commands<'b, 'c>,
    mut unit: Mut<'a, Unit>,
    time: &Res<Time>,
    transform: &Transform,
    unit_entity: &Entity,
    world_state: &Res<world_gen::WorldState>,
    next_location: &utils::Coordinates,
) -> (Commands<'b, 'c>, Mut<'a, Unit>) {
    let (x, y) = utils::to_world_location(&next_location);
    let z = unit::unit_height(&world_state, &next_location);

    unit.location = *next_location;

    commands
        .entity(*unit_entity)
        .insert(animation::TranslationAnimation {
            start: transform.translation,
            end: Vec3::new(x, y, z),
            start_time: time.elapsed_seconds(),
            duration: 0.5,
        });

    (commands, unit)
}
