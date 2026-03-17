#![allow(unused)]

mod camera;
mod mesh;
mod rasterizer;
mod transform;
mod vector;

use minifb::{Key, Window, WindowOptions};

use crate::{
    camera::Camera,
    mesh::cube,
    rasterizer::Rasterizer,
    transform::{rotate_point_x, rotate_point_y},
    vector::Vec3,
};

const SIZE: usize = 300;
const BG_COLOR: u32 = 0x000022;

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

    let mut rot_x = 0.0_f32;
    let mut rot_y = 0.0_f32;
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalized();
    let mut rasterizer = Rasterizer::new(SIZE);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, -2.0));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rasterizer.clear(BG_COLOR);

        let speed = 2.0;

        if window.is_key_down(Key::Left) {
            rot_y -= speed;
        }
        if window.is_key_down(Key::Right) {
            rot_y += speed;
        }
        if window.is_key_down(Key::Up) {
            rot_x -= speed;
        }
        if window.is_key_down(Key::Down) {
            rot_x += speed;
        }
        if window.is_key_down(Key::W) {
            camera.position.z += 0.02;
        }
        if window.is_key_down(Key::S) {
            camera.position.z -= 0.02;
        }
        if window.is_key_down(Key::A) {
            camera.position.x -= 0.02;
        }
        if window.is_key_down(Key::D) {
            camera.position.x += 0.02;
        }

        let transformed: Vec<Vec3> = mesh
            .vertices
            .iter()
            .map(|&v| {
                let v = rotate_point_y(v, rot_y);
                rotate_point_x(v, rot_x)
            })
            .collect();

        for &(i0, i1, i2) in &mesh.triangles {
            let v0 = transformed[i0];
            let v1 = transformed[i1];
            let v2 = transformed[i2];

            let ab = v1 - v0;
            let ac = v2 - v0;
            let normal = ab.cross(ac).normalized();
            let intensity = normal.dot(light_dir).max(0.0);
            let shade = (intensity * 255.0) as u32;
            let color = shade << 16;

            let Some((p0, z0)) = camera.project(v0) else {
                continue;
            };
            let Some((p1, z1)) = camera.project(v1) else {
                continue;
            };
            let Some((p2, z2)) = camera.project(v2) else {
                continue;
            };

            rasterizer.draw_triangle_2d(p0, z0, p1, z1, p2, z2, color);
        }

        window
            .update_with_buffer(&rasterizer.buffer, SIZE, SIZE)
            .unwrap();
    }
}
