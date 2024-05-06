use bevy::{prelude::*, utils::HashMap};
use bevy_mod_picking::prelude::*;

use crate::{
    actions, controls,
    tile::{self, TILE_SIZE},
    utils, world_gen,
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
}

pub fn create_resources<'a, 'b>(
    materials: ResMut<'a, Assets<StandardMaterial>>,
    meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    UnitResources,
    ResMut<'a, Assets<StandardMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let (settler, materials, meshes) = settler::init_resources(materials, meshes);
    let (caravan, materials, meshes) = caravan::init_resources(materials, meshes);

    (UnitResources { settler, caravan }, materials, meshes)
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

    (
        MaterialMeshBundle {
            mesh: unit_resources.settler.mesh.clone(),
            material: unit_resources.settler.color.clone(),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        },
        unit,
        On::<Pointer<Click>>::send_event::<controls::SelectUnit>(),
    )
}

trait UnitTrait {
    fn get_normal_material(&self, unit_resources: &Res<UnitResources>) -> Handle<StandardMaterial>;
    fn get_selected_material(
        &self,
        unit_resources: &Res<UnitResources>,
    ) -> Handle<StandardMaterial>;
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
    match &unit.kind {
        UnitKind::Settler(settler) => settler.get_selected_material(unit_resources),
        UnitKind::Caravan(caravan) => caravan.get_selected_material(unit_resources),
    }
}

pub fn get_normal_material(
    unit: &Unit,
    unit_resources: &Res<UnitResources>,
) -> Handle<StandardMaterial> {
    match &unit.kind {
        UnitKind::Settler(settler) => settler.get_normal_material(unit_resources),
        UnitKind::Caravan(caravan) => caravan.get_normal_material(unit_resources),
    }
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
            println!("Unit has not moved this turn");
            let delta = utils::Coordinates {
                x: (target.x - unit.location.x).clamp(-1, 1),
                y: (target.y - unit.location.y).clamp(-1, 1),
            };

            println!("Delta: {:?}", delta);

            unit.location + delta
        }
        (_, true) => unit.location,
        (None, _) => unit.location,
    }
}
