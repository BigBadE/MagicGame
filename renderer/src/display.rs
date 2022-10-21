use glium::{Display, Frame, Surface};

pub struct GameDisplay {
    display: Display
}

impl GameDisplay {
    pub fn new(display: Display) -> Self {
        return GameDisplay {
            display
        }
    }

    pub fn start_frame(&mut self) -> GameFrame {
        return GameFrame::new(self.display.draw());
    }
}

pub struct GameFrame {
    frame: Frame
}

impl GameFrame {
    pub fn new(frame: Frame) -> Self {
        return GameFrame {
            frame
        };
    }

    pub fn clear(&mut self) {
        self.frame.clear_color(0.0, 0.0, 1.0, 1.0);
    }

    pub fn end_frame(self) {
        match self.frame.finish() {
            Err(error) => println!("Error ending frame: {}", error),
            Ok(_) => {}
        }
    }
}