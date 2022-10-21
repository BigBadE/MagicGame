use crate::world::world::World;

pub struct Game {
    pub world: World
}

impl Game {
    pub fn new() -> Self {
        return Game {
            world: World::new()
        };
    }
}