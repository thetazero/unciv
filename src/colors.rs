use bevy::prelude::*;

pub fn bright_hue(hue: f32) -> Color {
    Color::hsl(360. * hue, 0.95, 0.7)
}

pub fn plastic_material(hue: f32, saturation: f32, lightness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::hsl(hue, saturation, lightness),
        metallic: 0.0,
        reflectance: 0.0,
        ..Default::default()
    }
}
