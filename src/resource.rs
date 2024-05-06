use bevy::prelude::*;

#[derive(Component, Hash, PartialEq, Eq, Clone, Debug)]
pub enum Resource {
    Wood,
    Stone,
}
