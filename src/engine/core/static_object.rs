use serde::{Deserialize, Serialize};

use crate::engine::renderer::transform::{self, Transform};

pub struct StaticObject {
    mesh_name: String,
    pub transform: Transform
}

impl StaticObject {
    pub fn new(transform: Transform, mesh_name: String) -> Self {
        Self {
            mesh_name,
            transform,
        }
    }

    // pub fn transform(&mut self) -> Transform {
    //     self.transform
    // }

    pub fn mesh_name(&self) -> String {
        self.mesh_name.clone()
    }
    
}