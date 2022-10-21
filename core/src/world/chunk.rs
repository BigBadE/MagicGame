use crate::world::pixel::Pixel;

pub struct Chunk {
    pub pixels: [Pixel; 512*512]
}

impl Chunk {
    pub fn new() -> Self {
        return Chunk {
            pixels: [Pixel::new(); 512*512]
        };
    }
}