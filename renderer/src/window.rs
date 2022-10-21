use glium::{Display, glutin};
use glium::glutin::event::Event;
use glium::glutin::event_loop::ControlFlow;
use crate::display::GameDisplay;

pub struct Window {
    pub display: GameDisplay
}

impl Window {
    pub fn start(callback: fn(&mut Window)) -> ! {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        let mut window = Window {
            display: GameDisplay::new(display)
        };

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        event_loop.run(move |event, _target, control_flow| {
            callback(&mut window);

            *control_flow = ControlFlow::WaitUntil(next_frame_time);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => (),
            }
        });
    }
}