#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2]
}

impl Vertex {
    pub fn new(position: [f32; 2]) -> Self {
        return Vertex {
            position,
            tex_coords: [0.0, 0.0]
        }
    }

    pub fn new_coord(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        return Vertex {
            position,
            tex_coords
        }
    }
}

implement_vertex!(Vertex, position, tex_coords);