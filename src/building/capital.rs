use crate::resource;

#[derive(Clone)]
pub struct Capital {
    level: i32,
}

impl Default for Capital {
    fn default() -> Self {
        Capital { level: 1 }
    }
}

pub fn production(capital: &Capital) -> Vec<(resource::Resource, i32)> {
    vec![
        (resource::Resource::Wood, 1),
        (resource::Resource::Stone, 1),
    ]
}

pub fn name() -> String {
    "Capital".to_string()
}
