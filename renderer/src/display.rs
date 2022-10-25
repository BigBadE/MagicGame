use anyhow::Error;
use glium::{Display, DrawParameters, Frame, Program, Surface, VertexBuffer};
use glium::texture::RawImage2d;
use glium::uniforms::{EmptyUniforms, Uniforms, UniformsStorage};
use glium::uniforms::UniformValue::SrgbTexture2d;
use crate::util::Vertex;

pub struct GameDisplay {
    display: Display
}

impl GameDisplay {
    pub fn new(display: Display) -> Self {
        return GameDisplay {
            display
        }
    }

    pub fn start_frame(&mut self) -> GameFrame {
        return GameFrame::new(self.display.draw());
    }

    pub fn get_shader(&self, vertex_shader: &str, fragment_shader: &str) -> Result<Shader, Error> {
        let program = match Program::from_source(&self.display, vertex_shader, fragment_shader, None) {
            Ok(program) => program,
            Err(error) => return Err(Error::new(error))
        };
        return Ok(Shader {
            program
        });
    }
}

pub struct Shader {
    program: Program
}

pub struct InputUniforms<T> where T : Uniforms {
    uniform: T
}

pub struct GameFrame {
    frame: Frame
}

impl GameFrame {
    pub fn new(frame: Frame) -> Self {
        return GameFrame {
            frame
        };
    }

    pub fn clear(&mut self) {
        self.frame.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn draw(&mut self, display: &GameDisplay, vertexes: &Vec<Vertex>, indices: &[u32],
                shader: &Shader, image: Vec<u16>) {
        let dimensions = (image.len() as u32, image.len() as u32);
        let image = RawImage2d::from_raw_rgb(image, dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&display.display, image).unwrap();
        let uniform = uniform! {
            tex: texture
        };
        self.frame.draw(&VertexBuffer::new(&display.display, &vertexes).unwrap(),
                        &glium::IndexBuffer::new(&display.display,
                                                 glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
                        &shader.program, &uniform, &DrawParameters::default()).unwrap();
    }

    pub fn end_frame(self) {
        match self.frame.finish() {
            Err(error) => println!("Error ending frame: {}", error),
            Ok(_) => {}
        }
    }
}