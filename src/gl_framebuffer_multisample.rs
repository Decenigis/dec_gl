use gl::types::{GLuint, GLint, GLfloat};
use glm::{IVec2, ivec2, UVec2};

use crate::{string};
use crate::app::logger;

use super::{RenderError, Vertex, InitialiseAttribPtrs, ShaderProgram, Renderable};

pub struct FrameBufferMultisample {
   fbo_id: GLuint,
   texture_id: GLuint,
   depth_buffer_id: GLuint,
   size: IVec2,
   samples: GLint,
}

impl Drop for FrameBufferMultisample {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, [self.fbo_id].as_ptr());
            gl::DeleteTextures(1, [self.texture_id].as_ptr());
            gl::DeleteTextures(1, [self.depth_buffer_id].as_ptr());
        }
    }
}

impl FrameBufferMultisample {
    pub fn new(width: GLint, height: GLint, samples: GLint) -> Result<FrameBufferMultisample, RenderError> {
        let mut fbo_id: GLuint = 0;
        let mut texture_id: GLuint = 0;
        let mut depth_buffer_id: GLuint = 0;

        unsafe {
            logger::add_to_log(format!("Creating a multisampled framebuffer with {} samples, of size {}x{}", samples, width, height).as_str());

            gl::GenFramebuffers(1, &mut fbo_id);
            if fbo_id == 0 {
                return Err(RenderError::BufferError { error: string!("Failed to create framebuffer!") })
            };
            gl::GenTextures(1, &mut texture_id);
            if texture_id == 0 {
                return Err(RenderError::BufferError { error: string!("Failed to create texture for framebuffer!") })
            };
            gl::GenTextures(1, &mut depth_buffer_id);
            if depth_buffer_id == 0 {
                return Err(RenderError::BufferError { error: string!("Failed to create depth and stencil buffer for framebuffer!") })
            };

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, texture_id); //create the image texture for the FBO
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, samples, gl::RGBA, width, height, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, depth_buffer_id); //create the depth texture for the FBO
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, samples, gl::DEPTH24_STENCIL8, width, height, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D_MULTISAMPLE, texture_id, 0); //bind the image texture
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D_MULTISAMPLE, depth_buffer_id, 0); //bind the depth texture

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE { //if anything didn't work, return an error
                match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                    gl::FRAMEBUFFER_UNDEFINED => return Err(RenderError::BufferError { error: string!("Framebuffer undefined!") }),
                    gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                        if cfg!(windows){
                            return Err(RenderError::BufferError { error: string!("Incomplete framebuffer attachment points! This is fatal on Windows.") })
                        }
                        logger::add_error_to_log("Incomplete framebuffer attachment points! This is not NECESSARILY fatal on the current system.")
                    },
                    gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => return Err(RenderError::BufferError { error: string!("No images are attached to the framebuffer!") }),
                    gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => return Err(RenderError::BufferError { error: string!("Incomplete draw buffer!") }),
                    gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => return Err(RenderError::BufferError { error: string!("Incomplete read buffer!") }),
                    gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => return Err(RenderError::BufferError { error: string!("Unsupported internal formats for attached framebuffer images!") }),
                    gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => return Err(RenderError::BufferError { error: string!("Multisample textures are invalid!") }),
                    _ => return Err(RenderError::BufferError { error: string!("Unknown error with framebuffer creation!") }),
                }

            }

            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        }

        Ok(FrameBufferMultisample {
            fbo_id,
            texture_id,
            depth_buffer_id,
            size: ivec2(width, height),
            samples})
    }

    pub fn resize(&mut self, width: GLuint, height: GLuint) {
        logger::add_to_log(format!("Trying to resize a framebuffer to size {}x{}", width, height).as_str());
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.texture_id); //re-create the bound texture to match the new dimensions
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, self.samples, gl::RGBA, width as i32, height as i32, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.depth_buffer_id);//re-create the bound texture to match the new dimensions
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, self.samples, gl::DEPTH24_STENCIL8, width as i32, height as i32, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            self.size = ivec2(width as i32, height as i32);

            logger::add_to_log("Successfully resized framebuffer");
        }
    }

    pub fn bind_draw_target (&self) -> bool{
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo_id);
            true
        }
    }

    pub fn bind_default_framebuffer () {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); }
    }

    pub fn blit (&self, target_size: UVec2, mask: GLuint, filter: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
            gl::BlitFramebuffer( 0, 0, self.size.x, self.size.y, 0, 0, target_size.x as i32, target_size.y as i32, mask, filter);
        }
    }

    pub fn bind_buffer_textures(&self, shader_program: &mut ShaderProgram) { // The textures will ALWAYS be bound to unit 0 and 1
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.texture_id);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.depth_buffer_id);

            shader_program.set_uniform(string!("framebuffer_texture"), 0);
            shader_program.set_uniform(string!("depth_buffer_texture"), 1);
        }
    }

    pub fn clear(&self, mask: GLuint) {
        self.bind_draw_target();
        unsafe { gl::Clear(mask); }
    }

    pub fn clear_with_zeros(&self) {
        self.bind_draw_target();
        unsafe {
            let temp = [1.0f32, 1.0f32, 1.0f32, 0.0f32];
            gl::ClearBufferfv(gl::COLOR, 0, &temp as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &[1.0f32] as _);
        }
    }
}