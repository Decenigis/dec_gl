#![allow(dead_code)]

use crate::shader::set_uniform::SetUniform;

pub trait ShaderProgram {
    fn bind(&self);
    fn set_uniform<T: 'static + SetUniform + ToString>(&mut self, name: String, value: T);
}
