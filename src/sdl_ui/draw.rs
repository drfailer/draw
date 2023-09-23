use sdl2::{pixels::Color, rect::Rect, render::Canvas};

use super::{ui::HEIGHT , coordinates};

pub fn rectangle(
    canvas: &mut Canvas<sdl2::video::Window>,
    coord: coordinates::Vec2,
    width: u32,
    height: u32,
    color: Color,
    fill: bool,
) {
    let (sx, sy) = coordinates::norm_to_screen(coord);

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
    coord: coordinates::Vec2,
    size: u32,
    color: Color,
    fill: bool,
) {
    rectangle(canvas, coord, size, size, color, fill);
}

fn circle_equation(x: i32, y: i32, a: i32, b: i32, r: i32) -> i32 {
    let r2 = r * r;
    return (x - a) * (x - a) + (y - b) * (y - b) - r2;
}

/**
 * TODO: add fill option
 */
pub fn circle(
    canvas: &mut Canvas<sdl2::video::Window>,
    coord: coordinates::Vec2,
    radius: i32,
    color: Color,
    fill: bool,
) {
    let (x, y) = coordinates::norm_to_screen(coord);
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
                        canvas,
                        coordinates::Vec2::Screen(ix, iy),
                        2 * dx as u32,
                        1,
                        color,
                        true,
                    );
                    rectangle(
                        canvas,
                        coordinates::Vec2::Screen(ix, y + dy),
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
                    while circle_equation(ix, norm_iy, norm_x, norm_y, radius) <= 2*radius {
                        let dx = x - ix;
                        let dy = y - iy;
                        square(canvas, coordinates::Vec2::Screen(ix, iy), 1, color, true);
                        square(canvas, coordinates::Vec2::Screen(ix, y + dy), 1, color, true);
                        square(canvas, coordinates::Vec2::Screen(x + dx, iy), 1, color, true);
                        square(canvas, coordinates::Vec2::Screen(x + dx, y + dy), 1, color, true);
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

fn line(canvas: &mut Canvas<sdl2::video::Window>, point1: coordinates::Vec2, point2: coordinates::Vec2) {
    todo!();
}

fn triangle(
    canvas: &mut Canvas<sdl2::video::Window>,
    point1: coordinates::Vec2,
    point2: coordinates::Vec2,
    point3: coordinates::Vec2,
) {
    todo!();
}
