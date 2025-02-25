use gl::types::GLuint;
use crate::{RenderError, Vertex};

pub trait Renderable<T: Vertex> {

    fn initialise(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError>;
    fn update_data(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError>;
    fn draw (&self);
    fn uninitialise(&mut self);
    fn is_initialised(&self) -> bool;
}
