use std::borrow::Borrow;
use std::path::Iter;
use renderer::window::Window;
use core::game::game::Game;
use core::physics::physics::Physics;
use core::util::random::Random;

fn main() {
    Window::start(setup, game_loop);
}

fn setup(window: Window) -> Game {
    let random = Random::new(1);
    return Game::new(random, window);
}

fn game_loop(game: &mut Game) {
    Physics::physics_tick(game);

    let window = &mut game.window;
    let mut frame = window.display.start_frame();
    frame.clear();

    game.world.update(&window.display);

    for chunk in game.world.chunks.values() {
        chunk.borrow_mut().mesh.draw(&window.display, &mut frame, &game.shaders.get_shader("standard"));
    }

    if game.drawing {
        let position = (-(1.0 - window.cursor.0 * 2.0 / window.display.size.0 as f64).clamp(-1.0, 1.0),
                        (1.0 - window.cursor.1 * 2.0 / window.display.size.1 as f64).clamp(-1.0, 1.0));
        let clicking =
            game.world.get_chunk(((position.0 / window.display.chunk_size.0).floor() as i32,
                                  (position.1 / window.display.chunk_size.1).floor() as i32));

        let mut sand_pos = ((position.0 % window.display.chunk_size.0 / window.display.chunk_size.0) * 512.0,
                        (position.1 % window.display.chunk_size.1 / window.display.chunk_size.1) * 512.0);
        if sand_pos.0 < 0.0 {
            sand_pos = (512.0 + sand_pos.0, sand_pos.1);
        }
        if sand_pos.1 < 0.0 {
            sand_pos = (sand_pos.0, 512.0 + sand_pos.1);
        }

        clicking.unwrap().borrow_mut().set_pixel_type(
            sand_pos.0 as usize, sand_pos.1 as usize,
            game.resource_manager.get_type("pixel", "sand"));
    }

    frame.end_frame();
}