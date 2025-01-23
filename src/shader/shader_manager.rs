use std::collections::HashMap;

use super::ShaderProgram;
use crate::RenderError;

pub struct ShaderManager {
    shader_map: HashMap<String, ShaderProgram>
}


impl ShaderManager {
    pub fn new () -> ShaderManager {
        ShaderManager { shader_map: HashMap::new() }
    }

    pub fn register_shader(&mut self, name: String, shader_result: Result<ShaderProgram, RenderError>) -> Result<&ShaderProgram, RenderError> {
        match shader_result {
            Ok(texture) => {
                match self.shader_map.insert(name.clone(), texture) {
                    Some(_) => Err(RenderError::ShaderError { shader_name: name, shader_type: "SHADER_PROGRAM".to_string(), error: "Shader already exists in shader manager!".to_string() }),
                    None => Ok(self.shader_map.get(&name).unwrap())
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn bind(&mut self, shader_name: String) -> Result<&mut ShaderProgram, RenderError>{
        match self.get_shader(shader_name) {
            Ok(shader) => {
                shader.bind();
                Ok(shader)
            },
            Err(e) => Err(e)
        }      
    }

    pub fn get_shader(&mut self, shader_name: String) -> Result<&mut ShaderProgram, RenderError> {
        match self.shader_map.get_mut(&shader_name.clone()) {
            Some(shader) => Ok(shader),
            None => Err(RenderError::ShaderError { shader_name, shader_type: "SHADER_PROGRAM".to_string(), error: "Shader doesn't exist!".to_string() })
        }
    }
}