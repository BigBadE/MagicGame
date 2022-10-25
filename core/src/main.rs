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

fn main() {
    let random = Random::new(1);
    Window::start(Game::new(random), game_loop);
}

fn game_loop(game: &mut Game, window: &mut Window) {
    let mut frame = window.display.start_frame();
    frame.clear();

    let background = vec![Vertex { position: [-1.0, -1.0] }, Vertex { position: [1.0, -1.0] },
                          Vertex { position: [1.0, 1.0] }, Vertex { position: [-1.0, 1.0] }];
    Physics::physics_tick(game);

    while !window.mouse_input.is_empty() {
        let position = window.cursor;
        let clicking = game.world.get_chunk((position.0 as i32 >> 9, position.1 as i32 >> 9));
        clicking.borrow_mut().set_pixel_type(position.0 as usize % 512, position.1 as usize % 512, PixelType::SAND);
        window.mouse_input.remove(0);
    }
    frame.end_frame();
}