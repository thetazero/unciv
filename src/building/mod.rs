use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::PickableBundle;

use crate::{resource, utils};

pub mod capital;
pub mod city;

#[derive(Clone)]
pub enum Building {
    Capital(capital::Capital),
    City(city::City),
}

trait BuildingTrait {
    fn production(&self) -> Vec<(resource::Resource, i32)>;
    fn name(&self) -> String;
    fn get_mesh(&self, building_resources: &Res<BuildingResources>) -> Handle<Mesh>;
    fn get_material(&self, building_resources: &Res<BuildingResources>) -> Handle<ColorMaterial>;
    fn load_mesh() -> Mesh;
    fn load_material() -> ColorMaterial;
}

#[derive(Resource)]
pub struct BuildingResources {
    capital_mesh: Handle<Mesh>,
    capital_material: Handle<ColorMaterial>,
    city_mesh: Handle<Mesh>,
    city_material: Handle<ColorMaterial>,
}

pub fn create_building_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<ColorMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    BuildingResources,
    ResMut<'a, Assets<ColorMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let capital_mesh = meshes.add(capital::Capital::load_mesh());
    let capital_material = materials.add(capital::Capital::load_material());
    let city_mesh = meshes.add(city::City::load_mesh());
    let city_material = materials.add(city::City::load_material());

    let resources = BuildingResources {
        capital_mesh,
        capital_material,
        city_mesh,
        city_material,
    };

    (resources, materials, meshes)
}

pub fn building_production(building: &Building) -> Vec<(resource::Resource, i32)> {
    match building {
        Building::Capital(capital) => capital.production(),
        Building::City(city) => city.production(),
    }
}

pub fn building_name(building: &Building) -> String {
    match building {
        Building::Capital(capital) => capital.name(),
        Building::City(city) => city.name(),
    }
}

pub fn building_mesh(
    building: &Building,
    building_resources: &Res<BuildingResources>,
) -> Handle<Mesh> {
    match building {
        Building::Capital(capital) => capital.get_mesh(building_resources),
        Building::City(city) => city.get_mesh(building_resources),
    }
}

pub fn building_material(
    building: &Building,
    building_resources: &Res<BuildingResources>,
) -> Handle<ColorMaterial> {
    match building {
        Building::Capital(capital) => capital.get_material(building_resources),
        Building::City(city) => city.get_material(building_resources),
    }
}

pub fn make_bundle(
    building: &Building,
    building_resources: &Res<BuildingResources>,
) -> (MaterialMesh2dBundle<ColorMaterial>, PickableBundle) {
    // let mut transform = utils::to_transform(location);
    // transform.translation.z = 2.;
    let transform = Transform::from_xyz(0., 0., 2.);

    (
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(building_mesh(building, building_resources)),
            material: building_material(building, building_resources),
            transform,
            ..default()
        },
        PickableBundle::default(),
    )
}
