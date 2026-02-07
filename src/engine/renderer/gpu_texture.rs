use gl::types::{GLint, GLsizei, GLuint};


pub struct GpuTexture {
    texture_id: GLuint
}

impl GpuTexture {
    pub fn new(
        image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
        width: i32,
        height: i32
    ) -> Self {
        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const _
            );
            
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

            gl::BindTexture(gl::TEXTURE_2D, 0);        
        }
        Self { 
            texture_id 
        }
    }

    pub fn texture_id(&self) -> GLuint
    {
        self.texture_id
    } 
}