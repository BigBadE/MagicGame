use crate::PixelType;
use crate::util::random::Random;
use crate::world::pixel::Pixel;
use crate::world::world::World;

pub struct Chunk {
    pub pixels: [Pixel; 512*512],
    pub active: Vec<(u16, u16)>
}

impl Chunk {
    pub fn new(rand: &mut Random) -> Self {
        return Chunk {
            pixels: [Pixel::new(rand.next_u8()); 512*512],
            active: Vec::new()
        };
    }

    pub fn set_pixel_type(&mut self, x: usize, y: usize, pixel_type: PixelType) {
        let pixel = &mut self.pixels[x * 512 + y];
        pixel.set_type(pixel_type);
        self.active.push((x as u16, y as u16));
    }

    pub fn physics_tick(&mut self) {
        for position in self.active.as_slice() {
            if self.pixels[(position.0 * 512 + position.1 - 1) as usize].check_type(PixelType::AIR) {
                self.pixels.swap((position.0 * 512 + position.1 - 1) as usize,
                                 (position.0 * 512 + position.1) as usize);
            }
        }
    }
}