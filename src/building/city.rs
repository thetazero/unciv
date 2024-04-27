use super::BuildingTrait;
use crate::resource;

#[derive(Clone)]
pub struct City {}

impl Default for City {
    fn default() -> Self {
        City {}
    }
}

impl BuildingTrait for City {
    fn production(&self) -> Vec<(resource::Resource, i32)> {
        vec![
            (resource::Resource::Wood, 1),
            (resource::Resource::Stone, 1),
        ]
    }

    fn name(&self) -> String {
        "City".to_string()
    }
}
