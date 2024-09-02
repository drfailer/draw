use super::color::Color;
use super::coordinates::c2::Vec2;
use super::coordinates::c3::{rotate, to_screen3, to_world3, Vec3};
use super::draw_2d;
use crate::ui::ui::UI;

pub fn line(ui: &mut impl UI, point1: Vec3, point2: Vec3, color: Color) {
    let (mut x1, mut y1, _) = to_screen3(point1).unwrap();
    let (mut x2, mut y2, _) = to_screen3(point2).unwrap();

    draw_2d::line(ui, Vec2::Screen(x1, y1), Vec2::Screen(x2, y2), color);
}

fn barycenter3(points: Vec<Vec3>) -> Vec3 {
    let mut xbar = 0.;
    let mut ybar = 0.;
    let mut zbar = 0.;
    let nb_points = points.len() as f64;

    for point in points {
        let (xp, yp, zp) = to_world3(point).unwrap();
        xbar += xp;
        ybar += yp;
        zbar += zp;
    }
    return Vec3::World(xbar / nb_points, ybar / nb_points, zbar / nb_points);
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
    let (x, y, z) = to_world3(coord).unwrap();
    // todo: wrong value
    // let ratio = (ui.width() + ui.height()) as f64 / 2.0;
    // let width = size as f64 / ui.width() as f64;
    // let height = size as f64 / ui.height() as f64;
    // let depth = size as f64 / ratio;
    let width = size as f64;
    let height = size as f64;
    let depth = size as f64;
    // let depth = 1.;

    let mut point1 = Vec3::World(x, y, z);
    let mut point2 = Vec3::World(x + width, y, z);
    let mut point3 = Vec3::World(x + width, y - height, z);
    let mut point4 = Vec3::World(x, y - height, z);
    let mut point5 = Vec3::World(x, y, z + depth);
    let mut point6 = Vec3::World(x + width, y, z + depth);
    let mut point7 = Vec3::World(x + width, y - height, z + depth);
    let mut point8 = Vec3::World(x, y - height, z + depth);

    let barycenter = barycenter3(vec![
        point1.clone(),
        point2.clone(),
        point3.clone(),
        point4.clone(),
        point5.clone(),
        point6.clone(),
        point7.clone(),
        point8.clone(),
    ]);

    let points = vec![
        &mut point1,
        &mut point2,
        &mut point3,
        &mut point4,
        &mut point5,
        &mut point6,
        &mut point7,
        &mut point8,
    ];

    for point in points {
        rotate(barycenter.clone(), point, angle_x, angle_y, angle_z);
    }

    line(ui, point1.clone(), point2.clone(), color);
    line(ui, point2.clone(), point3.clone(), color);
    line(ui, point3.clone(), point4.clone(), color);
    line(ui, point4.clone(), point1.clone(), color);

    line(ui, point1.clone(), point5.clone(), color);
    line(ui, point2.clone(), point6.clone(), color);
    line(ui, point3.clone(), point7.clone(), color);
    line(ui, point4.clone(), point8.clone(), color);

    line(ui, point5.clone(), point6.clone(), color);
    line(ui, point6.clone(), point7.clone(), color);
    line(ui, point7.clone(), point8.clone(), color);
    line(ui, point8.clone(), point5.clone(), color);
}
