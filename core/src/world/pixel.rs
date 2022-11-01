use macros::JsonResource;
use json::JsonValue;
use crate::resources::resources::JsonResource;

const RAND_SEED_BITS: u8 = 0xC0;
const PIXEL_TYPE_BITS: u8 = 0x3F;

#[derive(Copy, Clone)]
pub struct Pixel {
    //6 bits of id + 2 bits of random seed
    pub flags: u8,
}

impl Pixel {
    pub fn new(seed: u8) -> Self {
        return Pixel {
            flags: 0 + seed & 0xC0
        };
    }

    pub fn set_type(&mut self, pixel_type: PixelType) {
        self.flags = pixel_type.set_type(self.flags);
    }

    pub fn check_type(&self, pixel_type: PixelType) -> bool {
        return pixel_type.is_type(self.flags);
    }
}

#[derive(JsonResource)]
pub struct PixelType {
    #[ignore_field]
    pub flags: u8,
}

impl JsonResource for PixelType {}

impl PixelType {
    pub fn new(value: &JsonValue, index: usize) -> Self {
        let temp = PixelType {
            flags: index as u8
        };

        if temp.flags & RAND_SEED_BITS != 0 {
            panic!("Pixel over {}, too many pixels registered.", RAND_SEED_BITS);
        }

        __load(&temp, value);

        return temp;
    }

    pub fn set_type(&self, flags: u8) -> u8 {
        return flags & RAND_SEED_BITS + self.flags;
    }

    pub fn is_type(&self, flags: u8) -> bool {
        return flags & PIXEL_TYPE_BITS == self.flags;
    }
}