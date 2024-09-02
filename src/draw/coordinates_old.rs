use crate::ui::sdl::{HEIGHT, WIDTH};
use libm::{cos, sin};

/*
 * Normalised coordinates are in (-1, 1)
 */

const DE: f64 = 1.;

/******************************************************************************/
/*                                     2D                                     */
/******************************************************************************/

#[derive(Copy, Clone)]
pub enum Vec2 {
    Screen(i32, i32),
    Norm(f64, f64),
}

// -1..1 => 0..WIDTH
// x in -1..1
// x + 1 in 0..2
// (x + 1)/2 in 0..1
// WIDTH*(x + 1)/2 in 0..WIDTH
pub fn norm_to_screen2(coord: Vec2) -> (i32, i32) {
    return match coord {
        Vec2::Norm(x, y) => {
            let sx = (WIDTH as f64) * (x + 1.) / 2.;
            let sy = (HEIGHT as f64) - (HEIGHT as f64) * (y + 1.) / 2.;
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
pub fn screen_to_norm2(coord: Vec2) -> (f64, f64) {
    return match coord {
        Vec2::Screen(x, y) => {
            let nx = 2. * (x as f64) / (WIDTH as f64) - 1.;
            let ny = 2. * ((-y as f64) + (HEIGHT as f64)) / (HEIGHT as f64) - 1.;
            (nx, ny)
        }
        Vec2::Norm(x, y) => (x, y),
    };
}

/******************************************************************************/
/*                                     3D                                     */
/******************************************************************************/

#[derive(Copy, Clone, Debug)]
pub enum Vec3 {
    Screen(i32, i32, i32),
    Norm(f64, f64, f64),
}

// todo: add constants / variables for the position of the camara and the view matrix (make it
// detatchec from the screen)

// todo: width and height should be parameters
pub fn norm_to_screen3(coord: Vec3) -> (i32, i32, i32) {
    return match coord {
        Vec3::Norm(x, y, z) => {
            let x = (DE / z) * x;
            let y = (DE / z) * y;
            let sx = (WIDTH as f64) * (x + 1.) / 2.;
            let sy = (HEIGHT as f64) - (HEIGHT as f64) * (y + 1.) / 2.;
            (sx as i32, sy as i32, 1)
        }
        Vec3::Screen(x, y, _) => (x, y, 1),
    };
}

pub fn screen_to_norm3(coord: Vec3) -> (f64, f64, f64) {
    return match coord {
        Vec3::Screen(x, y, z) => {
            let nx = 2. * (x as f64) / (WIDTH as f64) - 1.;
            let ny = 2. * ((-y as f64) + (HEIGHT as f64)) / (HEIGHT as f64) - 1.;
            let nx = (z as f64 / DE) * nx as f64;
            let ny = (z as f64 / DE) * ny as f64;
            (nx.into(), ny.into(), z.into())
        }
        Vec3::Norm(x, y, z) => (x, y, z),
    };
}

fn rotate_x(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {
    (
        x,
        y * cos(angle) - z * sin(angle),
        y * sin(angle) + z * cos(angle),
    )
}

fn rotate_y(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {
    (
        x * cos(angle) + z * sin(angle),
        y,
        -x * sin(angle) + z * cos(angle),
    )
}

fn rotate_z(x: f64, y: f64, z: f64, angle: f64) -> (f64, f64, f64) {
    (
        x * cos(angle) - y * sin(angle),
        x * sin(angle) + y * cos(angle),
        z,
    )
}

pub fn rotate(
    barycenter: Vec3,
    point: &mut Vec3,
    angle_x: f64,
    angle_y: f64,
    angle_z: f64,
) {
    let (xbar, ybar, zbar) = screen_to_norm3(barycenter);
    let (x, y, z) = screen_to_norm3(*point);

    let (x, y, z) = rotate_x(x - xbar, y - ybar, z - zbar, angle_x);
    let (x, y, z) = rotate_y(x, y, z, angle_y);
    let (x, y, z) = rotate_z(x, y, z, angle_z);
    *point = Vec3::Norm(x + xbar, y + ybar, z + zbar);
}
