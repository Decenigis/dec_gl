extern crate gl;
use std::path::Path;

use gl::types::GLuint;
use glm::{IVec2, ivec2};
use image::RgbaImage;
use crate::RenderError;


pub struct Texture2D {
    id: GLuint,
    size: IVec2
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        if self.id == 0 { return }
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}


impl Texture2D {
    pub fn new(path: &Path, antialiased: bool) -> Result<Texture2D, RenderError>{
        let mut texture = Self::new_uninitialised();
        match texture.load(path, antialiased) {
            Ok(_) => {},
            Err(e) => return Err(e),   
        }
        Ok(texture)
    }

    pub fn new_from_raw_data(data: &Vec<u8>, size: IVec2, antialiased: bool) -> Result<Texture2D, RenderError> {
        let mut texture = Self::new_uninitialised();
        match texture.load_from_raw_data(data, size, antialiased) {
            Ok(_) => {},
            Err(e) => return Err(e),   
        }
        Ok(texture)
    }

    pub fn new_error_texture() -> Texture2D {
        let mut texture = Self::new_uninitialised() ;
        texture.load_error_texture();

        texture
    }

    pub fn new_uninitialised() -> Texture2D {
        Texture2D { id: 0, size: ivec2(0, 0) }
    }

    fn load(&mut self, path: &Path, antialiased: bool) -> Result<(), RenderError> {

        match image::open(path) {
            Err(_e) => {
                self.load_error_texture();
                Ok(())
            },
            Ok(image) => {
                let mut rgb_image = image.into_rgba8();
                image::imageops::flip_vertical_in_place(&mut rgb_image);

                self.load_from_raw_data( rgb_image.as_raw(), ivec2(rgb_image.width() as i32, rgb_image.height() as i32,), antialiased)
            },
        }
    }

    fn load_from_raw_data(&mut self, data: &Vec<u8>, size: IVec2, antialiased: bool) -> Result<(), RenderError> {
        unsafe { gl::GenTextures(1, &mut self.id); }
        if self.id == 0 {
            return Err(RenderError::TextureIntError { error: "Failed to generate OpenGL texture ID.".to_string() })
        }
        self.bind();

        unsafe { 
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            if antialiased {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            }

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                size.x,
                size.y,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            ); 
        }

        self.size = ivec2(size.x, size.y);

        Ok(())
    }

    pub fn load_error_texture(&mut self) {
        let error_image = RgbaImage::from_vec(2, 2, vec![0, 0, 0, 255, 255, 255, 0, 255, 0, 0, 0, 255, 255, 255, 0, 255]).unwrap();

        self.load_from_raw_data( error_image.as_raw(), ivec2(error_image.width() as i32, error_image.height() as i32,), false).unwrap();
    }

    pub fn bind(&self) {
        unsafe { 
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id); 
        }
    }

    pub fn bind_to_unit(&self, unit: u32) {
        unsafe { 
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id); 
        }
    }
}