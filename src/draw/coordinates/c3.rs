use std::cmp::max;

use crate::ui::sdl::{HEIGHT, WIDTH};

use super::config::{DISTANCE_CAMERA, VIEW_DISTANCE, VIEW_SCREEN_HEIGHT, VIEW_SCREEN_WIDTH};
use libm::{cos, sin};

#[derive(Clone, Debug)]
pub enum Vec3 {
    Local(f64, f64, f64, Box<Vec3>),
    World(f64, f64, f64),
    View(f64, f64, f64),
    Clip(f64, f64, f64),
    Screen(i32, i32, i32),
}

/******************************************************************************/
/*                                convertions                                 */
/******************************************************************************/

pub fn to_local3(coordinate: Vec3, center: Vec3) -> Option<(f64, f64, f64)> {
    return match coordinate {
        Vec3::Local(x, y, z, c) => {
            let (old_cx, old_cy, old_cz) = to_world3(*c).unwrap();
            let (new_cx, new_cy, new_cz) = to_world3(center).unwrap();
            Some((
                x + old_cx - new_cx,
                y + old_cy - new_cy,
                z + old_cz - new_cz,
            ))
        }
        Vec3::World(x, y, z) => {
            let (xc, yc, zc) = to_world3(center).unwrap();
            Some((x - xc, y - yc, z - zc))
        }
        _ => None,
    };
}

pub fn to_world3(coordinate: Vec3) -> Option<(f64, f64, f64)> {
    return match coordinate {
        Vec3::Local(x, y, z, c) => {
            let (cx, cy, cz) = to_world3(*c).unwrap();
            Some((x + cx, y + cy, z + cz))
        }
        Vec3::World(x, y, z) => Some((x, y, z)),
        _ => None,
    };
}

// todo: the view will have a camera coordinate (moving the character)
pub fn to_view3(coordinate: Vec3) -> Option<(f64, f64, f64)> {
    return match coordinate {
        Vec3::Local(x, y, z, c) => {
            let (x, y, z) = to_world3(Vec3::Local(x, y, z, c)).unwrap();
            to_view3(Vec3::World(x, y, z))
        }
        Vec3::World(x, y, z) => {
            let x = (DISTANCE_CAMERA / z) * x;
            let y = (DISTANCE_CAMERA / z) * y;
            Some((x, y, 1.))
        }
        Vec3::View(x, y, z) => Some((x, y, z)),
        _ => None,
    };
}

// -500..500 -> 0..W
// x in -500..500
// x + 500 in 0..1000
// (x + 500) / 1000 in 0..1
// W * (x + 500) / 1000 in 0..W

// todo: the view will have a camera coordinate (moving the character)
pub fn to_screen3(coordinate: Vec3) -> Option<(i32, i32, i32)> {
    return match coordinate {
        Vec3::Local(x, y, z, c) => {
            let (x, y, z) = to_view3(Vec3::Local(x, y, z, c)).unwrap();
            to_screen3(Vec3::World(x, y, z))
        }
        Vec3::World(x, y, z) => {
            let (x, y, z) = to_view3(Vec3::World(x, y, z)).unwrap();
            to_screen3(Vec3::View(x, y, z))
        }
        Vec3::View(x, y, _) => {
            let w = VIEW_SCREEN_WIDTH as f64;
            let h = VIEW_SCREEN_HEIGHT as f64;
            let xb = w / 2.;
            let yb = h / 2.;
            let m = max(WIDTH, HEIGHT) as f64;
            let dx = (m - WIDTH as f64) / 2.;
            let dy = (m - HEIGHT as f64) / 2.;
            let x = m * ((x + xb) / w);
            let y = m * ((y + yb) / w);

            Some(((x - dx) as i32, (y - dy) as i32, 1))
        }
        Vec3::Clip(_, _, _) => None,
        Vec3::Screen(x, y, z) => Some((x, y, z)),
    };
}

/******************************************************************************/
/*                                   rotate                                   */
/******************************************************************************/

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

pub fn rotate(barycenter: Vec3, point: &mut Vec3, angle_x: f64, angle_y: f64, angle_z: f64) {
    let (x, y, z) = to_local3((*point).clone(), barycenter.clone()).unwrap();

    let (x, y, z) = rotate_x(x, y, z, angle_x);
    let (x, y, z) = rotate_y(x, y, z, angle_y);
    let (x, y, z) = rotate_z(x, y, z, angle_z);

    let (x, y, z) = to_world3(Vec3::Local(x, y, z, Box::<Vec3>::new(barycenter))).unwrap();
    *point = Vec3::World(x, y, z);
}
