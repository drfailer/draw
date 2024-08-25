use std::mem::swap;

use sdl2::pixels::Color;

use crate::coordinates::{Vec2, norm_to_screen, screen_to_norm};

use crate::ui::sdl::{SdlUi, HEIGHT, WIDTH};

pub fn rectangle(
    ui: &mut SdlUi,
    coord: Vec2,
    width: u32,
    height: u32,
    color: Color,
    fill: bool,
) {
    let (sx, sy) = norm_to_screen(coord);

    ui.set_color(color);
    if fill {
        ui.draw_rect(sx, sy, width, height);
    } else {
        ui.draw_rect(sx, sy, width, 1);
        ui.draw_rect(sx, sy + height as i32, width, 1);
        ui.draw_rect(sx, sy, 1, height);
        ui.draw_rect(sx + width as i32, sy, 1, height);
    }
}

pub fn square(ui: &mut SdlUi, coord: Vec2, size: u32, color: Color, fill: bool) {
    rectangle(ui, coord, size, size, color, fill);
}

fn circle_equation(x: i32, y: i32, a: i32, b: i32, r: i32) -> i32 {
    let r2 = r * r;
    return (x - a) * (x - a) + (y - b) * (y - b) - r2;
}

/**
 * TODO: add fill option
 */
pub fn circle(ui: &mut SdlUi, coord: Vec2, radius: i32, color: Color, fill: bool) {
    let (x, y) = norm_to_screen(coord);
    let begin_y = y - radius;
    let end_y = y;
    let mut loop_x;
    let mut ix = x;

    if fill {
        for iy in begin_y..=end_y {
            loop_x = true;
            while loop_x && ix >= x - radius as i32 {
                let (norm_ix, norm_iy) = (ix, -iy + HEIGHT as i32);
                let (norm_x, norm_y) = (x, -y + HEIGHT as i32);
                if circle_equation(norm_ix, norm_iy, norm_x, norm_y, radius) >= 0 {
                    let dx = x - ix;
                    let dy = y - iy;
                    rectangle(
                        ui,
                        Vec2::Screen(ix, iy),
                        2 * dx as u32,
                        1,
                        color,
                        true,
                    );
                    rectangle(
                        ui,
                        Vec2::Screen(ix, y + dy),
                        2 * dx as u32,
                        1,
                        color,
                        true,
                    );
                    loop_x = false;
                } else {
                    ix -= 1;
                }
            }
        }
    } else {
        for iy in begin_y..=end_y {
            loop_x = true;
            while loop_x && ix >= x - radius as i32 {
                let (norm_ix, norm_iy) = (ix, -iy + HEIGHT as i32);
                let (norm_x, norm_y) = (x, -y + HEIGHT as i32);
                if circle_equation(norm_ix, norm_iy, norm_x, norm_y, radius) >= 0 {
                    let ix_save = ix;
                    while circle_equation(ix, norm_iy, norm_x, norm_y, radius) <= 2 * radius {
                        let dx = x - ix;
                        let dy = y - iy;
                        square(ui, Vec2::Screen(ix, iy), 1, color, true);
                        square(ui, Vec2::Screen(ix, y + dy), 1, color, true);
                        square(ui, Vec2::Screen(x + dx, iy), 1, color, true);
                        square(
                            ui,
                            Vec2::Screen(x + dx, y + dy),
                            1,
                            color,
                            true,
                        );
                        ix -= 1;
                    }
                    ix = ix_save;
                    loop_x = false;
                } else {
                    ix -= 1;
                }
            }
        }
    }
}

pub fn line(ui: &mut SdlUi, point1: Vec2, point2: Vec2, color: Color) {
    let (mut screen_x1, mut screen_y1) = norm_to_screen(point1);
    let (mut screen_x2, mut screen_y2) = norm_to_screen(point2);
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
        let direction = diffy as f32 / diffx as f32;
        for ix in screen_x1..=screen_x2 {
            let iy = (direction * (ix - screen_x1) as f32) as i32 + screen_y1;
            ui.draw_pixel(ix, iy);
        }
    } else {
        if screen_y1 > screen_y2 {
            swap(&mut screen_x1, &mut screen_x2);
            swap(&mut screen_y1, &mut screen_y2);
            diffx = -diffx;
            diffy = -diffy;
        }
        let direction = diffx as f32 / diffy as f32;
        for iy in screen_y1..=screen_y2 {
            let ix = (direction * (iy - screen_y1) as f32) as i32 + screen_x1;
            ui.draw_pixel(ix, iy);
        }
    }
}

fn sort_points_on_y(
    point1: (&mut i32, &mut i32),
    point2: (&mut i32, &mut i32),
    point3: (&mut i32, &mut i32),
) {
    let (x1, y1) = point1;
    let (x2, y2) = point2;
    let (x3, y3) = point3;

    if y1 > y2 {
        swap(x1, x2);
        swap(y1, y2);
    }
    if y2 > y3 {
        swap(x2, x3);
        swap(y2, y3);
    }
    if y1 > y2 {
        swap(x1, x2);
        swap(y1, y2);
    }
}

pub fn triangle(
    ui: &mut SdlUi,
    point1: Vec2,
    point2: Vec2,
    point3: Vec2,
    color: Color,
    fill: bool,
) {
    if fill {
        let (mut x1, mut y1) = norm_to_screen(point1);
        let (mut x2, mut y2) = norm_to_screen(point2);
        let (mut x3, mut y3) = norm_to_screen(point3);
        sort_points_on_y((&mut x1, &mut y1), (&mut x2, &mut y2), (&mut x3, &mut y3));

        // coeficients
        let mut diffx13 = x1 - x3;
        let mut diffy13 = y1 - y3;
        let mut diffx12 = x1 - x2;
        let mut diffy12 = y1 - y2;
        let mut diffx23 = x2 - x3;
        let mut diffy23 = y2 - y3;
        let direction13 = diffx13 as f32 / diffy13 as f32;
        let direction12 = diffx12 as f32 / diffy12 as f32;
        let direction23 = diffx23 as f32 / diffy23 as f32;

        // first half triangle
        for iy in y1..=y2 {
            let ix1 = (direction13 * (iy - y1) as f32) as i32 + x1;
            let ix2 = (direction12 * (iy - y2) as f32) as i32 + x2;
            line(
                ui,
                Vec2::Screen(ix1, iy),
                Vec2::Screen(ix2, iy),
                color,
            );
        }

        // second half triangle
        for iy in y2..=y3 {
            let ix2 = (direction13 * (iy - y1) as f32) as i32 + x1;
            let ix3 = (direction23 * (iy - y3) as f32) as i32 + x3;
            line(
                ui,
                Vec2::Screen(ix2, iy),
                Vec2::Screen(ix3, iy),
                color,
            );
        }
    } else {
        line(ui, point1, point2, color);
        line(ui, point1, point3, color);
        line(ui, point2, point3, color);
    }
}
