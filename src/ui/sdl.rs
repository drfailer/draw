use super::ui::UI;
use sdl2::{event::Event, keyboard::Keycode, rect::Rect};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub struct SdlUi {
    sdl: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    run: bool,
}

impl SdlUi {
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
        return SdlUi {
            sdl,
            canvas,
            run: true,
        };
    }

    fn event_handler(&mut self, event: Event) {
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

impl UI for SdlUi {
    fn handl_events(&mut self) {
        for event in self.sdl.event_pump().unwrap().poll_iter() {
            self.event_handler(event);
        }
    }

    fn clear(&mut self, color: crate::draw::color::Color) {
        self.set_color(color);
        self.canvas.clear();
    }

    fn set_color(&mut self, color: crate::draw::color::Color) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(color.r, color.g, color.b));
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.canvas
            .fill_rect(Rect::new(x, y, width, height))
            .unwrap();
    }

    fn draw_pixel(&mut self, x: i32, y: i32) {
        self.draw_rect(x, y, 1, 1);
    }

    fn run(&self) -> bool {
        return self.run;
    }

    fn height(&self) -> u32 {
        return HEIGHT;
    }

    fn width(&self) -> u32 {
        return WIDTH;
    }
}
