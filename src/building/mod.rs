use crate::resource;

trait Building {
    const DESCRIPTION: &'static str;
    const NAME: &'static str;
    fn production(&self) -> Vec<(resource::Resource, i32)>;
}
