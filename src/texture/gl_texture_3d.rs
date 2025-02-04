extern crate gl;

use gl::types::GLuint;
use mockall::automock;
use crate::RenderError;
use crate::types::{IVec3, ivec3};

pub struct Texture3D {
    id: GLuint,
    size: IVec3
}

impl Drop for Texture3D {
    fn drop(&mut self) {
        if self.id == 0 { return }
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}


#[automock]
impl Texture3D {
    pub fn new_from_raw_data(data: &Vec<u8>, size: IVec3, antialiased: bool) -> Result<Texture3D, RenderError> {
        let mut texture = Self::new_uninitialised();
        match texture.load_from_raw_data(data, size, antialiased) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        Ok(texture)
    }

    pub fn new_uninitialised() -> Texture3D {
        Texture3D { id: 0, size: ivec3(0, 0, 0) }
    }

    fn load_from_raw_data(&mut self, data: &Vec<u8>, size: IVec3, antialiased: bool) -> Result<(), RenderError> {
        unsafe { gl::GenTextures(1, &mut self.id); }
        if self.id == 0 {
            return Err(RenderError::TextureIntError { error: "Failed to generate OpenGL texture ID.".to_string() })
        }
        self.bind();

        unsafe {
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            if antialiased {
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            } else {
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            }

            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                gl::RGBA as i32,
                size.x,
                size.y,
                size.z,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
        }

        self.size = size;

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