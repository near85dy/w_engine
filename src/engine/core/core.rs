use std::collections::HashMap;

use uuid::Uuid;

use crate::engine::{core::mesh::Mesh, renderer::{render_command::RenderCommand, transform::Transform}, resource_manager::model_loader::ModelLoader};


pub struct Core {
    static_meshes: HashMap<Uuid, Mesh>,
}

impl Core {
    pub fn new() -> Self {
        Self {
            static_meshes: HashMap::new(),
        }
    }

    pub fn initialize(&self)
    {
        
    }

    pub fn update(&mut self, render_commands: &mut Vec<RenderCommand>)
    {
        if self.static_meshes.is_empty()
        {
            let model_data = ModelLoader::load_model("Untitled.obj".to_string()).unwrap();
            let mesh = Mesh::new(model_data.vertices, model_data.indices);
            let uuid = Uuid::new_v4();

            render_commands.push(RenderCommand::CreateMesh { id: uuid.clone(), vertices: mesh.vertices.clone(), indices: mesh.indices.clone() });

            self.static_meshes.insert(uuid.clone(), mesh);
        }
        else {
            for (uuid, mesh) in &mut self.static_meshes {
                mesh.transform.rotate_local_x(0.0005);
                render_commands.push(RenderCommand::DrawMesh { id: uuid.clone(), transform: mesh.transform });
            }
        }
    }
}