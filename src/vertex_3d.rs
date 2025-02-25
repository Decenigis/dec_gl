#![allow(unused)]

use gl::types::GLfloat;
use crate::Vertex;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex3d {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    pub u: GLfloat,
    pub v: GLfloat
}


impl Vertex for Vertex3d {
    fn initialise_attrib_ptrs () {
        unsafe { 
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 20, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 20, 12 as *const _);
            gl::EnableVertexAttribArray(1);    
        };
    }
}
