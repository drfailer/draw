extern crate sdl2;

mod ui;
mod draw;

use sdl2::pixels::Color;
use ui::{
    sdl,
    sdl::{HEIGHT, SdlUi, WIDTH},
};
use draw::coordinates;
use draw::draw_2d;
use std::time::Duration;

pub fn main() {
    let mut ui = SdlUi::new("Drawing");
    ui.set_color(Color::RGB(0, 0, 0));
    ui.clear();
    ui.present();

    while ui.run() {
        draw_2d::rectangle(
            &mut ui,
            coordinates::Vec2::Screen(20, 20),
            80,
            40,
            Color::RGB(255, 255, 255),
            false,
        );
        draw_2d::square(
            &mut ui,
            coordinates::Vec2::Screen(180, 180),
            40,
            Color::RGB(255, 0, 0),
            false,
        );
        for radius in (1..100).step_by(5) {
            draw_2d::circle(
                &mut ui,
                coordinates::Vec2::Screen(200, 200),
                radius,
                Color::RGB(255, 255, 255),
                false,
            );
        }
        draw_2d::circle(
            &mut ui,
            coordinates::Vec2::Norm(-1., -1.),
            20,
            Color::RGB(255, 255, 255),
            false,
        );
        draw_2d::line(
            &mut ui,
            coordinates::Vec2::Norm(0., 0.),
            coordinates::Vec2::Norm(-1., 1.),
            Color::RGB(0, 255, 0),
        );
        draw_2d::line(
            &mut ui,
            coordinates::Vec2::Norm(0., 0.),
            coordinates::Vec2::Norm(1., 1.),
            Color::RGB(0, 0, 255),
        );
        draw_2d::line(
            &mut ui,
            coordinates::Vec2::Norm(0., 0.),
            coordinates::Vec2::Norm(0.25, 1.),
            Color::RGB(0, 255, 255),
        );
        draw_2d::triangle(
            &mut ui,
            coordinates::Vec2::Norm(-0.5, -0.5),
            coordinates::Vec2::Norm(0.5, -0.7),
            coordinates::Vec2::Norm(0., 0.5),
            Color::RGB(255, 0, 0),
            true,
        );
        draw_2d::triangle(
            &mut ui,
            coordinates::Vec2::Norm(-0.5, -0.5),
            coordinates::Vec2::Norm(0.5, -0.7),
            coordinates::Vec2::Norm(0., 0.5),
            Color::RGB(0, 255, 0),
            false,
        );

        ui.handl_events();
        ui.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
