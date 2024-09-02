extern crate sdl2;

mod draw;
mod ui;

use draw::color::Color;
use draw::coordinates::{c2::ScreenCoord, c3::Vec3};
use draw::draw_2d;
use std::time::Duration;
use ui::sdl::SdlUi;
use ui::ui::UI;

pub fn draw_2d(ui: &mut SdlUi) {
    draw_2d::rectangle(
        ui,
        ScreenCoord(20, 20),
        80,
        40,
        Color::rgb(255, 255, 255),
        false,
    );
    draw_2d::square(
        ui,
        ScreenCoord(180, 180),
        40,
        Color::rgb(255, 0, 0),
        false,
    );
    for radius in (1..100).step_by(5) {
        draw_2d::circle(
            ui,
            ScreenCoord(200, 200),
            radius,
            Color::rgb(255, 255, 255),
            false,
        );
    }
    draw_2d::line(
        ui,
        ScreenCoord(0, 0),
        ScreenCoord(0, ui.height() as i32),
        Color::rgb(0, 255, 0),
    );
    draw_2d::line(
        ui,
        ScreenCoord(0, 0),
        ScreenCoord(ui.width() as i32, ui.height() as i32),
        Color::rgb(0, 0, 255),
    );
    draw_2d::line(
        ui,
        ScreenCoord(0, 0),
        ScreenCoord(250, ui.height() as i32),
        Color::rgb(0, 255, 255),
    );
    draw_2d::triangle(
        ui,
        ScreenCoord(100, 100),
        ScreenCoord(200, 500),
        ScreenCoord(50, 200),
        Color::rgb(255, 0, 0),
        true,
    );
    draw_2d::triangle(
        ui,
        ScreenCoord(100, 100),
        ScreenCoord(200, 500),
        ScreenCoord(50, 200),
        Color::rgb(0, 255, 0),
        false,
    );
}

fn draw_3d(ui: &mut SdlUi, distance: f64, angle_x: f64, angle_y: f64, angle_z: f64) {
    draw::draw_3d::cube(
        ui,
        // coordinates::Vec3::Screen((WIDTH as i32 - 250) / 2, (HEIGHT as i32 - 250) / 2, 1),
        Vec3(-15., 15., 20.),
        // Vec3::World(300., 300., 2.0),
        // coordinates::Vec3::Norm(0., 0., 2.0),
        30.,
        angle_x,
        angle_y,
        angle_z,
        Color::hash("#00FFFF"),
    );
}

pub fn main() {
    let mut ui = SdlUi::new("Drawing");
    ui.set_color(Color::rgb(0, 0, 0));
    ui.clear(draw::color::BLACK);
    ui.present();
    let mut angle_x: f64 = 0.;
    let mut angle_y: f64 = 0.;
    let mut angle_z: f64 = 0.;

    while ui.run() {
        // draw_2d(&mut ui);
        ui.clear(draw::color::BLACK);
        let distance = libm::cos(angle_y) + 2.0;
        draw_3d(&mut ui, distance, angle_x, angle_y, angle_z);
        angle_x += 0.01;
        angle_y += 0.01;
        angle_z += 0.01;
        ui.handl_events();
        ui.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
