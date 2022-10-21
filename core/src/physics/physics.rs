use std::time::{Duration, Instant};
use crate::Game;

pub struct Physics {

}

impl Physics {
    pub fn physics_tick(game: &mut Game) {
        game.next_tick = Instant::now() + Duration::from_nanos(33_333_333);

        for chunk in game.world.chunks.values() {
            chunk.borrow_mut().physics_tick(&mut game.world);
        }
    }
}