#![allow(dead_code)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::{vertex, RenderError, Vertex};
use crate::renderable::Renderable;
#[derive(PartialEq)]

pub struct NullableRenderable<T: Vertex> {
    initialised: Rc<RefCell<bool>>,
    vertices: Rc<RefCell<Vec<T>>>,
    indices: Rc<RefCell<Option<Vec<u32>>>>,
    was_drawn: Rc<RefCell<bool>>,

    _phantom: PhantomData<T>
}


impl<T: Vertex> NullableRenderable<T> {
    pub fn new<F: Vertex>(
        initialised: Rc<RefCell<bool>>,
        vertices: Rc<RefCell<Vec<T>>>,
        indices: Rc<RefCell<Option<Vec<u32>>>>,
        was_drawn: Rc<RefCell<bool>>
    ) -> Self {
        Self {
            initialised,
            vertices,
            indices,
            was_drawn,

            _phantom: PhantomData
        }
    }
}

impl<T: Vertex> Drop for NullableRenderable<T> {
    fn drop(&mut self) {
        self.uninitialise();
    }
}

impl<T: Vertex> Renderable<T> for NullableRenderable<T> {

    fn initialise(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        self.initialised.replace(true);

        self.update_data(vertices, indices)
    }

    fn update_data(&mut self, vertices: &Vec<T>, indices: Option<&Vec<u32>>) -> Result<(), RenderError> {
        if !*self.initialised.borrow() {
            return Err(RenderError::RenderableError { error: "Tried to set data for a renderable that isn't initialised!".to_string() })
        }

        self.vertices.borrow_mut().clear();

        for vertex in vertices {
            self.vertices.borrow_mut().push(*vertex);
        }

        match indices {
            None => {
                self.indices.replace(None);
            },
            Some(index_data) => {
                let mut index_vec = vec![];

                for index in index_data {
                    index_vec.push(*index)
                }

                self.indices.replace(Some(index_vec));
            }
        }

        Ok(())
    }

    fn draw (&self) {
        self.was_drawn.replace(true);
    }

    fn uninitialise(&mut self) {
        self.initialised.replace(false);
    }

    fn is_initialised(&self) -> bool {
        self.initialised.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::Vertex2d;
    use crate::renderable::Renderable;
    use super::*;

    #[test]
    fn sets_initialised_on_initialise() {
        let initialised = Rc::new(RefCell::new(false));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            initialised.clone(),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(false))
        );

        nullable_renderable.initialise(&vec![], None).unwrap();

        assert_eq!(true, *initialised.borrow());
        assert_eq!(true, nullable_renderable.is_initialised());
    }

    #[test]
    fn sets_uninitialised_on_uninitialise() {
        let initialised = Rc::new(RefCell::new(true));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            initialised.clone(),
            Rc::new(RefCell::new(vec![])),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(false))
        );

        nullable_renderable.uninitialise();

        assert_eq!(false, *initialised.borrow());
        assert_eq!(false, nullable_renderable.is_initialised());
    }

    #[test]
    fn sets_vertices_on_initialise() {
        let expected_vertices = vec![
            Vertex2d { x: 1.0,  y: 2.0,  u: 3.0,  v: 4.0 },
            Vertex2d { x: 5.0,  y: 6.0,  u: 7.0,  v: 8.0 },
            Vertex2d { x: 9.0,  y: 10.0, u: 11.0, v: 12.0},
        ];
        let vertices = Rc::new(RefCell::new(vec![]));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            vertices.clone(),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(false))
        );

        nullable_renderable.initialise(&expected_vertices, None).unwrap();

        assert_eq!(expected_vertices, *vertices.borrow());
    }

    #[test]
    fn sets_vertices_on_update_data() {
        let expected_vertices = vec![
            Vertex2d { x: 1.0,  y: 2.0,  u: 3.0,  v: 4.0 },
            Vertex2d { x: 5.0,  y: 6.0,  u: 7.0,  v: 8.0 },
            Vertex2d { x: 9.0,  y: 10.0, u: 11.0, v: 12.0},
        ];
        let vertices = Rc::new(RefCell::new(vec![]));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            vertices.clone(),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(false))
        );

        nullable_renderable.initialise(&vec![], None).unwrap();
        nullable_renderable.update_data(&expected_vertices, None).unwrap();

        assert_eq!(expected_vertices, *vertices.borrow());
    }

    #[test]
    fn update_data_returns_err_when_not_initialised() {
        let expected_vertices = vec![
            Vertex2d { x: 1.0,  y: 2.0,  u: 3.0,  v: 4.0 },
            Vertex2d { x: 5.0,  y: 6.0,  u: 7.0,  v: 8.0 },
            Vertex2d { x: 9.0,  y: 10.0, u: 11.0, v: 12.0},
        ];
        let vertices = Rc::new(RefCell::new(vec![]));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            vertices.clone(),
            Rc::new(RefCell::new(None)),
            Rc::new(RefCell::new(false))
        );

        assert!(matches!(nullable_renderable.update_data(&expected_vertices, None), Err {..}));
    }


    #[test]
    fn sets_indices_on_initialise() {
        let expected_vertices = vec![1, 2, 3, 4, 5, 6];
        let indices = Rc::new(RefCell::new(None));
        let mut nullable_renderable = NullableRenderable::<Vertex2d>::new::<Vertex2d>(
            Rc::new(RefCell::new(false)),
            Rc::new(RefCell::new(vec![])),
            indices.clone(),
            Rc::new(RefCell::new(false))
        );

        nullable_renderable.initialise(&vec![], Some(&expected_vertices)).unwrap();

        assert_eq!(expected_vertices, *indices.borrow().clone().unwrap());
    }
}
