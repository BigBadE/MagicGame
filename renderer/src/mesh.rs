use std::fmt::{Display, Formatter};
use crate::display::{GameDisplay, GameFrame, Shader};
use crate::util::Vertex;
use macros::JsonResource;
use json::JsonValue;

pub struct Mesh {
    vertexes: Vec<Vertex>,
    indices: Vec<u32>,
    texture: Vec<u8>,
    tex_size: (u32, u32),
}

impl Mesh {
    pub fn new(tex_size: (u32, u32)) -> Self {
        return Mesh {
            vertexes: Vec::new(),
            indices: Vec::new(),
            texture: vec![0; (tex_size.0 * tex_size.1 * 3) as usize],
            tex_size
        }
    }

    pub fn add_cube(&mut self, position: (f32, f32), size: (f32, f32)) {
        let offset = self.vertexes.len() as u32;
        self.vertexes.push(Vertex::new([position.0, position.1]));
        self.vertexes.push(Vertex::new([position.0 + size.0, position.1]));
        self.vertexes.push(Vertex::new([position.0, position.1 + size.1]));
        self.vertexes.push(Vertex::new([position.0 + size.0, position.1 + size.1]));
        self.indices.push(offset + 0);
        self.indices.push(offset + 1);
        self.indices.push(offset + 2);
        self.indices.push(offset + 2);
        self.indices.push(offset + 1);
        self.indices.push(offset + 3);
    }

    pub fn draw(&mut self, display: &GameDisplay, frame: &mut GameFrame, shader: &Shader) {
        let mut output = vec![0; self.texture.len()];
        output.copy_from_slice(self.texture.as_slice());
        frame.draw(&display, &self.vertexes, &self.indices, &shader, output, self.tex_size);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let start = x * 3 * self.tex_size.0 as usize + y;
        self.texture[start] = color.r;
        self.texture[start + 1] = color.g;
        self.texture[start + 2] = color.b;
    }

    pub fn get_color(&mut self, x: usize, y: usize) -> Color {
        let start = x * 3 * self.tex_size.0 as usize + y;
        return Color {
            r: self.texture[start],
            g: self.texture[start+1],
            b: self.texture[start+2]
        };
    }

    // Starts from top right
    pub fn swap_color(&mut self, first: usize, second: usize) {
        self.texture.swap(first, second);
        self.texture.swap(first+1, second+1);
        self.texture.swap(first+2, second+2);
    }
}

#[derive(JsonResource, Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {}, {})", self.r, self.g, self.b);
    }
}

impl Color {
    pub fn default() -> Self {
        return Color {
            r: 0,
            g: 0,
            b: 0
        };
    }

    pub fn from(color: (u8, u8, u8)) -> Color {
        return Color {
            r: color.0,
            g: color.1,
            b: color.2
        }
    }

    pub fn new(value: &JsonValue) -> Self {
        let mut temp = Color::default();
        __load_Color(&mut temp, &value);
        return temp;
    }
}