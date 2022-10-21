use glium::glutin::event::{KeyboardInput, MouseButton};

pub struct KeyInput {
    input: KeyboardInput
}

impl KeyInput {
    pub fn new(input: KeyboardInput) -> Self {
        return KeyInput {
            input
        }
    }
}

pub struct MouseInput {
    button: MouseButton
}

impl MouseInput {
    pub fn new(button: MouseButton) -> Self {
        return MouseInput {
            button
        }
    }
}