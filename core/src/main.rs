use anyhow::Error;
use renderer::util::Vertex;
use renderer::window::Window;
use crate::game::game::Game;
use crate::physics::physics::Physics;
use crate::util::random::Random;
use crate::world::pixel::PixelType;

mod game;
mod world;
mod physics;
mod util;
mod shaders;

fn main() {
    Window::start(setup, game_loop);
}

fn setup(window: &Window) -> Game {
    let random = Random::new(1);
    return Game::new(random, window);
}

fn game_loop(game: &mut Game, window: &mut Window) {
    let mut frame = window.display.start_frame();
    frame.clear();

    let background = vec![
        Vertex { position: [-1.0, -1.0] }, Vertex { position: [1.0, -1.0] }, Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] }];
    let indices = [0, 1, 2, 2, 3, 1];
    frame.draw(&window.display, &background, &indices, &game.shaders.get_shader("standard"));

    Physics::physics_tick(game);

    while !window.mouse_input.is_empty() {
        let position = window.cursor;
        let clicking = game.world.get_chunk((position.0 as i32 >> 9, position.1 as i32 >> 9));
        clicking.borrow_mut().set_pixel_type(position.0 as usize % 512, position.1 as usize % 512, PixelType::SAND);
        window.mouse_input.remove(0);
    }
    frame.end_frame();
}