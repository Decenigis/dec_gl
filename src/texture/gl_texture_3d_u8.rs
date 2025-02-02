extern crate gl;

use gl::types::GLuint;
use glm::{IVec3, ivec3};
use mockall::automock;
use crate::RenderError;

pub struct Texture3Du8 {
    id: GLuint,
    size: IVec3
}

impl Drop for Texture3Du8 {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}


#[automock]
impl Texture3Du8 {
    pub fn new() -> Texture3Du8 {
        let texture = Texture3Du8 { id: 0, size: ivec3(0, 0, 0) };

        texture
    }


    pub fn set_data(&mut self, data: &Vec<u8>, size: IVec3) -> Result<(), RenderError>{
        self.size = size;

        if data.len() as i32 != self.size.x * self.size.y * self.size.z {
            return Err(RenderError::TextureIntError { error: "Set image dimensions do not fit the size of the provided data!".to_string() });
        }

        if self.id == 0 {
            unsafe { gl::GenTextures(1, &mut self.id); }
            if self.id == 0 {
                return Err(RenderError::TextureIntError { error: "Failed to generate OpenGL texture ID.".to_string() })
            }
        }

        self.bind();

        unsafe {
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                gl::R8UI as i32,
                self.size.x,
                self.size.y,
                self.size.z,
                0,
                gl::RED_INTEGER,
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast(),
            );
        }

        Ok(())
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_3D, self.id);
        }
    }

    pub fn bind_to_unit(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_3D, self.id);
        }
    }
}
