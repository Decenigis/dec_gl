#![allow(dead_code)]
use gl::types::GLuint;
use crate::{Vertex, RenderError};


pub enum Renderable {
    InitialisedWithIndexing { vao: GLuint, vbo: GLuint, ibo: GLuint, index_count: i32},
    Initialised { vao: GLuint, vbo: GLuint, vertex_count: i32,},
    Uninitialised
}

impl Drop for Renderable {
    fn drop(&mut self) {
        self.uninitialise();
    }
}

impl Renderable {
    pub fn new_uninitialised () -> Renderable {
        Renderable::Uninitialised {  }
    }

    pub fn new_initialised<T: 'static + Vertex>(vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<Renderable, RenderError> {
        let mut new_renderable = Renderable::new_uninitialised();
        match new_renderable.initialise(vertices, indices) {
            Err(e) => return Err(e),
            _ => {}
        }
        Ok(new_renderable)
    }

    pub fn initialise<T: 'static + Vertex>(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        self.uninitialise();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            if vao == 0 {
                return Err(RenderError::RenderableError { error: "Failed to create vertex array for the world map!".to_string()});
            }

            gl::GenBuffers(1, &mut vbo);
            if vbo == 0 {
                return Err(RenderError::RenderableError {
                    error: "Failed to create vertex buffer for the world map!".to_string(),
                });
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<T>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );
            gl::BindVertexArray(vao);
        }
        T::initialise_attrib_ptrs();

        match indices {
            None => *self = Renderable::Initialised { vao, vbo, vertex_count: vertices.len() as i32 },
            Some(index_data) => {
                let mut ibo = 0;
                unsafe {
                    gl::GenBuffers(1, &mut ibo);
                    if ibo == 0 {
                        return Err(RenderError::RenderableError {
                            error: "Failed to create index buffer for the world map!".to_string(),
                        });
                    }
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (index_data.len() * size_of::<GLuint>()) as isize,
                        index_data.as_ptr().cast(),
                        gl::STATIC_DRAW,
                    );
                }

                *self = Renderable::InitialisedWithIndexing { vao, vbo, ibo, index_count: index_data.len() as i32 }
            }

        }

        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        return Ok(()) //If I'm a real thinker then there should be no way for an uncaught invalid state to exist.
    }

    pub fn update_data<T: 'static>(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        match self {
            Self::Initialised { vbo, vertex_count, .. } => {
                *vertex_count = vertices.len() as i32;
                unsafe {
                    gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertices.len() * size_of::<T>()) as isize,
                        vertices.as_ptr().cast(),
                        gl::STATIC_DRAW,
                    );
                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                }
                Ok(())
            }
            Self::InitialisedWithIndexing { vbo, ibo, index_count, .. } => {
                match indices {
                    Some(index_data) => {
                        *index_count = index_data.len() as i32;
                        unsafe {
                            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
                            gl::BufferData(
                                gl::ARRAY_BUFFER,
                                (vertices.len() * size_of::<T>()) as isize,
                                vertices.as_ptr().cast(),
                                gl::STATIC_DRAW,
                            );
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ibo);
                            gl::BufferData(
                                gl::ELEMENT_ARRAY_BUFFER,
                                (index_data.len() * size_of::<GLuint>()) as isize,
                                index_data.as_ptr().cast(),
                                gl::STATIC_DRAW,
                            );
                            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
                        }
                    }
                    None => return Err(RenderError::RenderableError { error: "No index data provided to an indexed renderable!".to_string() })
                }
                Ok(())
            }
            _ => return Err(RenderError::RenderableError { error: "Tried to set data for a renderable that isn't initialised!".to_string() }),
        }
    }

    pub fn draw (&self) {
        match self {
            Self::Uninitialised {} => return,
            Self::Initialised { vao, vertex_count, .. } => unsafe {
                gl::BindVertexArray(*vao);
                gl::DrawArrays(gl::TRIANGLES, 0, *vertex_count);
                gl::BindVertexArray(0);
            }
            Self::InitialisedWithIndexing { vao, index_count, .. } => unsafe {
                gl::BindVertexArray(*vao);
                gl::DrawElements(gl::TRIANGLES,
                                 *index_count,
                                 gl::UNSIGNED_INT,
                                 0 as *const _ );
                gl::BindVertexArray(0);
            }

        }
    }

    pub fn uninitialise(&mut self) {
        match self {
            Renderable::Uninitialised {} => return,
            Renderable::Initialised { vao, vbo, .. } => unsafe {
                gl::DeleteVertexArrays(1, [*vao].as_ptr());
                gl::DeleteBuffers(1, [*vbo].as_ptr());
            }
            Renderable::InitialisedWithIndexing { vao, vbo, ibo, .. } => unsafe {
                gl::DeleteVertexArrays(1, [*vao].as_ptr());
                gl::DeleteBuffers(1, [*vbo].as_ptr());
                gl::DeleteBuffers(1, [*ibo].as_ptr());
            }
        }
    }

    pub fn is_initialised(&self) -> bool {
        match self {
            Self::Uninitialised => false,
            _ => true
        }
    }
}
