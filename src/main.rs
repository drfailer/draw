extern crate sdl2;

mod sdl_ui;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl_ui::{coordinates, ui::{WIDTH, HEIGHT}};

pub fn main() {
    let mut ui = sdl_ui::ui::UI::new("Drawing");
    ui.canvas.set_draw_color(Color::RGB(0, 0, 0));
    ui.canvas.clear();
    ui.canvas.present();

    while ui.run {
        sdl_ui::draw::rectangle(
            &mut ui.canvas,
            coordinates::Vec2::Screen(20, 20),
            80,
            40,
            Color::RGB(255, 255, 255),
            false,
        );
        sdl_ui::draw::square(
            &mut ui.canvas,
            coordinates::Vec2::Screen(180, 180),
            40,
            Color::RGB(255, 0, 0),
            false,
        );
        for radius in (1..100).step_by(5) {
            sdl_ui::draw::circle(&mut ui.canvas, coordinates::Vec2::Screen(200, 200), radius, Color::RGB(255, 255, 255), false);
        }
        sdl_ui::draw::circle(&mut ui.canvas, coordinates::Vec2::Norm(-1., -1.), 20, Color::RGB(255, 255, 255), false);
        sdl_ui::draw::line(&mut ui.canvas, coordinates::Vec2::Norm(0., 0.), coordinates::Vec2::Norm(-1., 1.), Color::RGB(0, 255, 0));
        sdl_ui::draw::line(&mut ui.canvas, coordinates::Vec2::Norm(0., 0.), coordinates::Vec2::Norm(1., 1.), Color::RGB(0, 0, 255));
        sdl_ui::draw::line(&mut ui.canvas, coordinates::Vec2::Norm(0., 0.), coordinates::Vec2::Norm(0.25, 1.), Color::RGB(0, 255, 255));
        sdl_ui::draw::triangle(&mut ui.canvas, coordinates::Vec2::Norm(-0.5, -0.5), coordinates::Vec2::Norm(0.5, -0.7), coordinates::Vec2::Norm(0., 0.5), Color::RGB(255, 0, 0), true);
        sdl_ui::draw::triangle(&mut ui.canvas, coordinates::Vec2::Norm(-0.5, -0.5), coordinates::Vec2::Norm(0.5, -0.7), coordinates::Vec2::Norm(0., 0.5), Color::RGB(0, 255, 0), false);

        for event in ui.sdl.event_pump().unwrap().poll_iter() {
            ui.event_handler(event);
        }

        ui.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
