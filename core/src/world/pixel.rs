use macros::JsonResource;
use json::JsonValue;
use renderer::mesh::Color;
use crate::resources::resources::JsonResource;

const RAND_SEED_BITS: u16 = 0xC000;
const PIXEL_TYPE_BITS: u16 = 0x0FFF;
const PHYSICS_BITS: u16 = 0x3000;

#[derive(Copy, Clone)]
pub struct Pixel {
    //2 bits of random seed + 2 bits of physics logic + 12 bits of id
    pub flags: u16,
}

impl Pixel {
    pub fn new(seed: u8) -> Self {
        return Pixel {
            flags: 0 + ((seed as u16) << 4 & RAND_SEED_BITS)
        };
    }
    
    pub fn set_type(&mut self, pixel_type: &PixelType) {
        self.flags = pixel_type.set_type(self.flags);
    }

    pub fn check_type(&self, pixel_type: &PixelType) -> bool {
        return pixel_type.is_type(self.flags);
    }

    pub fn is_air(&self) -> bool {
        return self.flags & PIXEL_TYPE_BITS == 0;
    }
}

#[derive(JsonResource)]
pub struct PixelType {
    #[ignore_field]
    pub flags: u16,
    pub color: Color
}

impl JsonResource for PixelType {}

impl PixelType {
    pub fn new(value: &JsonValue, index: usize) -> Self {
        let mut temp = PixelType {
            flags: index as u16,
            color: Color::default(),
        };

        if temp.flags & PHYSICS_BITS != 0 {
            panic!("Pixel over {}, too many pixels registered.", RAND_SEED_BITS);
        }

        __load_PixelType(&mut temp, value);

        return temp;
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }
    
    pub fn set_type(&self, flags: u16) -> u16 {
        return flags & RAND_SEED_BITS + self.flags;
    }

    pub fn is_type(&self, flags: u16) -> bool {
        return flags & PIXEL_TYPE_BITS == self.flags;
    }
}