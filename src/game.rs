use crate::{
    gui::Messages,
    map::{make_map, Map},
    object::Object,
};

pub struct Game {
    pub map: Map,
    pub messages: Messages,
}

impl Game {
    pub fn new(objects: &mut Vec<Object>) -> Self {
        Game {
            map: make_map(objects),
            messages: Messages::new(),
        }
    }
}
