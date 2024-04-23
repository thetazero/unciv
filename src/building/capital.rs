use bevy::prelude::*;

#[derive(Component)]
struct Capital {
    level: i32,
}

impl Building for Capital {
    const DESCRIPTION: &'static str = "The capital of your empire.";
    const NAME: &'static str = "Capital";
}
