
use crate::ui::sdl::{HEIGHT, WIDTH};

/*
 * Normalised coordinates are in (-1, 1)
 */

/******************************************************************************/
/*                                     2D                                     */
/******************************************************************************/

#[derive(Copy, Clone)]
pub enum Vec2 {
    Screen(i32, i32),
    Norm(f32, f32),
}

// -1..1 => 0..WIDTH
// x in -1..1
// x + 1 in 0..2
// (x + 1)/2 in 0..1
// WIDTH*(x + 1)/2 in 0..WIDTH

pub fn norm_to_screen(coord: Vec2) -> (i32, i32) {
    return match coord {
        Vec2::Norm(x, y) => {
            let sx = (WIDTH as f32) * (x + 1.) / 2.;
            let sy = (HEIGHT as f32) - (HEIGHT as f32) * (y + 1.) / 2.;
            (sx as i32, sy as i32)
        }
        Vec2::Screen(x, y) => (x, y),
    };
}

// 0..WIDTH => -1..1
// x in 0..WIDTH
// x / WIDTH in 0..1
// 2 * x / WIDTH in 0..2
// 2 * x / WIDTH - 1 in -1..1

// y in HEIGHT..0
// -y + HEIGHT in 0..HEIGHT
// (-y + HEIGHT) / HEIGHT in 0..1
// 2 * (-y + HEIGHT) / HEIGHT - 1 in -1..1

pub fn screen_to_norm(coord: Vec2) -> (f32, f32) {
    return match coord {
        Vec2::Screen(x, y) => {
            let sx = 2. * (x as f32) / (WIDTH as f32) - 1.;
            let sy = 2. * ((-y as f32) + (HEIGHT as f32)) / (HEIGHT as f32) - 1.;
            (sx, sy)
        }
        Vec2::Norm(x, y) => (x, y),
    };
}

/******************************************************************************/
/*                                     3D                                     */
/******************************************************************************/

pub enum Vec3 {
    Screen(i32, i32, i32),
    Norm(f32, f32, f32),
}

// TODO
