use std::time::Instant;
use crate::util::random::Random;
use crate::world::world::World;

pub struct Game {
    pub world: World,
    pub next_tick: Instant
}

impl Game {
    pub fn new(random: Random) -> Self {
        return Game {
            world: World::new(random),
            next_tick: Instant::now()
        };
    }
}