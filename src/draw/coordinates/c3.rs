use std::cmp::max;

use super::config::{DISTANCE_CAMERA, VIEW_DISTANCE, VIEW_SCREEN_HEIGHT, VIEW_SCREEN_WIDTH};
use super::c2::ScreenCoord;
use libm::{cos, sin};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

#[derive(Copy, Clone, Debug)]
pub struct LocalCoord3(pub Vec3, pub Vec3);

#[derive(Copy, Clone, Debug)]
pub struct ViewScreenCoord(pub f64, pub f64);

/******************************************************************************/
/*                                    Vec3                                    */
/******************************************************************************/

#[allow(dead_code)]
impl Vec3 {
    pub fn to_local(self, center: Vec3) -> LocalCoord3 {
        let Vec3(xc, yc, zc) = center;
        let Vec3(x, y, z) = self;
        return LocalCoord3(Vec3(x - xc, y - yc, z - zc), center);
    }

    pub fn to_view(self) -> ViewScreenCoord {
        let Vec3(x, y, z) = self;
        let x = (DISTANCE_CAMERA / z) * x;
        let y = (DISTANCE_CAMERA / z) * y;
        return ViewScreenCoord(x, y);
    }

    pub fn to_screen(self, screen_width: u32, screen_height: u32) -> ScreenCoord {
        return self.to_view().to_screen(screen_width, screen_height);
    }
}

/******************************************************************************/
/*                                LocalCoord3                                 */
/******************************************************************************/

#[allow(dead_code)]
impl LocalCoord3 {
    pub fn to_world(self) -> Vec3 {
        let LocalCoord3(point, center) = self;
        let Vec3(x, y, z) = point;
        let Vec3(xc, yc, zc) = center;

        return Vec3(x + xc, y + yc, z + zc);
    }

    pub fn to_local(self, new_center: Vec3) -> LocalCoord3 {
        let Vec3(nxc, nyc, nzc) = new_center;
        let LocalCoord3(point, old_center) = self;
        let Vec3(x, y, z) = point;
        let Vec3(xc, yc, zc) = old_center;

        return LocalCoord3(Vec3(x + xc - nxc, y + yc - nyc, z + zc - nzc), new_center);
    }

    pub fn to_view(self) -> ViewScreenCoord {
        return self.to_world().to_view();
    }

    pub fn to_screen(self, screen_width: u32, screen_height: u32) -> ScreenCoord {
        return self.to_world().to_screen(screen_width, screen_height);
    }
}

/******************************************************************************/
/*                              ViewScreenCoord                               */
/******************************************************************************/

#[allow(dead_code)]
impl ViewScreenCoord {
    // -500..500 -> 0..W
    // x in -500..500
    // x + 500 in 0..1000
    // (x + 500) / 1000 in 0..1
    // W * (x + 500) / 1000 in 0..W
    pub fn to_screen(self, screen_width: u32, screen_height: u32) -> ScreenCoord {
        let ViewScreenCoord(x, y) = self;
        let w = VIEW_SCREEN_WIDTH as f64;
        let h = VIEW_SCREEN_HEIGHT as f64;
        let xb = w / 2.;
        let yb = h / 2.;
        let m = max(screen_width, screen_height) as f64;
        let dx = (m - screen_width as f64) / 2.;
        let dy = (m - screen_height as f64) / 2.;
        let x = m * ((x + xb) / w);
        let y = m * ((y + yb) / w);

        return ScreenCoord((x - dx) as i32, (y - dy) as i32);
    }
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
    let LocalCoord3(local_point, _) = point.to_local(barycenter);
    let Vec3(x, y, z) = local_point;

    let (x, y, z) = rotate_x(x, y, z, angle_x);
    let (x, y, z) = rotate_y(x, y, z, angle_y);
    let (x, y, z) = rotate_z(x, y, z, angle_z);

    *point = LocalCoord3(Vec3(x, y, z), barycenter).to_world();
}
