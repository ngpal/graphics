#![allow(unused)]

mod camera;
mod mesh;
mod rasterizer;
mod vector;

use minifb::{Key, Window, WindowOptions};

use crate::{
    camera::Camera,
    mesh::{Mesh, cube},
    rasterizer::Rasterizer,
    vector::{Quat, Vec2, Vec3},
};

const NEAR: f32 = 0.2;

/// Clip a triangle (given in camera-relative space) against the near plane z = NEAR.
/// Returns 0, 1, or 2 sub-triangles, all with z > NEAR.
fn clip_near(a: Vec3, b: Vec3, c: Vec3) -> Vec<[Vec3; 3]> {
    let inside = [a, b, c]
        .iter()
        .copied()
        .filter(|v| v.z > NEAR)
        .collect::<Vec<_>>();
    let outside = [a, b, c]
        .iter()
        .copied()
        .filter(|v| v.z <= NEAR)
        .collect::<Vec<_>>();

    // Linearly interpolate from v_in to v_out to find the point where z == NEAR.
    let clip_edge = |vi: Vec3, vo: Vec3| -> Vec3 {
        let t = (NEAR - vo.z) / (vi.z - vo.z);
        Vec3::new(
            vo.x + t * (vi.x - vo.x),
            vo.y + t * (vi.y - vo.y),
            NEAR,
        )
    };

    match inside.len() {
        3 => vec![[a, b, c]],
        0 => vec![],
        2 => {
            // One vertex behind near plane → clip to a quad (two triangles).
            let p0 = clip_edge(inside[0], outside[0]);
            let p1 = clip_edge(inside[1], outside[0]);
            vec![
                [inside[0], inside[1], p1],
                [inside[0], p1, p0],
            ]
        }
        1 => {
            // Two vertices behind near plane → clip to one smaller triangle.
            let p0 = clip_edge(inside[0], outside[0]);
            let p1 = clip_edge(inside[0], outside[1]);
            vec![[inside[0], p0, p1]]
        }
        _ => vec![],
    }
}

const SIZE: usize = 600;
const BG_COLOR: u32 = 0x000022;

fn main() {
    let mut window = Window::new(
        "Cube - ESC to exit",
        SIZE,
        SIZE,
        WindowOptions {
            // scale: minifb::Scale::X2,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| panic!("{}", e));

    window.set_target_fps(60);

    let mesh = Mesh::from_obj_file("utah_teapot.obj");
    let light_dir = Vec3::new(0.0, 0.0, 1.0).normalized();
    let mut rasterizer = Rasterizer::new(SIZE);
    let mut camera = Camera::new(Vec3::new(0.0, 0.0, -2.0));
    let mut orientation = Quat::identity();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rasterizer.clear(BG_COLOR);

        let speed = 1.5_f32;

        if window.is_key_down(Key::Left) {
            orientation = Quat::from_axis_angle(Vec3::new(0., 1., 0.), -speed) * orientation;
        }
        if window.is_key_down(Key::Right) {
            orientation = Quat::from_axis_angle(Vec3::new(0., 1., 0.), speed) * orientation;
        }
        if window.is_key_down(Key::Up) {
            orientation = Quat::from_axis_angle(Vec3::new(1., 0., 0.), -speed) * orientation;
        }
        if window.is_key_down(Key::Down) {
            orientation = Quat::from_axis_angle(Vec3::new(1., 0., 0.), speed) * orientation;
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
            .map(|&v| orientation.rotate(v))
            .collect();

        for &(i0, i1, i2) in &mesh.triangles {
            let v0 = transformed[i0];
            let v1 = transformed[i1];
            let v2 = transformed[i2];

            let normal = (v1 - v0).cross(v2 - v0).normalized();
            let intensity = normal.dot(-light_dir).max(0.0);
            let color = (intensity * 255.0) as u32;
            let color = color << 16;

            // backface culling: skip triangles facing away from the camera
            let view_dir = (camera.position - v0).normalized();
            if normal.dot(view_dir) <= 0.0 {
                continue;
            }

            // Clip the triangle against the near plane in camera-relative space.
            let r0 = v0 - camera.position;
            let r1 = v1 - camera.position;
            let r2 = v2 - camera.position;

            for [c0, c1, c2] in clip_near(r0, r1, r2) {
                // Project each clipped (already-relative) vertex directly.
                let project = |r: Vec3| -> (Vec2, f32) {
                    (Vec2::new(r.x / r.z, r.y / r.z), r.z)
                };

                let (p0, z0) = project(c0);
                let (p1, z1) = project(c1);
                let (p2, z2) = project(c2);

                rasterizer.draw_triangle_2d(p0, z0, p1, z1, p2, z2, color);
            }
        }

        window
            .update_with_buffer(&rasterizer.buffer, SIZE, SIZE)
            .unwrap();
    }
}
