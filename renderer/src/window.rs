use glium::{Display, glutin};
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use crate::display::GameDisplay;
use crate::input::{KeyInput, MouseInput};

pub struct Window {
    pub display: GameDisplay,
    pub cursor: (f64, f64),
    pub key_presses: Vec<KeyInput>,
    pub mouse_input: Vec<MouseInput>
}

impl Window {
    pub fn start<T: 'static>(mut context: fn(&Window) -> T, callback: fn(&mut T, &mut Window)) -> ! {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        let mut window = Window {
            display: GameDisplay::new(display),
            cursor: (0.0, 0.0),
            key_presses: Vec::new(),
            mouse_input: Vec::new()
        };

        let mut context = context(&window);
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        event_loop.run(move |event, _target, control_flow| {
            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    glutin::event::WindowEvent::KeyboardInput { input, is_synthetic, .. } => {
                        if !is_synthetic {
                            window.key_presses.push(KeyInput::new(input))
                        }
                    }
                    glutin::event::WindowEvent::MouseInput { button, .. } => {
                        window.mouse_input.push(MouseInput::new(button))
                    }
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        window.cursor = (position.x, position.y);
                    }
                    _ => return,
                },
                Event::MainEventsCleared => {
                    callback(&mut context, &mut window);
                }
                _ => (),
            }
        });
    }
}