const A: u32 = 69069;

pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        return Random {
            seed
        };
    }

    pub fn push(&mut self) -> Random {
        let output = self.next();
        self.seed = self.next();
        return Random {
            seed: output
        }
    }

    pub fn next(&mut self) -> u32 {
        let next = self.seed;
        self.seed = u32::wrapping_mul(next, A) + 1;
        return next;
    }

    pub fn next_u8(&mut self) -> u8 {
        return self.next() as u8;
    }
}