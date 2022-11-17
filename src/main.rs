use renderer::window::Window;
use core::game::game::Game;
use core::physics::physics::Physics;
use core::util::random::Random;

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

    Physics::physics_tick(game);

    game.world.update(&window.display);

    for chunk in game.world.chunks.values() {
        if window.resized {
            chunk.borrow_mut().resize((window.display.chunk_size.0 as f32, window.display.chunk_size.1 as f32));
        }

        chunk.borrow_mut().mesh.draw(&window.display, &mut frame, &game.shaders.get_shader("standard"));
    }

    while !window.mouse_input.is_empty() {
        let position = ((window.cursor.0 * window.display.chunk_size.0) as i32,
                        (window.cursor.1 * window.display.chunk_size.1) as i32);
        let clicking =
            game.world.get_chunk((position.0 / 512, position.1 / 512));
        println!("Spawning sand at ({}, {}) chunk ({}, {})", position.0 % 512,
                 position.1 % 512, position.0 / 512, position.1 / 512);
        clicking.unwrap().borrow_mut().set_pixel_type(
            (position.0 % 512) as usize, (position.1 % 512) as usize,
            game.resource_manager.get_type("pixel", "sand"));
        window.mouse_input.remove(0);
    }
    frame.end_frame();
}