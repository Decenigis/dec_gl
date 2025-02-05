use crate::shader::set_uniform::SetUniform;

pub trait ShaderProgram {
    fn bind(&self);
    fn set_uniform(&mut self, name: String, value: &dyn SetUniform);
}
