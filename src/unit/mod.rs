use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;

use crate::{controls, utils};

pub mod settler;

#[derive(Component)]
pub struct Unit {
    pub kind: UnitKind,
    pub location: utils::Coordinates,
    pub target: Option<utils::Coordinates>,
    pub owner: Option<i32>,
}

pub enum UnitKind {
    Settler(settler::Settler),
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
}

pub fn create_resources<'a, 'b>(
    materials: ResMut<'a, Assets<ColorMaterial>>,
    meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    UnitResources,
    ResMut<'a, Assets<ColorMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let (settler, materials, meshes) = settler::init_resources(materials, meshes);

    (UnitResources { settler }, materials, meshes)
}

pub fn spawn<'a, 'b>(
    mut commands: Commands<'a, 'b>,
    unit_resources: &Res<UnitResources>,
    unit: Unit,
) -> Commands<'a, 'b> {
    let (x, y) = utils::to_world_location(&unit.location);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(unit_resources.settler.mesh.clone()),
            material: unit_resources.settler.color.clone(),
            transform: Transform::from_xyz(x, y, 1.),
            ..default()
        },
        unit,
        On::<Pointer<Click>>::send_event::<controls::SelectUnit>(),
    ));

    commands
}

trait UnitTrait {
    fn get_normal_material(&self, unit_resources: &Res<UnitResources>) -> Handle<ColorMaterial>;
    fn get_selected_material(&self, unit_resources: &Res<UnitResources>) -> Handle<ColorMaterial>;
}

pub fn get_selected_material(
    unit: &Unit,
    unit_resources: &Res<UnitResources>,
) -> Handle<ColorMaterial> {
    match &unit.kind {
        UnitKind::Settler(settler) => settler.get_selected_material(unit_resources),
    }
}

pub fn get_normal_material(
    unit: &Unit,
    unit_resources: &Res<UnitResources>,
) -> Handle<ColorMaterial> {
    match &unit.kind {
        UnitKind::Settler(settler) => settler.get_normal_material(unit_resources),
    }
}
