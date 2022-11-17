use glium::glutin::event;
use glium::glutin::event::KeyboardInput;

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
    pub button: MouseButton,
    pub pressed: bool
}

impl MouseInput {
    pub fn new(button: event::MouseButton, pressed: bool) -> Self {
        return MouseInput {
            button: match button {
                event::MouseButton::Left => MouseButton::Left,
                event::MouseButton::Right => MouseButton::Right,
                event::MouseButton::Middle => MouseButton::Middle,
                event::MouseButton::Other(val) => MouseButton::Other(val)
            },
            pressed
        }
    }
}

pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}