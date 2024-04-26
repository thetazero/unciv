use bevy::prelude::*;

use super::UnitTrait;

pub struct Settler {}

impl Default for Settler {
    fn default() -> Self {
        Settler {}
    }
}

pub struct SettlerResources {
    pub mesh: Handle<Mesh>,
    pub color: Handle<ColorMaterial>,
    pub selected_color: Handle<ColorMaterial>,
}

pub fn init_resources<'a, 'b>(
    mut materials: ResMut<'a, Assets<ColorMaterial>>,
    mut meshes: ResMut<'b, Assets<Mesh>>,
) -> (
    SettlerResources,
    ResMut<'a, Assets<ColorMaterial>>,
    ResMut<'b, Assets<Mesh>>,
) {
    let mesh = meshes.add(Mesh::from(Circle { radius: 20.0 }));

    let color = materials.add(Color::hsl(0., 0.8, 0.5));
    let selected_color = materials.add(Color::hsl(0., 0.8, 0.7));

    (
        SettlerResources {
            mesh,
            color,
            selected_color,
        },
        materials,
        meshes,
    )
}

impl UnitTrait for Settler {
    fn get_normal_material(
        &self,
        unit_resources: &Res<super::UnitResources>,
    ) -> Handle<ColorMaterial> {
        return unit_resources.settler.color.clone();
    }

    fn get_selected_material(&self, unit_resources: &Res<super::UnitResources>) -> Handle<ColorMaterial> {
        return unit_resources.settler.selected_color.clone();
    }
}
