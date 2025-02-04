use gl::types::{GLenum, GLuint};

pub struct ShaderComponent {
    id: GLuint
}


impl ShaderComponent {

    pub fn empty_shader () -> ShaderComponent {
        ShaderComponent { id: 0 }
    }
    pub fn load_and_compile_shader(source: &String, shader_type: GLenum)  -> Result<ShaderComponent, String>{
        let mut shader = ShaderComponent { id: 0 };

        unsafe { // Compiling the shader
            shader.id = gl::CreateShader(shader_type);
            if shader.id == 0 {
                return Err("GPU returned shader with ID 0; perhaps OpenGL is not supported or settings are wrong for the system type?".to_string())
            } // Shader ID should NEVER be 0, this is very bad

            gl::ShaderSource(
                shader.id,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
            gl::CompileShader(shader.id);
        }

        unsafe { //Get the result of shader comp and throw an error if it failed
            let mut success = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;

                gl::GetShaderInfoLog(
                    shader.id,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len.try_into().unwrap());
                return Err(format!("Compilation Error: {}", String::from_utf8_lossy(&v).to_string()));
            }
        }

        Ok(shader)
    }

    #[allow(non_snake_case)]
    pub fn get_ID(&self) -> GLuint {
        self.id
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}