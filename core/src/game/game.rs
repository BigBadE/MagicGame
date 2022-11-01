use std::time::Instant;
use renderer::window::Window;
use crate::resources::resources::ResourceManager;
use crate::shaders::shaders::ShaderDatabase;
use crate::util::random::Random;
use crate::world::world::World;

pub struct Game {
    pub world: World,
    pub next_tick: Instant,
    pub resource_manager: ResourceManager,
    pub shaders: ShaderDatabase
}

impl Game {
    pub fn new(random: Random, window: &Window) -> Self {
        return Game {
            world: World::new(random),
            next_tick: Instant::now(),
            resource_manager: ResourceManager::new(),
            shaders: ShaderDatabase::new(&window.display)
        };
    }
}