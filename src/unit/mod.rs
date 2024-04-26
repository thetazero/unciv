use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

pub mod settler;

#[derive(Component)]
pub enum Unit {
    Settler(settler::Settler),
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
    unit_resources: Res<UnitResources>,
    unit: Unit,
) -> Commands<'a, 'b> {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(unit_resources.settler.mesh.clone()),
            material: unit_resources.settler.color.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.),
            ..default()
        },
        unit,
    ));

    commands
}
