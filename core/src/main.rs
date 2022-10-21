use renderer::util::VectorInt;
use renderer::window::Window;
use crate::game::game::Game;

mod game;
mod world;

fn main() {
    Window::start(Game::new(), game_loop);
}

fn game_loop(game: &mut Game, window: &mut Window) {
    let mut frame = window.display.start_frame();
    frame.clear();

    while !window.mouse_input.is_empty() {
        game.world.get_chunk(VectorInt { position: [
            window.cursor.position[0] as i32 >> 9,
            window.cursor.position[1] as i32 >> 9
        ] });
        window.mouse_input.remove(0);
    }
    frame.end_frame();
}