use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

pub struct UI {
    pub sdl: sdl2::Sdl,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub run: bool,
}

impl UI {
    pub fn new(title: &str) -> Self {
        let sdl = sdl2::init().expect("Error: can't init sdl.");
        let canvas = sdl
            .video()
            .expect("Error: can't init video.")
            .window(title, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .expect("Error: can't create window.")
            .into_canvas()
            .build()
            .expect("Error: can't create canvas.");
        return UI { sdl, canvas, run: true };
    }

    pub fn event_handler(&mut self, event: Event) {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => self.run = false,
            _ => {} // otherwhise do nothing
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

