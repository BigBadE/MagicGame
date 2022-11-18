use std::cell::RefCell;
use std::collections::HashMap;
use renderer::display::GameDisplay;
use crate::util::random::Random;
use crate::world::chunk::Chunk;
use crate::world::generation::generator::Generator;

pub struct World {
    pub chunks: HashMap<(i32, i32), RefCell<Chunk>>,
    pub generator: Generator,
    pub random: Random,
}

impl World {
    pub fn new(random: Random) -> Self {
        return World {
            chunks: HashMap::with_capacity(25),
            generator: Generator::new(),
            random,
        };
    }

    pub fn update(&mut self, display: &GameDisplay) {
        let size = ((display.size.0 / 512 + 1) as i32, (display.size.1 / 512 + 1) as i32);
        for x in -size.0..=size.0 {
            for y in -size.1..=size.1 {
                if !self.chunks.contains_key(&(x, y)) {
                    self.load_chunk(display, (x, y));
                }
            }
        }
    }

    pub fn load_chunk(&mut self, display: &GameDisplay, position: (i32, i32)) {
        let chunk = Chunk::new(&mut self.random, display.chunk_size, position);
        self.generator.generate(&chunk);
        self.chunks.insert(position, RefCell::new(chunk));
    }

    pub fn get_chunk(&self, position: (i32, i32)) -> Option<&RefCell<Chunk>> {
        return self.chunks.get(&position);
    }
}