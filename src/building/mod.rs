use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::{resource, tile::TILE_SIZE};

pub mod capital;
pub mod house;

#[derive(Clone, Debug)]
pub struct Building {
    pub kind: BuildingKind,
}

#[derive(Clone, Debug)]
pub enum BuildingKind {
    Capital(capital::Capital),
    City(house::House),
}

trait BuildingTrait {
    fn production(&self) -> Vec<(resource::Resource, i32)>;
    fn name(&self) -> String;
    fn get_mesh(&self, building_resources: &Res<BuildingResources>) -> Handle<Scene>;
    fn get_material(&self, building_resources: &Res<BuildingResources>)
        -> Handle<StandardMaterial>;
    fn load_mesh() -> Mesh;
    fn load_material() -> StandardMaterial;
}

#[derive(Resource)]
pub struct BuildingResources {
    capital_mesh: Handle<Scene>,
    capital_material: Handle<StandardMaterial>,
    city_mesh: Handle<Scene>,
    city_material: Handle<StandardMaterial>,
}

pub fn create_building_resources<'a>(
    mut materials: ResMut<'a, Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) -> (BuildingResources, ResMut<'a, Assets<StandardMaterial>>) {
    let capital_mesh = asset_server.load("capital.glb#Scene0");
    let capital_material = materials.add(capital::Capital::load_material());
    let city_mesh = asset_server.load("city.glb#Scene0");
    let city_material = materials.add(house::House::load_material());

    let resources = BuildingResources {
        capital_mesh,
        capital_material,
        city_mesh,
        city_material,
    };

    (resources, materials)
}

pub fn building_production(building: &Building) -> Vec<(resource::Resource, i32)> {
    match &building.kind {
        BuildingKind::Capital(capital) => capital.production(),
        BuildingKind::City(city) => city.production(),
    }
}

pub fn building_name(building: &Building) -> String {
    match &building.kind {
        BuildingKind::Capital(capital) => capital.name(),
        BuildingKind::City(city) => city.name(),
    }
}

pub fn building_mesh(
    building: &Building,
    building_resources: &Res<BuildingResources>,
) -> SceneBundle {
    let scene = match &building.kind {
        BuildingKind::Capital(capital) => capital.get_mesh(building_resources),
        BuildingKind::City(city) => city.get_mesh(building_resources),
    };

    let mut transform = Transform::from_xyz(0., 0., TILE_SIZE as f32 / 2.);

    transform.scale = Vec3::splat(0.3);
    transform.rotate_local_x(f32::to_radians(90.));

    SceneBundle {
        scene,
        transform,
        ..default()
    }
}

pub fn make_bundle(
    building: &Building,
    building_resources: &Res<BuildingResources>,
) -> (SceneBundle, PickableBundle) {
    (
        building_mesh(building, building_resources),
        PickableBundle::default(), // TODO: This does not work
    )
}
