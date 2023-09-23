use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use super::ui::{HEIGHT, WIDTH};

// NOTE: if the coordinates are normalised, may be make a conversion here
// it would be nice to add an enum for coordinates in differents plans (and conversion functions)

pub enum Coordinates {
    Screen(i32, i32),
    Norm(f32, f32),
}

// -1..1 => 0..WIDTH
// x in -1..1
// x + 1 in 0..2
// (x + 1)/2 in 0..1
// WIDTH*(x + 1)/2 in 0..WIDTH

// 0..WIDTH => -1..1
// x in 0..WIDTH
// x / WIDTH in 0..1
// 2 * x / WIDTH in 0..2
// 2 * x / WIDTH - 1 in -1..1
//
// y in HEIGHT..0
// -y + HEIGHT in 0..HEIGHT
// (-y + HEIGHT) / HEIGHT in 0..1
// 2 * (-y + HEIGHT) / HEIGHT - 1 in -1..1

fn norm_to_screen(coord: Coordinates) -> (i32, i32) {
    return match coord {
        Coordinates::Norm(x, y) => {
            let sx = (WIDTH as f32) * (x + 1.) / 2.;
            let sy = (HEIGHT as f32) - (HEIGHT as f32) * (y + 1.) / 2.;
            (sx as i32, sy as i32)
        }
        Coordinates::Screen(x, y) => (x, y),
    };
}

fn screen_to_norm(coord: Coordinates) -> (f32, f32) {
    return match coord {
        Coordinates::Screen(x, y) => {
            let sx = 2. * (x as f32) / (WIDTH as f32) - 1.;
            let sy = 2. * ((-y as f32) + (HEIGHT as f32)) / (HEIGHT as f32) - 1.;
            (sx, sy)
        }
        Coordinates::Norm(x, y) => (x, y),
    };
}

pub fn rectangle(
    canvas: &mut Canvas<sdl2::video::Window>,
    coord: Coordinates,
    width: u32,
    height: u32,
    color: Color,
    fill: bool,
) {
    let (sx, sy) = norm_to_screen(coord);

    canvas.set_draw_color(color);
    if fill {
        canvas.fill_rect(Rect::new(sx, sy, width, height)).unwrap();
    } else {
        canvas.fill_rect(Rect::new(sx, sy, width, 1)).unwrap();
        canvas
            .fill_rect(Rect::new(sx, sy + height as i32, width, 1))
            .unwrap();
        canvas.fill_rect(Rect::new(sx, sy, 1, height)).unwrap();
        canvas
            .fill_rect(Rect::new(sx + width as i32, sy, 1, height))
            .unwrap();
    }
}

pub fn square(
    canvas: &mut Canvas<sdl2::video::Window>,
    coord: Coordinates,
    size: u32,
    color: Color,
    fill: bool,
) {
    rectangle(canvas, coord, size, size, color, fill);
}

/* Symetric:
 * xa = ix - x & ya = iy - y
 * xa' = xa + 2(r - xa)
 * xa' = 2r - xa
 * xa' = 2r - ix + x
 */

/**
 * TODO: add fill option
 * TODO: optimize this
 */
pub fn circle(
    canvas: &mut Canvas<sdl2::video::Window>,
    coord: Coordinates,
    radius: i32,
    color: Color,
) {
    let (x, y) = norm_to_screen(coord);
    let radius2 = (radius * radius) as i32;
    let beginx = x - radius;
    let endx = x;
    let beginy = y - radius;
    let endy = y;

    for iy in beginy..=endy {
        for ix in beginx..=endx {
            let (nix, niy) = (ix, -iy + HEIGHT as i32);
            let (nx, ny) = (x, -y + HEIGHT as i32);
            if (nix - nx) * (nix - nx) + (niy - ny) * (niy - ny) - radius2 <= 0 {
                let dx = x - ix;
                let dy = y - iy;
                square(canvas, Coordinates::Screen(ix, iy), 1, color, true);
                square(canvas, Coordinates::Screen(x + dx, iy), 1, color, true);
                square(canvas, Coordinates::Screen(ix, y + dy), 1, color, true);
                square(canvas, Coordinates::Screen(x + dx, y + dy), 1, color, true);
            }
        }
    }
}
