use std::hash::Hash;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone, PartialEq)]
pub struct Vector {
    pub position: [f32; 2],
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VectorInt {
    pub position: [i32; 2],
}

implement_vertex!(Vertex, position);

