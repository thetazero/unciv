use crate::resource;

use super::BuildingTrait;

#[derive(Clone)]
pub struct Capital {
    level: i32,
}

impl Default for Capital {
    fn default() -> Self {
        Capital { level: 1 }
    }
}

impl BuildingTrait for Capital {
    fn production(&self) -> Vec<(resource::Resource, i32)> {
        vec![
            (resource::Resource::Wood, 1 * self.level),
            (resource::Resource::Stone, 1 * self.level),
        ]
    }

    fn name(&self) -> String {
        format!("Capital {}", self.level)
    }
}
