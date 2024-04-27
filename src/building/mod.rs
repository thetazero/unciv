use crate::resource;

pub mod capital;
pub mod city;

#[derive(Clone)]
pub enum Building {
    Capital(capital::Capital),
    City(city::City),
}

trait BuildingTrait {
    fn production(&self) -> Vec<(resource::Resource, i32)>;
    fn name(&self) -> String;
}

pub fn building_production(building: &Building) -> Vec<(resource::Resource, i32)> {
    match building {
        Building::Capital(capital) => capital.production(),
        Building::City(city) => city.production(),
    }
}

pub fn building_name(building: &Building) -> String {
    match building {
        Building::Capital(capital) => capital.name(),
        Building::City(city) => city.name(),
    }
}
