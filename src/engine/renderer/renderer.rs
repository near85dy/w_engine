use std::{collections::HashMap, ffi::CString};

use gl::{self, types::{GLfloat, GLuint}};
use uuid::Uuid;

use crate::engine::renderer::{camera::Camera, gpu_mesh::GpuMesh, render_command::RenderCommand, render_object_data::RenderObjectData, shader::Shader, transform::Transform};

pub struct Renderer {
   shader_list: HashMap<String, Shader>,
   mesh_list: HashMap<Uuid, RenderObjectData>,
   camera: Camera,
}

impl Renderer {
    pub fn new(window: &mut glfw::PWindow) -> Self {
        gl::load_with(|s| {
            match window.get_proc_address(s) {
                Some(func) => func as *const _,
                None => std::ptr::null(),
            }
        });
        
        Self {
            mesh_list: HashMap::new(),
            shader_list: HashMap::new(),
            camera: Camera::new(),
        }
    }

    pub fn initialize(&mut self)
    {
        let mut shader: Shader = Shader::new();
        shader.create_shader(String::from("base_shader")).unwrap();
        self.shader_list.insert(String::from("base_shader"), shader);

        //unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
    }

    pub fn draw(&mut self, render_commands: Vec<RenderCommand>)
    {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::ClearColor(0.11, 0.67, 0.84, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for command in render_commands {
            match command {
                RenderCommand::CreateMesh { id, vertices, indices } => { 
                    let mesh = GpuMesh::new(id, vertices, indices);
                    let render_data = RenderObjectData::new(mesh);
                    self.mesh_list.insert(id.clone(), render_data);
                    println!("Created mesh: {}", id);
                }
                RenderCommand::DrawMesh { id, transform } => { 
                    if let Some(render_object) = self.mesh_list.get_mut(&id) {
                        let mesh = &render_object.mesh;
                        let mvp = self.camera.projection()
                            * self.camera.view()
                            * transform.get_transform_matrix();

                        unsafe {
                            gl::UseProgram(self.shader_list["base_shader"].get_id());
                            self.shader_list["base_shader"]
                                .set_mat4(CString::new("u_MVP").unwrap(), &mvp);
                            gl::BindVertexArray(mesh.vao());
                            gl::DrawElements(
                                gl::TRIANGLES,
                                mesh.indices.len() as i32,
                                gl::UNSIGNED_INT,
                                std::ptr::null(),
                            );
                            gl::BindVertexArray(0);
                        }
                    } else {
                        eprintln!("DrawMesh called for missing mesh: {:?}", id);
                    }
                },
                _ => {
                    println!("Unknown render command");
                }
            }
        }

        // for(id,render_object) in &self.mesh_list {
        //     let mvp = self.camera.projection() * self.camera.view() * mesh_list[id].transform.get_transform_matrix();
            
        //     unsafe {
        //         gl::UseProgram(self.shader_list["base_shader"].get_id());
        //         self.shader_list["base_shader"].set_mat4(CString::new("u_MVP").unwrap(), &mvp);
        //         gl::BindVertexArray(render_object.vao);
        //         gl::DrawElements(gl::TRIANGLES, mesh_list[id].mesh.indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        //         gl::BindVertexArray(0);
        //     };
        // }
    }
}