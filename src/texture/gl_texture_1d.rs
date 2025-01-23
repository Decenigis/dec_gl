extern crate gl;
use gl::types::GLuint;
use crate::RenderError;

pub struct Texture1D {
    id: GLuint,
    size: i32
}

impl Drop for Texture1D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}


impl Texture1D {
    pub fn new() -> Texture1D{
        let texture = Texture1D { id: 0, size: 0 };

        texture
    }


    pub fn set_data(&mut self, data: &Vec<u8>, size: i32, aliased: bool) -> Result<(), RenderError>{   
        self.size = size;

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
            if aliased {
                gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            } else {
                gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            }

            

            gl::TexImage1D(
                gl::TEXTURE_1D,
                0,
                gl::RGBA as i32,
                self.size,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
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