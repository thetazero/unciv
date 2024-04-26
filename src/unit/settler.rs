use bevy::prelude::*;

pub struct Settler {}

impl Default for Settler {
    fn default() -> Self {
        Settler {}
    }
}

pub struct SettlerResources {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
}

pub fn init_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<ColorMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    SettlerResources,
    ResMut<'a, Assets<ColorMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(Circle { radius: 25.0 }));

    let color = materials.add(Color::rgb(1.0, 1.0, 0.0));

    (SettlerResources { mesh, color }, materials, meshes)
}
