extern crate gl;

use gl::types::GLuint;
use mockall::automock;
use crate::RenderError;

pub struct Texture1DInt {
    id: GLuint,
    size: i32
}

impl Drop for Texture1DInt {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl Default for Texture1DInt {
    fn default() -> Self {
        Self::new()
    }
}


#[automock]
impl Texture1DInt {
    pub fn new() -> Texture1DInt{
        let texture = Texture1DInt { id: 0, size: 0 };

        texture
    }


    pub fn set_data(&mut self, data: &Vec<GLuint>) -> Result<(), RenderError>{   
        self.size = data.len() as i32;

        if self.id == 0 {
            unsafe { gl::GenTextures(1, &mut self.id); }
            if self.id == 0 {
                return Err(RenderError::TextureIntError { error: "Failed to generate OpenGL texture ID.".to_string() })
            }
        }

        self.bind();

        unsafe { 
            gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            

            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                gl::R32UI as i32,
                self.size,
                0,
                gl::RED_INTEGER,
                gl::UNSIGNED_INT,
                data.as_ptr().cast(),
            ); 
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe { 
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_1D, self.id); 
        }
    }

    pub fn bind_to_unit(&self, unit: u32) {
        unsafe { 
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_1D, self.id); 
        }
    }
}