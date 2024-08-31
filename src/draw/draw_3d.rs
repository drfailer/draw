use std::mem::swap;

use super::color::Color;
use super::coordinates::{norm_to_screen3, screen_to_norm3, rotate, Vec3};
use crate::ui::ui::UI;

pub fn line(ui: &mut impl UI, point1: Vec3, point2: Vec3, color: Color) {
    let (mut screen_x1, mut screen_y1, _) = norm_to_screen3(point1);
    let (mut screen_x2, mut screen_y2, _) = norm_to_screen3(point2);
    let mut diffx = screen_x2 - screen_x1;
    let mut diffy = screen_y2 - screen_y1;
    ui.set_color(color);

    // if diffy.abs() <= diffx.abs() we iterate on x, otherwise we iterate on y
    // this is done to avoid drawing broken lines
    if diffy.abs() <= diffx.abs() {
        if screen_x1 > screen_x2 {
            swap(&mut screen_x1, &mut screen_x2);
            swap(&mut screen_y1, &mut screen_y2);
            diffx = -diffx;
            diffy = -diffy;
        }
        let direction = diffy as f64 / diffx as f64;
        for ix in screen_x1..=screen_x2 {
            let iy = (direction * (ix - screen_x1) as f64) as i32 + screen_y1;
            ui.draw_pixel(ix, iy);
        }
    } else {
        if screen_y1 > screen_y2 {
            swap(&mut screen_x1, &mut screen_x2);
            swap(&mut screen_y1, &mut screen_y2);
            diffx = -diffx;
            diffy = -diffy;
        }
        let direction = diffx as f64 / diffy as f64;
        for iy in screen_y1..=screen_y2 {
            let ix = (direction * (iy - screen_y1) as f64) as i32 + screen_x1;
            ui.draw_pixel(ix, iy);
        }
    }
}

pub fn cube(
    ui: &mut impl UI,
    coord: Vec3,
    size: i32,
    angle_x: f64,
    angle_y: f64,
    angle_z: f64,
    color: Color,
) {
    let (x, y, z) = screen_to_norm3(coord);
    let ratio = (ui.width() + ui.height()) as f64 / 2.0;
    let width = size as f64 / ui.width() as f64;
    let height = size as f64 / ui.height() as f64;
    let depth = size as f64 / ratio;

    let point1 = Vec3::Norm(x, y, z);
    let point2 = Vec3::Norm(x + width, y, z);
    let point3 = Vec3::Norm(x + width, y - height, z);
    let point4 = Vec3::Norm(x, y - height, z);
    let point5 = Vec3::Norm(x, y, z + depth);
    let point6 = Vec3::Norm(x + width, y, z + depth);
    let point7 = Vec3::Norm(x + width, y - height, z + depth);
    let point8 = Vec3::Norm(x, y - height, z + depth);

    line(ui, point1, point2, color);
    line(ui, point2, point3, color);
    line(ui, point3, point4, color);
    line(ui, point4, point1, color);

    line(ui, point1, point5, color);
    line(ui, point2, point6, color);
    line(ui, point3, point7, color);
    line(ui, point4, point8, color);

    line(ui, point5, point6, color);
    line(ui, point6, point7, color);
    line(ui, point7, point8, color);
    line(ui, point8, point5, color);
}
