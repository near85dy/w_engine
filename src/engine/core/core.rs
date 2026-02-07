use std::{collections::HashMap, path::Path};

use glam::Vec3;
use uuid::Uuid;

use crate::engine::{core::{ lua::lua::Lua, static_object::StaticObject, texture::Texture}, renderer::{render_command::RenderCommand, transform::Transform}, resource_manager::{model_loader::ModelLoader, texture_loader::TextureLoader}};


pub struct Core {
    static_meshes: HashMap<Uuid, StaticObject>,
    loaded_textures: HashMap<Uuid, Texture>,
    lua: Lua,
    camera_tranform: Transform,
}

impl Core {
    pub fn new() -> Self {
        Self {
            static_meshes: HashMap::new(),
            loaded_textures: HashMap::new(),
            camera_tranform: Transform::new(),
            lua: Lua::new(),
        }
    }

    pub fn initialize(&mut self)
    {
        self.lua.initialize();
    }

    pub fn update(&mut self, render_commands: &mut Vec<RenderCommand>)
    {
        if self.static_meshes.is_empty()
        {
            let model_data = ModelLoader::load_model("Untitled.obj".to_string()).unwrap();
            let uuid = Uuid::new_v4();

            render_commands.push(RenderCommand::LoadMesh { 
                id: uuid.clone(),
                vertices: model_data.vertices, 
                indices: model_data.indices 
            });

            self.static_meshes.insert(uuid.clone(), StaticObject::new(Transform::new(), "Untitled.obj".to_string()));
        
            let (texture_data, width, height) = TextureLoader::load("1.png");

            render_commands.push(RenderCommand::LoadTexture { id: uuid.clone(), width: width as i32, height: height as i32, image: texture_data });
            
            self.loaded_textures.insert(uuid.clone(), Texture::new());

            self.camera_tranform.position = Vec3::new(0.0, 0.0, 5.0);
            render_commands.push(RenderCommand::TransformCamera { position: self.camera_tranform.position, rotation: self.camera_tranform.rotation});
        }
        else {
            for (uuid, mesh) in &mut self.static_meshes {

                mesh.transform.rotate_local_y(0.0005);
                render_commands.push(RenderCommand::BindTexture { id: uuid.clone() });
                render_commands.push(RenderCommand::DrawMesh { id: uuid.clone(), transform: mesh.transform });
            }
        }
    }
}