use std::cell::RefCell;
use std::collections::{HashMap};
use std::rc::Rc;
use crate::shader::set_uniform::SetUniform;
use crate::shader::shader_program::ShaderProgram;

pub struct NullableShaderProgram {
    uniform_values: Rc<RefCell<HashMap<String, String>>>,
    bound: Rc<RefCell<bool>>,
}

impl ShaderProgram for NullableShaderProgram {
    fn bind(&self) {
        self.bound.replace(true);
    }

    //Converts all uniform values to Strings.
    //This is because rust dyn references suck
    fn set_uniform(&mut self, name: String, value: &dyn SetUniform) {
        self.uniform_values.borrow_mut().insert(name, value.to_string());
    }
}

impl NullableShaderProgram {
    pub fn new(uniform_values: Rc<RefCell<HashMap<String, String>>>, bound: Rc<RefCell<bool>>) -> Self {
        Self {
            uniform_values,
            bound,
        }
    }
}



#[cfg(test)]
mod nullable_shader_program_tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::shader::{NullableShaderProgram, ShaderProgram};

    #[test]
    fn bind_test() {
        let bound = Rc::new(RefCell::new(false));
        let shader_program = NullableShaderProgram::new(Rc::new(RefCell::new(HashMap::new())), bound.clone());

        shader_program.bind();

        assert_eq!(*bound.borrow(), true);
    }

    #[test]
    fn set_uniform_test() {
        let value = 1;

        let uniform_values = Rc::new(RefCell::new(HashMap::new()));
        let mut shader_program = NullableShaderProgram::new(uniform_values.clone(), Rc::new(RefCell::new(false)));

        shader_program.set_uniform("test".to_string(), &value);

        assert_eq!(*uniform_values.borrow().get("test").unwrap(), value.to_string());
    }
}