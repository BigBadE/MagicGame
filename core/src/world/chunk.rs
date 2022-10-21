use std::collections::HashMap;
use renderer::util::VectorInt;
use crate::PixelType;
use crate::util::random::Random;
use crate::world::pixel::Pixel;
use crate::world::world::World;

pub struct Chunk<'a> {
    pub pixels: [Pixel; 512*512],
    pub active: HashMap<(u16, u16), &'a Pixel>
}

impl<'a> Chunk<'a> {
    pub fn new(rand: &mut Random) -> Self {
        return Chunk {
            pixels: [Pixel::new(rand.next_u8()); 512*512],
            active: HashMap::new()
        };
    }

    pub fn set_pixel_type(&mut self, x: usize, y: usize, pixel_type: PixelType) {
        let pixel = &mut self.pixels[x * 512 + y];
        pixel.set_type(pixel_type);
        self.active.insert((x as u16, y as u16), pixel);
    }

    pub fn physics_tick(&mut self, world: &mut World) {
        for (position, pixel) in self.active {
            if self.pixels[position.0 * 512 + position.1 - 1].check_type(PixelType::AIR) {
                self.pixels.swap((position.0 * 512 + position.1 - 1) as usize,
                                 (position.0 * 512 + position.1) as usize);
            }
        }
    }
}