use std::cell::RefCell;
use std::collections::HashMap;
use renderer::util::VectorInt;
use crate::world::chunk::Chunk;

pub struct World {
    chunks: HashMap<VectorInt, RefCell<Chunk>>,
}

impl World {
    pub fn new() -> Self {
        return World {
            chunks: HashMap::with_capacity(25)
        };
    }

    fn try_unload(&mut self) {
        todo!()
    }

    pub fn get_chunk(&mut self, position: VectorInt) -> &RefCell<Chunk> {
        if self.chunks.contains_key(&position) {
            return self.chunks.get(&position).expect("Couldn't find chunk!");
        }

        self.try_unload();
        self.chunks.insert(position, RefCell::new(Chunk::new()));
        return &self.chunks[&position];
    }
}