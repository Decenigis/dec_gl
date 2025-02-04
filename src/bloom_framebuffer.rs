use gl::types::{GLenum, GLfloat, GLint, GLuint};
use glm::{IVec2, ivec2, UVec2, vec2};
use crate::{FrameBuffer, Renderable, RenderError, Vertex3d};
use super::shader::{GLShaderProgram, ShaderManager};

pub struct BloomFramebuffer {
    fbo_id: GLuint,
    texture_id: GLuint,
    bloom_texture_id: GLuint,
    depth_buffer_id: GLuint,
    ping_pong_buffers: [FrameBuffer; 2],
    out_buffer: FrameBuffer,
    size: IVec2,
    samples: GLint,
    renderable: Renderable,
    do_bloom: bool,
    bloom_levels: i32
}

impl Drop for BloomFramebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, [self.fbo_id].as_ptr());
            gl::DeleteTextures(1, [self.texture_id].as_ptr());
            gl::DeleteTextures(1, [self.bloom_texture_id].as_ptr());
            gl::DeleteTextures(1, [self.depth_buffer_id].as_ptr());
        }
    }
}

impl BloomFramebuffer {
    pub fn new (width: GLint, height: GLint, samples: GLint, do_bloom: bool, bloom_levels: i32) -> Result<BloomFramebuffer, RenderError> {
        let mut fbo_id: GLuint = 0;
        let mut texture_ids: [GLuint; 2] = [0, 0];
        let mut depth_buffer_id: GLuint = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut fbo_id);
            if fbo_id == 0 {
                return Err(RenderError::BufferError { error: "Failed to create framebuffer!".to_string() });
            };

            gl::GenTextures(2, &mut texture_ids[0]);
            if texture_ids[0] == 0 || texture_ids[1] == 0 {
                return Err(RenderError::BufferError { error: "Failed to create texture for framebuffer!".to_string() })
            };

            gl::GenTextures(1, &mut depth_buffer_id);
            if depth_buffer_id == 0 {
                return Err(RenderError::BufferError { error: "Failed to create depth and stencil buffer for framebuffer!".to_string() })
            };

