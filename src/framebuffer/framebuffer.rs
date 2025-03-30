use gl::types::GLuint;
use crate::RenderError;
use crate::shader::ShaderManager;
use crate::types::UVec2;

pub trait Framebuffer {

    fn bind_draw_target (&self);
    fn clear (&self);
    fn resize (&mut self, width: GLuint, height: GLuint);
    fn blit (&self, target_size: UVec2, mask: GLuint, filter: GLuint, shader_manager: &mut ShaderManager) -> Result<(), RenderError>;

    fn bind_default_framebuffer () {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); }
    }
}
