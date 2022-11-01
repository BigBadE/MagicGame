use macros::{json_implement, JsonResource};
use json::JsonValue;
use crate::resources::resources::JsonResource;

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
    #[require]
    pub flags: u8,
}

impl PixelType {
    pub fn new(value: JsonValue) -> Self {
        let temp = PixelType {
            flags: 0
        };

        __load(&temp, value);
        return temp;
    }

    pub fn set_type(&self, flags: u8) -> u8 {
        return flags & 0xC0 + self.flags;
    }

    pub fn is_type(&self, flags: u8) -> bool {
        return flags & 0x3F == self.flags;
    }
}