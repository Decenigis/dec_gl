#![allow(unused)]

use gl::types::GLfloat;
use crate::Vertex;


pub struct Vertex2d {
    pub x: GLfloat,
    pub y: GLfloat,

    pub u: GLfloat,
    pub v: GLfloat
}


impl Vertex for Vertex2d {
    fn initialise_attrib_ptrs () {
        unsafe { 
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 16, 8 as *const _);
            gl::EnableVertexAttribArray(1);    
        };
    }
}