use gl::types::GLint;

pub trait SetUniform { // allows for any type to be settable if you implement SetUniform for it
    unsafe fn set_uniform(self, location: GLint);
}


impl SetUniform for glm::Mat4 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_array().as_ptr() as *const f32) ;
    }
}

impl SetUniform for glm::Vec2 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform2fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for glm::Vec3 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform3fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for glm::Vec4 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform4fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for f32 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform1f(location, self);
    }
}

impl SetUniform for i32{
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform1i(location, self);
    }
}