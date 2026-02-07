use std::{ffi::CString, fs, ptr};

use gl::{GetUniformLocation, types::{self, GLuint}};
use glam::{Mat4, Vec3};


pub struct Shader {
    program_id: GLuint,
}

impl Shader {
    pub fn new() -> Self {
        let program_id = unsafe { gl::CreateProgram() };
         
        Self { 
            program_id
        }
    }

    pub fn get_id(&self) -> GLuint
    {
        return self.program_id;
    }

    pub fn create_shader(&mut self, file_name: String) -> std::io::Result<()> {
        let frag_shader_code = CString::new(fs::read_to_string(
            format!("assets/shaders/{}.fsh", file_name)
        )?).unwrap();

        let vert_shader_code = CString::new(fs::read_to_string(
            format!("assets/shaders/{}.vsh", file_name)
        )?).unwrap();

        unsafe {
            let shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(shader_id, 1, &frag_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);

            gl::AttachShader(self.program_id, shader_id);

            gl::LinkProgram(self.program_id);

            gl::DeleteShader(shader_id);
        };

        unsafe {
            let shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(shader_id, 1, &vert_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);

            gl::AttachShader(self.program_id, shader_id);

            gl::LinkProgram(self.program_id);

            gl::DeleteShader(shader_id);
        };

        println!("Shader {} was loaded", file_name);
        Ok(())
    }

    pub fn set_mat4(&self, location_name: CString, matrix: &Mat4)
    {
        unsafe {
            let location_id = gl::GetUniformLocation(self.program_id, location_name.as_ptr());
            gl::UniformMatrix4fv(location_id, 1, gl::FALSE, matrix.as_ref().as_ptr());
        }
    }

    pub fn set_vec3(&self, location_name: CString, vec: Vec3)
    {
        unsafe {
            let location_id = gl::GetUniformLocation(self.program_id, location_name.as_ptr());
            gl::Uniform3f(location_id, vec.x, vec.y, vec.z);
        }
    }
}