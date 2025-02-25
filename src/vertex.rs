use std::fmt::Debug;

pub trait Vertex: Copy + Clone + Debug + PartialEq {
    
    fn initialise_attrib_ptrs ();
}
