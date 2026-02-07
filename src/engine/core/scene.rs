use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::engine::core::{entity::Entity, static_object::StaticObject};

pub struct Scene {
   static_objects: HashMap<String, StaticObject>,
   entities: HashMap<String, Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {  
            entities: HashMap::new(),
            static_objects: HashMap::new(),
        }
    }
}