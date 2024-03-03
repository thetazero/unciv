use bevy::prelude::*;

pub fn bright_hue(hue: f32) -> Color {
    Color::hsl(360. * hue, 0.95, 0.7)
}

pub fn normal_hue(hue: f32) -> Color {
    Color::hsl(360. * hue, 0.85, 0.4)
}
