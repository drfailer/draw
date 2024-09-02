use super::color::Color;
use super::coordinates::c2::ScreenCoord;
use super::coordinates::c3::{LocalCoord3, Vec3, ViewScreenCoord, rotate};
use super::draw_2d;
use crate::ui::ui::UI;

pub fn line(ui: &mut impl UI, point1: Vec3, point2: Vec3, color: Color) {
    draw_2d::line(
        ui,
        point1.to_screen(ui.width(), ui.height()),
        point2.to_screen(ui.width(), ui.height()),
        color,
    );
}

fn barycenter3(points: Vec<Vec3>) -> Vec3 {
    let mut xbar = 0.;
    let mut ybar = 0.;
    let mut zbar = 0.;
    let nb_points = points.len() as f64;

    for point in points {
        let Vec3(xp, yp, zp) = point;
        xbar += xp;
        ybar += yp;
        zbar += zp;
    }
    return Vec3(xbar / nb_points, ybar / nb_points, zbar / nb_points);
}

pub fn cube(
    ui: &mut impl UI,
    coordinate: Vec3,
    size: f64,
    angle_x: f64,
    angle_y: f64,
    angle_z: f64,
    color: Color,
) {
    let Vec3(x, y, z) = coordinate;
    let mut point1 = Vec3(x, y, z);
    let mut point2 = Vec3(x + size, y, z);
    let mut point3 = Vec3(x + size, y - size, z);
    let mut point4 = Vec3(x, y - size, z);
    let mut point5 = Vec3(x, y, z + size);
    let mut point6 = Vec3(x + size, y, z + size);
    let mut point7 = Vec3(x + size, y - size, z + size);
    let mut point8 = Vec3(x, y - size, z + size);

    let barycenter = barycenter3(vec![
        point1, point2, point3, point4, point5, point6, point7, point8,
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
        rotate(barycenter, point, angle_x, angle_y, angle_z);
    }

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
