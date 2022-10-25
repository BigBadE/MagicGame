use std::cell::RefCell;
use std::collections::HashMap;
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

    fn try_unload(&mut self) {
        todo!();
    }

    pub fn load_chunk(&mut self, position: (i32, i32)) {
        self.try_unload();
        self.chunks.insert(position, RefCell::new(Chunk::new(&mut self.random, position)));
    }

    pub fn get_chunk(&self, position: (i32, i32)) -> &RefCell<Chunk> {
        if self.chunks.contains_key(&position) {
            return &self.chunks.get(&position).expect("Couldn't find chunk!");
        }
        panic!("Tried to get unloaded chunk at ({}, {})", position.0, position.1);
    }
}