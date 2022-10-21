use renderer::window::Window;

fn main() {
    Window::start(game_loop);
}

fn game_loop(window: &mut Window) {
    let mut frame = window.display.start_frame();
    frame.clear();

    frame.end_frame();
}