            for i in 0..2 {
                gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, texture_ids[i]); //create the image texture for the FBO
                gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, samples, gl::RGBA, width, height, 0);
                gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            }

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, depth_buffer_id); //create the depth texture for the FBO
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, samples, gl::DEPTH24_STENCIL8, width, height, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D_MULTISAMPLE, texture_ids[0], 0); //bind the image texture
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT1, gl::TEXTURE_2D_MULTISAMPLE, texture_ids[1], 0); //bind the image texture
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::TEXTURE_2D_MULTISAMPLE, depth_buffer_id, 0); //bind the depth texture

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE { //if anything didn't work, return an error
                match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                    gl::FRAMEBUFFER_UNDEFINED => return Err(RenderError::BufferError { error: "Framebuffer undefined!".to_string() }),
                    gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                        if cfg!(windows) {
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

            let vertex_data = vec![ //vertex data to be mapped to screen without the need for a matrix
                                    Vertex3d {x: -1.0,  y: -1.0, z: 0.0, u: 0.0, v: 0.0},
                                    Vertex3d {x:  1.0,  y: -1.0, z: 0.0, u: 1.0, v: 0.0},
                                    Vertex3d {x: -1.0,  y:  1.0, z: 0.0, u: 0.0, v: 1.0},

                                    Vertex3d {x:  1.0,  y: -1.0, z: 0.0, u: 1.0, v: 0.0},
                                    Vertex3d {x:  1.0,  y:  1.0, z: 0.0, u: 1.0, v: 1.0},
                                    Vertex3d {x: -1.0,  y:  1.0, z: 0.0, u: 0.0, v: 1.0},
            ];

            let renderable = Renderable::new_initialised(&vertex_data, None)?;

            let ping_pong_buffers = [
                FrameBuffer::new(width, height)?,
                FrameBuffer::new(width, height)?
            ];

            let out_buffer = FrameBuffer::new(width, height)?;

            Ok(BloomFramebuffer {
                fbo_id,
                texture_id: texture_ids[0],
                bloom_texture_id: texture_ids[1],
                ping_pong_buffers,
                out_buffer,
                depth_buffer_id,
                size: ivec2(width, height),
                samples,
                renderable,
                do_bloom,
                bloom_levels
            })
        }

    }

    pub fn bind_draw_target (&self) {
        const ATTACHMENTS: [GLenum; 2] = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1];
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo_id);
            gl::DrawBuffers(2, &ATTACHMENTS as _);
        }
    }

    pub fn clear (&self) {
        self.bind_draw_target();
        unsafe {
            const CLEAR_COLOUR: [f32; 4] = [0.15f32, 0.2f32, 0.4f32, 1.0f32];
            const BLOOM_COLOUR: [f32; 4] = [0.03f32, 0.06f32, 0.12f32, 0.0f32];
            gl::ClearBufferfv(gl::COLOR, 0, &CLEAR_COLOUR as *const GLfloat);
            gl::ClearBufferfv(gl::COLOR, 1, &BLOOM_COLOUR as *const GLfloat);
            gl::ClearBufferfv(gl::DEPTH, 0, &[1.0f32] as _);
        }

        self.out_buffer.clear();
    }

    pub fn resize (&mut self, width: GLuint, height: GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.texture_id); //re-create the bound texture to match the new dimensions
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, self.samples, gl::RGBA, width as i32, height as i32, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.bloom_texture_id); //re-create the bound texture to match the new dimensions
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, self.samples, gl::RGBA, width as i32, height as i32, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.depth_buffer_id);//re-create the bound texture to match the new dimensions
            gl::TexImage2DMultisample(gl::TEXTURE_2D_MULTISAMPLE, self.samples, gl::DEPTH24_STENCIL8, width as i32, height as i32, 0);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D_MULTISAMPLE, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        self.ping_pong_buffers[0].resize(width, height);
        self.ping_pong_buffers[1].resize(width, height);
        self.out_buffer.resize(width, height);

        self.size = ivec2(width as i32, height as i32);
    }


    pub fn blit (&self, target_size: UVec2, mask: GLuint, filter: GLuint, shader_manager: &mut ShaderManager) -> Result<(), RenderError> { //returns with the default framebuffer bound
        self.blit_colour_layer_to_out_buffer(target_size, mask, filter);
        if self.do_bloom {
            self.blit_brightness_to_ping_pong(target_size, mask, filter);
            self.gaussian_blur_to_outbuffer(target_size, shader_manager)?;
        }

        FrameBuffer::bind_default_framebuffer();
        self.out_buffer.blit(target_size, mask, filter);

        Ok(())
    }

    fn blit_colour_layer_to_out_buffer(&self, target_size: UVec2, mask: GLuint, filter: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
            gl::ReadBuffer(gl::COLOR_ATTACHMENT0);
        }
        self.out_buffer.bind_draw_target();

        self.reusable_blit(target_size, mask, filter);
    }

    fn blit_brightness_to_ping_pong (&self, target_size: UVec2, mask: GLuint, filter: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo_id);
            gl::ReadBuffer(gl::COLOR_ATTACHMENT1);
        }
        self.ping_pong_buffers[0].bind_draw_target();

        self.reusable_blit(target_size, mask, filter);
    }

    fn reusable_blit (&self, target_size: UVec2, mask: GLuint, filter: GLuint){
        unsafe {
            gl::BlitFramebuffer( 0, 0, self.size.x, self.size.y, 0, 0, target_size.x as i32, target_size.y as i32, mask, filter);
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
        }
    }

    fn gaussian_blur_to_outbuffer(&self, target_size: UVec2, shader_manager: &mut ShaderManager) -> Result<(), RenderError> {

        let bloom_shader = shader_manager.bind("BLOOM".to_string())?;
        bloom_shader.set_uniform("screen_size".to_string(), vec2(target_size.x as f32, target_size.y as f32));
        bloom_shader.set_uniform("samples".to_string(), self.samples);
        self.bind_buffer_textures(bloom_shader);

        unsafe { gl::Disable(gl::DEPTH_TEST); }

        for i in 0..self.bloom_levels*2 {
            bloom_shader.set_uniform("direction".to_string(), i % 2);

            self.ping_pong_buffers[((i + 1) % 2) as usize].bind_draw_target();
            self.ping_pong_buffers[(i % 2) as usize].bind_buffer_textures(bloom_shader);

            self.renderable.draw();
        };

        self.draw_blur_to_outbuffer(shader_manager)?;

        unsafe { gl::Enable(gl::DEPTH_TEST); }

        Ok(())
    }

    fn draw_blur_to_outbuffer(&self, shader_manager: &mut ShaderManager) -> Result<(), RenderError>  {
        unsafe {
            gl::BlendFunc(gl::ONE, gl::ONE);
        }

        let generic_shader = shader_manager.bind("GENERIC".to_string())?;
        self.ping_pong_buffers[0].bind_buffer_textures(generic_shader);
        self.out_buffer.bind_draw_target();

        self.renderable.draw();

        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Ok(())
    }

     fn bind_buffer_textures(&self, shader_program: &mut GLShaderProgram) { // The textures will ALWAYS be bound to unit 0 and 1
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.texture_id);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, self.bloom_texture_id);

            shader_program.set_uniform("colour_texture".to_string(), 0);
            shader_program.set_uniform("bloom_texture".to_string(), 1);
        }
    }
}