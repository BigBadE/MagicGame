use crate::display::{GameDisplay, GameFrame, Shader};
use crate::util::Vertex;
use macros::JsonResource;
use json::JsonValue;

pub struct Mesh {
    vertexes: Vec<Vertex>,
    indices: Vec<u32>,
    texture: Vec<u16>,
    tex_size: (usize, usize),
}

impl Mesh {
    pub fn new(tex_size: (usize, usize)) -> Self {
        return Mesh {
            vertexes: Vec::new(),
            indices: Vec::new(),
            texture: vec![0; tex_size.0*tex_size.1*3],
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
        let mut output = Vec::with_capacity(self.texture.len());
        output.copy_from_slice(self.texture.as_slice());
        frame.draw(&display, &self.vertexes, &self.indices, &shader, output);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Color) {
        let start = x * 3 * self.tex_size.0 + y;
        self.texture[start] = color.r
    }
}

#[derive(JsonResource, Copy, Clone)]
pub struct Color {
    r: u16,
    g: u16,
    b: u16,
    a: u16
}

impl Color {
    pub fn default() -> Self {
        return Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        };
    }
    pub fn new(value: &JsonValue) -> Self {
        let mut temp = Color::default();
        __load_Color(&mut temp, &value);
        return temp;
    }
}