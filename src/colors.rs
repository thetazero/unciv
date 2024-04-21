use bevy::prelude::*;

pub fn dark_hue(hue: f32) -> Color {
    Color::hsl(360. * hue, 0.95, 0.2)
}

pub fn bright_hue(hue: f32) -> Color {
    Color::hsl(360. * hue, 0.95, 0.7)
}
