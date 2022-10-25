use crate::util::random::Random;
use crate::world::pixel::{Pixel, PixelType};
use crate::world::world::World;

pub struct Chunk {
    pub position: (i32, i32),
    pub pixels: [Pixel; 512 * 512],
    pub active: Vec<(u16, u16)>,
    pub texture: Vec<u8>
}

impl Chunk {
    pub fn new(random: &mut Random, position: (i32, i32)) -> Self {
        return Chunk {
            position,
            pixels: [Pixel::new(random.next_u8()); 512 * 512],
            active: Vec::new(),
            texture: Vec::from([0; 512*512*3])
        };
    }

    pub fn set_pixel_type(&mut self, x: usize, y: usize, pixel_type: PixelType) {
        let pixel = &mut self.pixels[x * 512 + y];
        pixel.set_type(pixel_type);
        self.active.push((x as u16, y as u16));
    }

    pub fn physics_tick(&mut self, world: &World) {
        for position in self.active.as_slice() {
            if position.1 == 0 {
                let mut chunk = world.get_chunk((self.position.0, self.position.1)).borrow_mut();
                let temp = chunk.pixels[(position.0 * 512 + 511) as usize];
                chunk.pixels[(position.0 * 512 + 511) as usize] = self.pixels[(position.0 * 512) as usize];
                self.pixels[(position.0 * 512) as usize] = temp;
            }

            if self.pixels[(position.0 * 512 + position.1 - 1) as usize].check_type(PixelType::AIR) {
                self.pixels.swap((position.0 * 512 + position.1 - 1) as usize,
                                 (position.0 * 512 + position.1) as usize);
            }
        }
    }
}