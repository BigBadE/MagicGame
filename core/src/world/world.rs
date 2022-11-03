use std::cell::RefCell;
use std::collections::HashMap;
use renderer::display::GameDisplay;
use crate::util::random::Random;
use crate::world::chunk::Chunk;

pub struct World {
    pub chunks: HashMap<(i32, i32), RefCell<Chunk>>,
    pub random: Random,
}

impl World {
    pub fn new(random: Random) -> Self {
        return World {
            chunks: HashMap::with_capacity(25),
            random,
        };
    }

    pub fn update(&mut self, display: &GameDisplay) {
        let size = ((display.size.0 / 512 + 1) as i32, (display.size.1 / 512 + 1) as i32);
        for x in -size.0..=size.0 {
            for y in -size.1..=size.1 {
                if !self.chunks.contains_key(&(x, y)) {
                    self.load_chunk((x, y));
                }
            }
        }
    }

    pub fn load_chunk(&mut self, position: (i32, i32)) {
        self.chunks.insert(position, RefCell::new(Chunk::new(&mut self.random, position)));
    }

    pub fn get_chunk(&self, position: (i32, i32)) -> &RefCell<Chunk> {
        if self.chunks.contains_key(&position) {
            return &self.chunks.get(&position).expect("Couldn't find chunk!");
        }
        panic!("Tried to get unloaded chunk at ({}, {})", position.0, position.1);
    }
}