#![allow(dead_code)]
use crate::{Vertex, RenderError};


pub struct MockRenderable {
    initialised: bool
}

impl Drop for MockRenderable {
    fn drop(&mut self) {
        self.uninitialise();
    }
}

impl MockRenderable {
    pub fn new_uninitialised () -> MockRenderable {
        MockRenderable { initialised: false}
    }

    pub fn new_initialised<T: 'static + Vertex>(_vertices: &Vec<T>, _indices: Option<&Vec<u32>>) -> Result<MockRenderable, RenderError> {
        Ok( MockRenderable { initialised: true} )
    }

    pub fn initialise<T: 'static + Vertex>(&mut self, _vertices: &Vec<T>, _indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        self.initialised = true;
        Ok(()) //If I'm a real thinker then there should be no way for an uncaught invalid state to exist.
    }

    pub fn update_data<T: 'static>(&mut self, _vertices: &Vec<T>, _indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        Ok(())
    }

    pub fn draw (&self) {
    }

    pub fn uninitialise(&mut self) {
        self.initialised = false
    }

    pub fn is_initialised(&self) -> bool {
        self.initialised
    }
}
