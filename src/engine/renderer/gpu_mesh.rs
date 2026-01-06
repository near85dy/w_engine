use gl::types::{GLfloat, GLuint};
use uuid::Uuid;


pub struct GpuMesh {
    id: Uuid,
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint,
    pub vertices: Vec<f32>,
    pub indices: Vec<i32>
}

impl GpuMesh {
    pub fn new(id: Uuid, vertices: Vec<f32>, indices: Vec<i32>) -> Self {
        let mut vbo = 0;
        let mut vao = 0;
        let mut ebo = 0;
        
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const _, 
                gl::STATIC_DRAW,
            );  

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<GLuint>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 
                (3*std::mem::size_of::<GLfloat>()) as i32, std::ptr::null());
            
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
        };

        Self { 
            id,
            vbo, 
            vao, 
            ebo,
            indices,
            vertices
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn vao(&self) -> GLuint {
        self.vao
    }
    pub fn vbo(&self) -> GLuint {
        self.vbo
    }
    pub fn ebo(&self) -> GLuint {
        self.ebo
    }
}