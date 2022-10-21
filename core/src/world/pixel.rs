
#[derive(Copy, Clone)]
pub struct Pixel {
    //6 bits of id + 2 bits of random seed
    pub flags: u8
}

impl Pixel {
    pub fn new(seed: u8) -> Self {
        return Pixel {
            flags: 0 + seed & 0xC0
        }
    }

    pub fn set_type(&mut self, pixel_type: PixelType) {
        self.flags = self.flags & 0xC0 + pixel_type as u8;
    }

    pub fn check_type(&self, pixel_type: PixelType) -> bool {
        return self.flags & 0x3F == pixel_type as u8;
    }
}

pub enum PixelType {
    AIR = 0,
    SAND = 0x01
}