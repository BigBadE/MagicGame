const A: u32 = 48271;

pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        return Random {
            seed
        };
    }

    pub fn next(&mut self) -> u32 {
        let next = self.seed;
        self.seed = u32::wrapping_mul(next, A);
        return next;
    }

    pub fn next_u8(&mut self) -> u8 {
        return self.next() as u8;
    }
}