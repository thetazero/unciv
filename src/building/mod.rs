use crate::resource;

pub mod capital;

#[derive(Clone)]
pub enum Building {
    Capital(capital::Capital),
}

pub fn building_production(building: &Building) -> Vec<(resource::Resource, i32)> {
    match building {
        Building::Capital(capital) => capital::production(&capital),
    }
}

pub fn building_name(building: &Building) -> String {
    match building {
        Building::Capital(cap) => capital::name(cap),
    }
}
