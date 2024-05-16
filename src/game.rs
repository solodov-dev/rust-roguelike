use crate::{
    map::{make_map, Map},
    object::Object,
};

pub struct Game {
    pub map: Map,
}

impl Game {
    pub fn new(objects: &mut Vec<Object>) -> Self {
        Game {
            map: make_map(objects),
        }
    }
}
