use gl::types::{GLenum, GLuint};

pub struct Shader {
    id: GLuint
}


impl Shader {

    pub fn empty_shader () -> Shader {
        Shader { id: 0 }
    }
    pub fn load_and_compile_shader(source: &String, shader_type: GLenum)  -> Result<Shader, String>{
        let mut frag_shader = Shader { id: 0 };

        unsafe { // Compiling the shader
            frag_shader.id = gl::CreateShader(shader_type);
            if frag_shader.id == 0 {
                return Err("GPU returned shader with ID 0; perhaps OpenGL is not supported or settings are wrong for the system type?".to_string())
            } // Shader ID should NEVER be 0, this is very bad

            gl::ShaderSource(
                frag_shader.id,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
            gl::CompileShader(frag_shader.id);
        }

        unsafe { //Get the result of shader comp and throw an error if it failed
            let mut success = 0;
            gl::GetShaderiv(frag_shader.id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;

                gl::GetShaderInfoLog(
                    frag_shader.id,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                return Err(format!("Compilation Error: {}", String::from_utf8_lossy(&v).to_string()));
            }
        }

        Ok(frag_shader)
    }

    #[allow(non_snake_case)]
    pub fn get_ID(&self) -> GLuint {
        self.id
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}