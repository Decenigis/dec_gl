extern crate gl;
use gl::types::{GLuint, GLint};
use std::collections::HashMap;

use crate::shader::SetUniform;
use crate::{RenderError};
use super::Shader;


pub struct ShaderProgram { // what will actually be used as a shader
    id: GLuint,
    _name: String,
    uniforms: HashMap<String, GLint> // cache of the locations to free up some GPU time, however minimal 
}


impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

impl ShaderProgram {
    pub fn load_shader_program(filepath: &str, identifying_string: &str, geometry_included: bool) -> Result<ShaderProgram, RenderError> { //returns the finalised shader struct
        let mut shader_program = ShaderProgram { id: 0, _name: identifying_string.to_string(), uniforms: HashMap::new() }; // create a struct ready for the final ID

        // -- VERTEX SHADER -- //

        let vert_path = format!("{}.vsh", filepath); //format for filename of the shader

        let vert_shader_source = match std::fs::read_to_string(vert_path.clone()) {
            Ok(file_source) => file_source,
            Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: vert_path.to_string(), error: e.to_string() })
        };// loading source code for the vertex shader
            
        let vert_shader = match Shader::load_and_compile_shader(&vert_shader_source, gl::VERTEX_SHADER) {
            Ok(shader) => shader,
            Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: "VERTEX".to_string(), error: e }),
        } ;//compile the vertex shader

        // -- FRAGMENT SHADER -- //

        let frag_path = format!("{}.fsh", filepath);  //format for filename of the shader

        let frag_shader_source = match std::fs::read_to_string(frag_path.clone()) {
            Ok(file_source) => file_source,
            Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: frag_path.to_string(), error: e.to_string() })
        }; // loading source code for the fragment shader
            
        let frag_shader = match Shader::load_and_compile_shader(&frag_shader_source, gl::FRAGMENT_SHADER){
            Ok(shader) => shader,
            Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: "FRAGMENT".to_string(), error: e }),
        } ; //compile the fragment shader
        // -- GEOMETRY SHADER -- //

        let geometry_shader = if geometry_included {
            let geometry_path = format!("{}.gsh", filepath);

            let geometry_shader_source = match std::fs::read_to_string(geometry_path.clone()) {
                Ok(file_source) => file_source,
                Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: geometry_path.to_string(), error: e.to_string() })
            }; // loading source code for the fragment shader

            let geometry_shader = match Shader::load_and_compile_shader(&geometry_shader_source, gl::FRAGMENT_SHADER){
                Ok(shader) => shader,
                Err(e) => return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: "GEOMETRY".to_string(), error: e }),
            } ; //compile the fragment shader

            geometry_shader
        }
        else {
            Shader::empty_shader()
        };

        // -- SHADER PROGRAM -- //

        unsafe { 
            shader_program.id = gl::CreateProgram(); 
            gl::AttachShader(shader_program.id, vert_shader.get_ID()); // attach vertex to the shader program
            gl::AttachShader(shader_program.id, frag_shader.get_ID()); // attach fragment to the shader program

            if geometry_included {
                gl::AttachShader(shader_program.id, geometry_shader.get_ID());
            } // attach geometry to the shader program!

            gl::LinkProgram(shader_program.id);
        }
        
        //Get errors and handle them (this is a bit of a black box, but it works)
        unsafe {
            let mut success = 0;
            gl::GetProgramiv(shader_program.id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len: i32 = 0;
                gl::GetProgramInfoLog(
                shader_program.id,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                return Err(RenderError::ShaderError { shader_name: identifying_string.to_string(), shader_type: "PROGRAM".to_string(), error: format!("Program Link Error: {}", String::from_utf8_lossy(&v)) });
            }
        }

        let mut num_of_uniforms = 0;
        unsafe { gl::GetProgramiv(shader_program.id, gl::ACTIVE_UNIFORMS, &mut num_of_uniforms); }
        
        vert_shader.delete();
        frag_shader.delete(); //clean up now unused memory for the parts of the shaders

        if geometry_included {
            geometry_shader.delete();
        }

        shader_program.bind();

        Ok(shader_program)
    } 

    pub fn bind(&self) { 
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn set_uniform<T: SetUniform>(&mut self, name: String, value: T){
        let location: GLint;

        self.bind();

        match &mut self.uniforms.get(&name) { //check the uniform cache for a location associated to this name
            None => {
                let uniform = std::ffi::CString::new(name.clone()).unwrap(); // cstring bollocks for memory security (weird, right?)
                location = unsafe { gl::GetUniformLocation(self.id, uniform.as_ptr() as *const i8) }; // request location ID from GPU
            
                self.uniforms.insert(name, location); //add the location ID to the cache
            },
            Some(val) => {
                location = **val; // double deref the location ID for some reason, it just works!
            }
        };

        unsafe { value.set_uniform(location); }
    }
}