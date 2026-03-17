#![allow(unused)]

mod camera;
mod mesh;
mod rasterizer;
mod vector;

use minifb::{Key, Window, WindowOptions};

use crate::{
    camera::Camera,
    mesh::cube,
    rasterizer::Rasterizer,
    vector::{Quat, Vec3},
};

const SIZE: usize = 300;

fn main() {
    let mut window = Window::new(
        "Cube - ESC to exit",
        SIZE,
        SIZE,
        WindowOptions {
            scale: minifb::Scale::X2,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.set_target_fps(60);

    let mesh = cube();
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalized();
    let mut rasterizer = Rasterizer::new(SIZE);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, -2.0));
    let mut orientation = Quat::identity();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rasterizer.clear(0x000055);

        let speed = 1.5_f32;

        if window.is_key_down(Key::Left)  { orientation = Quat::from_axis_angle(Vec3::new(0.,1.,0.), -speed) * orientation; }
        if window.is_key_down(Key::Right) { orientation = Quat::from_axis_angle(Vec3::new(0.,1.,0.),  speed) * orientation; }
        if window.is_key_down(Key::Up)    { orientation = Quat::from_axis_angle(Vec3::new(1.,0.,0.), -speed) * orientation; }
        if window.is_key_down(Key::Down)  { orientation = Quat::from_axis_angle(Vec3::new(1.,0.,0.),  speed) * orientation; }
        if window.is_key_down(Key::W) { camera.position.z += 0.02; }
        if window.is_key_down(Key::S) { camera.position.z -= 0.02; }
        if window.is_key_down(Key::A) { camera.position.x -= 0.02; }
        if window.is_key_down(Key::D) { camera.position.x += 0.02; }

        let transformed: Vec<Vec3> = mesh.vertices
            .iter()
            .map(|&v| orientation.rotate(v))
            .collect();

        for &(i0, i1, i2) in &mesh.triangles {
            let v0 = transformed[i0];
            let v1 = transformed[i1];
            let v2 = transformed[i2];

            let normal = (v1 - v0).cross(v2 - v0).normalized();
            let intensity = normal.dot(light_dir).max(0.0);
            let color = (intensity * 255.0) as u32;
            let color = color << 16;

            let Some((p0, z0)) = camera.project(v0) else { continue };
            let Some((p1, z1)) = camera.project(v1) else { continue };
            let Some((p2, z2)) = camera.project(v2) else { continue };

            rasterizer.draw_triangle_2d(p0, z0, p1, z1, p2, z2, color);
        }

        window
            .update_with_buffer(&rasterizer.buffer, SIZE, SIZE)
            .unwrap();
    }
}
