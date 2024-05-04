use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{
    actions, controls,
    tile::{self, TILE_SIZE},
    utils,
};

pub mod caravan;
pub mod settler;

#[derive(Component, Clone, Debug)]
pub struct Unit {
    pub kind: UnitKind,
    pub location: utils::Coordinates,
    pub target: Option<utils::Coordinates>,
    pub owner: Option<i32>,
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
) -> (
    MaterialMeshBundle<StandardMaterial>,
    Unit,
    On<Pointer<Click>>,
) {
    let (x, y) = utils::to_world_location(&unit.location);

    (
        MaterialMeshBundle {
            mesh: unit_resources.settler.mesh.clone(),
            material: unit_resources.settler.color.clone(),
            transform: Transform::from_xyz(x, y, TILE_SIZE as f32 * 2.),
            ..default()
        },
        unit,
        On::<Pointer<Click>>::send_event::<controls::SelectUnit>(),
    )
}

pub fn spawn<'a, 'b>(
    mut commands: Commands<'a, 'b>,
    unit_resources: &Res<UnitResources>,
    unit: Unit,
) -> Commands<'a, 'b> {
    let (x, y) = utils::to_world_location(&unit.location);

    commands.spawn((
        MaterialMeshBundle {
            mesh: unit_resources.settler.mesh.clone(),
            material: unit_resources.settler.color.clone(),
            transform: Transform::from_xyz(x, y, TILE_SIZE as f32 * 1.),
            ..default()
        },
        unit,
        On::<Pointer<Click>>::send_event::<controls::SelectUnit>(),
    ));

    commands
}

trait UnitTrait {
    fn get_normal_material(&self, unit_resources: &Res<UnitResources>) -> Handle<StandardMaterial>;
    fn get_selected_material(
        &self,
        unit_resources: &Res<UnitResources>,
    ) -> Handle<StandardMaterial>;
    fn tile_action(
        &self,
        tile: Mut<tile::Tile>,
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
    tile: Mut<tile::Tile>,
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
