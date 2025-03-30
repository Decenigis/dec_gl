use gl::types::{GLuint, GLint, GLfloat};
use crate::renderable::{GlRenderable, Renderable};
use crate::types::{IVec2, ivec2, UVec2, Vec4, vec4};
use crate::{RenderError, Vertex2d};
use crate::shader::ShaderProgram;

pub struct SimpleFramebuffer {
    fbo_id: GLuint,
    texture_id: GLuint,
    depth_buffer_id: GLuint,
    size: IVec2,
    renderable: GlRenderable<Vertex2d>,
    clear_colour: Vec4,
}

impl Drop for SimpleFramebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, [self.fbo_id].as_ptr());
            gl::DeleteTextures(1, [self.texture_id].as_ptr());
            gl::DeleteTextures(1, [self.depth_buffer_id].as_ptr());
        }
    }
}


impl SimpleFramebuffer {
    pub fn new(width: GLint, height: GLint) -> Result<SimpleFramebuffer, RenderError> {
        let mut fbo_id: GLuint = 0;
        let mut texture_id: GLuint = 0;
        let mut depth_buffer_id: GLuint = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut fbo_id);
            if fbo_id == 0 {
                return Err(RenderError::BufferError { error: "Failed to create framebuffer!".to_string() })
            };
            gl::GenTextures(1, &mut texture_id);
            if texture_id == 0 {
                return Err(RenderError::BufferError { error: "Failed to create texture for framebuffer!".to_string() })
            };
            gl::GenTextures(1, &mut depth_buffer_id);
            if depth_buffer_id == 0 {
                return Err(RenderError::BufferError { error: "Failed to create depth and stencil buffer for framebuffer!".to_string() })
            };

            gl::BindTexture(gl::TEXTURE_2D, texture_id); //create the image texture for the FBO
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0 as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindTexture(gl::TEXTURE_2D, depth_buffer_id); //create the depth texture for the FBO
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, width, height, 0, gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8, 0 as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture_id, 0); //bind the image texture
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D, depth_buffer_id, 0); //bind the depth texture

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE { //if anything didn't work, return an error
                match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                    gl::FRAMEBUFFER_UNDEFINED => return Err(RenderError::BufferError { error: "Framebuffer undefined!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                        if cfg!(windows){
                            return Err(RenderError::BufferError { error: "Incomplete framebuffer attachment points! This is fatal on Windows.".to_string() })
                        }
                    },
                    gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => return Err(RenderError::BufferError { error: "No images are attached to the framebuffer!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => return Err(RenderError::BufferError { error: "Incomplete draw buffer!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => return Err(RenderError::BufferError { error: "Incomplete read buffer!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => return Err(RenderError::BufferError { error: "Unsupported internal formats for attached framebuffer images!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => return Err(RenderError::BufferError { error: "Multisample textures are invalid!".to_string() }),
                    _ => return Err(RenderError::BufferError { error: "Unknown error with framebuffer creation!".to_string() }),
                }

            }

            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        }

        let vertex_data = vec![ //vertex data to be mapped to screen without the need for a matrix
                                Vertex2d {x: -1.0,  y: -1.0, u: 0.0, v: 0.0},
                                Vertex2d {x:  1.0,  y: -1.0, u: 1.0, v: 0.0},
                                Vertex2d {x: -1.0,  y:  1.0, u: 0.0, v: 1.0},
                                Vertex2d {x: -1.0,  y:  1.0, u: 0.0, v: 1.0},
                                Vertex2d {x:  1.0,  y: -1.0, u: 1.0, v: 0.0},
                                Vertex2d {x:  1.0,  y:  1.0, u: 1.0, v: 1.0},
        ];
        let mut renderable = GlRenderable::<Vertex2d>::new();
        renderable.initialise(&vertex_data, None)?;

        Ok(SimpleFramebuffer {
            fbo_id,
            texture_id,
            depth_buffer_id,
            size: ivec2(width, height),
            renderable,
            clear_colour: vec4(0.0, 0.0, 0.0, 1.0),
        })
    }

    pub fn resize(&mut self, width: GLuint, height: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id); //re-create the bound texture to match the new dimensions
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 0 as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D, self.depth_buffer_id);//re-create the bound texture to match the new dimensions
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH24_STENCIL8 as i32, width as i32, height as i32, 0, gl::DEPTH_STENCIL, gl::UNSIGNED_INT_24_8, 0 as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            self.size = ivec2(width as i32, height as i32);
        }
    }

    pub fn bind_draw_target (&self){
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo_id);
        }
    }

    pub fn bind_default_framebuffer () {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); }
    }

    pub fn draw_this_framebuffer(&self, shader_program: &mut Box<dyn ShaderProgram>) { // The textures will ALWAYS be bound to unit 0 and 1
        self.bind_buffer_textures(shader_program);
        self.renderable.draw();
    }

    pub fn blit (&self, target_size: UVec2, mask: GLuint, filter: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
            gl::BlitFramebuffer( 0, 0, self.size.x, self.size.y, 0, 0, target_size.x as i32, target_size.y as i32, mask, filter);
        }
    }

    pub fn bind_buffer_textures(&self, shader_program: &mut Box<dyn ShaderProgram>) { // The textures will ALWAYS be bound to unit 0 and 1
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.depth_buffer_id);

            shader_program.set_uniform("framebuffer_texture".to_string(), &0);
            shader_program.set_uniform("depth_buffer_texture".to_string(), &1);
        }
    }

    pub fn clear(&self) {
        self.bind_draw_target();
        unsafe {
            gl::ClearBufferfv(gl::COLOR, 0, &self.clear_colour.as_array() as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &[1.0f32] as _);
        }
    }

    pub fn set_clear_colour(&mut self, colour: Vec4) {
        self.clear_colour = colour;
    }
}
