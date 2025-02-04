use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;
use crate::shader::set_uniform::SetUniform;
use crate::shader::shader_program::ShaderProgram;

pub struct NullableShaderProgram {
    expected_uniforms: HashMap<String, String>,
    expect_bind: Rc<RefCell<bool>>,
    expect_not_bind: bool
}

impl Drop for NullableShaderProgram {
    fn drop(&mut self) {
        if !self.expected_uniforms.is_empty() {
            panic!("Expected uniforms not set: {:?}", self.expected_uniforms);
        }
        if *self.expect_bind.borrow() {
            panic!("Expected to bind, but did not bind!");
        }
    }
}

impl ShaderProgram for NullableShaderProgram {
    fn bind(&self) {
        if *self.expect_bind.borrow() {
            self.expect_bind.replace(false);
        }
        if self.expect_not_bind {
            panic!("Expected not to bind, but did bind!");
        }
    }

    fn set_uniform<T: 'static + SetUniform + ToString>(&mut self, name: String, value: T) {
        let uniform_as_string = value.to_string();

        let should_remove_from_map = match self.expected_uniforms.get(&name) {
            Some(expected_value) => {
                if expected_value != &uniform_as_string {
                    panic!("Expected uniform value to be {}, but got {}", expected_value, uniform_as_string);
                }
                true
            },
            None => false
        };

        if should_remove_from_map {
            self.expected_uniforms.remove(&name);
        }
    }
}
