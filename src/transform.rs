use std::f32::consts::PI;

use crate::vector::Vec3;

pub fn rotate_point_y(point: Vec3, deg: f32) -> Vec3 {
    let rad = deg * PI / 180.0;

    let x = point.x * rad.cos() + point.z * rad.sin();
    let y = point.y;
    let z = point.x * -rad.sin() + point.z * rad.cos();

    Vec3::new(x, y, z)
}

pub fn rotate_point_x(point: Vec3, deg: f32) -> Vec3 {
    let rad = deg * PI / 180.0;

    let x = point.x;
    let y = point.z * -rad.sin() + point.y * rad.cos();
    let z = point.z * rad.cos() + point.y * rad.sin();

    Vec3::new(x, y, z)
}
