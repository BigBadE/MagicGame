use std::collections::HashMap;
use renderer::display::{GameDisplay, Shader};

pub struct ShaderDatabase {
    shaders: HashMap<&'static str, Shader>
}

impl ShaderDatabase {
    pub fn new(display: &GameDisplay) -> ShaderDatabase {
        let mut shaders = ShaderDatabase {
            shaders: HashMap::new()
        };

        shaders.try_add_shader(&display, "standard",
                               include_str!("standard.vert"), include_str!("standard.frag"));

        return shaders;
    }

    fn try_add_shader(&mut self, display: &GameDisplay, name: &'static str, vert: &str, frag: &str) {
        match display.get_shader(vert, frag) {
            Ok(shader) => self.shaders.insert(name, shader),
            Err(error) => {
                println!("Error compiling shader:\n{}", error);
                Option::None
            },
        };
    }

    pub fn get_shader(&self, name: &str) -> &Shader {
        return self.shaders.get(name).unwrap();
    }
}