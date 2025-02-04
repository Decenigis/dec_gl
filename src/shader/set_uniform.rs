use gl::types::GLint;
use crate::types::{IVec2, Mat4, Vec2, Vec3, Vec4};

pub trait SetUniform { // allows for any type to be settable if you implement SetUniform for it
    unsafe fn set_uniform(self, location: GLint);
}


impl SetUniform for Mat4 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_array().as_ptr() as *const f32) ;
    }
}

impl SetUniform for Vec2 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform2fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for Vec3 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform3fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for Vec4 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform4fv(location, 1, self.as_array().as_ptr()) ;
    }
}

impl SetUniform for IVec2 {
    unsafe fn set_uniform(self, location: GLint) {
        gl::Uniform2i(location, self.x, self.y) ;
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