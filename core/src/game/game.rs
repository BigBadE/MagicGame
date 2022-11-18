use std::time::Instant;
use renderer::input::{KeyInput, MouseInput};
use renderer::window::{Context, Window};
use crate::resources::resources::ResourceManager;
use crate::shaders::shaders::ShaderDatabase;
use crate::util::random::Random;
use crate::world::world::World;

pub struct Game {
    pub window: Window,
    pub world: World,
    pub next_tick: Instant,
    pub resource_manager: ResourceManager,
    pub shaders: ShaderDatabase,
    pub drawing: bool
}

impl Game {
    pub fn new(random: Random, window: Window) -> Self {
        let shaders = ShaderDatabase::new(&window.display);
        return Game {
            window,
            world: World::new(random),
            next_tick: Instant::now(),
            resource_manager: ResourceManager::new(),
            shaders,
            drawing: false
        };
    }

    pub fn update(&mut self) {

    }
}

impl Context for Game {
    fn resize(&mut self, size: (u32, u32)) {
        self.window.display.resize(size);

        for chunk in self.world.chunks.values() {
                chunk.borrow_mut().resize((self.window.display.chunk_size.0 as f32,
                                           self.window.display.chunk_size.1 as f32));
        }
    }

    fn key_input(&mut self, input: KeyInput) {
        todo!()
    }

    fn mouse_input(&mut self, input: MouseInput) {
        if input.pressed {
            self.drawing = true;
        } else {
            self.drawing = false;
        }
    }

    fn cursor_move(&mut self, position: (f64, f64)) {
        self.window.cursor = position;
    }
}