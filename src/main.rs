extern crate sdl2;

mod draw;
mod ui;

use draw::color::Color;
use draw::coordinates;
use draw::draw_2d;
use std::time::Duration;
use ui::sdl::SdlUi;
use ui::ui::UI;

pub fn draw_2d(ui: &mut SdlUi) {
    draw_2d::rectangle(
        ui,
        coordinates::Vec2::Screen(20, 20),
        80,
        40,
        Color::rgb(255, 255, 255),
        false,
    );
    draw_2d::square(
        ui,
        coordinates::Vec2::Screen(180, 180),
        40,
        Color::rgb(255, 0, 0),
        false,
    );
    for radius in (1..100).step_by(5) {
        draw_2d::circle(
            ui,
            coordinates::Vec2::Screen(200, 200),
            radius,
            Color::rgb(255, 255, 255),
            false,
        );
    }
    draw_2d::circle(
        ui,
        coordinates::Vec2::Norm(-1., -1.),
        20,
        Color::rgb(255, 255, 255),
        false,
    );
    draw_2d::line(
        ui,
        coordinates::Vec2::Norm(0., 0.),
        coordinates::Vec2::Norm(-1., 1.),
        Color::rgb(0, 255, 0),
    );
    draw_2d::line(
        ui,
        coordinates::Vec2::Norm(0., 0.),
        coordinates::Vec2::Norm(1., 1.),
        Color::rgb(0, 0, 255),
    );
    draw_2d::line(
        ui,
        coordinates::Vec2::Norm(0., 0.),
        coordinates::Vec2::Norm(0.25, 1.),
        Color::rgb(0, 255, 255),
    );
    draw_2d::triangle(
        ui,
        coordinates::Vec2::Norm(-0.5, -0.5),
        coordinates::Vec2::Norm(0.5, -0.7),
        coordinates::Vec2::Norm(0., 0.5),
        Color::rgb(255, 0, 0),
        true,
    );
    draw_2d::triangle(
        ui,
        coordinates::Vec2::Norm(-0.5, -0.5),
        coordinates::Vec2::Norm(0.5, -0.7),
        coordinates::Vec2::Norm(0., 0.5),
        Color::rgb(0, 255, 0),
        false,
    );
}

fn draw_3d(ui: &mut SdlUi) {
    draw::draw_3d::cube(
        ui,
        // coordinates::Vec3::Screen(500, 500, 2),
        coordinates::Vec3::Norm(-0.5, 0.5, 2.0),
        // coordinates::Vec3::Norm(0., 0., 2.0),
        500,
        0.0,
        0.0,
        0.0,
        draw::color::WHITE,
    );
}

pub fn main() {
    let mut ui = SdlUi::new("Drawing");
    ui.set_color(Color::rgb(0, 0, 0));
    ui.clear(draw::color::BLACK);
    ui.present();

    while ui.run() {
        // draw_2d(&mut ui);
        draw_3d(&mut ui);
        ui.handl_events();
        ui.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
