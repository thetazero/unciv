use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq)]
pub enum Resource {
    Wood,
    Stone,
}
