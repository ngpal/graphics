#![allow(unused)]

mod vector;

use crate::vector::{Vec2, Vec3};
use minifb::{Key, Window, WindowOptions};
use std::{cmp::max, f32::consts::PI};

const SIZE: usize = 300;

fn s2m(coords: Vec2) -> Option<usize> {
    let half = SIZE as f32 / 2.0;

    let x = coords.x * half + half;
    let y = -coords.y * half + half;

    let xi = x as i32;
    let yi = y as i32;

    if xi < 0 || xi >= SIZE as i32 || yi < 0 || yi >= SIZE as i32 {
        return None;
    }

    Some((xi as usize) + (yi as usize) * SIZE)
}

fn to_screen(v: Vec2) -> (i32, i32) {
    let half = SIZE as f32 / 2.0;

    let x = (v.x * half + half) as i32;
    let y = (-v.y * half + half) as i32;

    (x, y)
}

fn draw_filled_triangle(
    buffer: &mut [u32],
    z_buffer: &mut [f32],
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: u32,
) {
    let (x0, y0) = to_screen(p0.project());
    let (x1, y1) = to_screen(p1.project());
    let (x2, y2) = to_screen(p2.project());

    let min_x = x0.min(x1).min(x2).max(0);
    let max_x = x0.max(x1).max(x2).min(SIZE as i32 - 1);

    let min_y = y0.min(y1).min(y2).max(0);
    let max_y = y0.max(y1).max(y2).min(SIZE as i32 - 1);

    let edge = |ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32| {
        (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
    };

    let area = edge(x0, y0, x1, y1, x2, y2) as f32;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let w0 = edge(x1, y1, x2, y2, x, y);
            let w1 = edge(x2, y2, x0, y0, x, y);
            let w2 = edge(x0, y0, x1, y1, x, y);

            if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                let w0f = w0 as f32 / area;
                let w1f = w1 as f32 / area;
                let w2f = w2 as f32 / area;

                // interpolate depth
                let z = p0.z * w0f + p1.z * w1f + p2.z * w2f;

                let idx = x as usize + y as usize * SIZE;

                if z < z_buffer[idx] {
                    z_buffer[idx] = z;
                    buffer[idx] = color;
                }
            }
        }
    }
}

fn draw_circle(buffer: &mut [u32], c: Vec2, r: f32, color: u32) {
    let step = 2.0 / SIZE as f32;

    for dy in -((r / step) as i32)..=((r / step) as i32) {
        for dx in -((r / step) as i32)..=((r / step) as i32) {
            let sx = c.x + dx as f32 * step;
            let sy = c.y + dy as f32 * step;

            if (sx - c.x) * (sx - c.x) + (sy - c.y) * (sy - c.y) <= r * r {
                if let Some(idx) = s2m(Vec2::new(sx, sy)) {
                    buffer[idx] = color;
                }
            }
        }
    }
}

fn draw_line(buffer: &mut [u32], p0: Vec2, p1: Vec2, color: u32) {
    let x0 = p0.x * SIZE as f32 / 2.0;
    let y0 = p0.y * SIZE as f32 / 2.0;
    let x1 = p1.x * SIZE as f32 / 2.0;
    let y1 = p1.y * SIZE as f32 / 2.0;

    let dx = x1 - x0;
    let dy = y1 - y0;

    let steps = dx.abs().max(dy.abs()).max(1.0) as usize;

    for i in 0..=steps {
        let t = i as f32 / steps as f32;

        let x = p0.x + (p1.x - p0.x) * t;
        let y = p0.y + (p1.y - p0.y) * t;

        if let Some(idx) = s2m(Vec2::new(x, y)) {
            buffer[idx] = color;
        }
    }
}

fn rotate_point_y(point: Vec3, deg: f32) -> Vec3 {
    let rad = deg * PI / 180.;

    let x = point.x * rad.cos() + point.z * rad.sin();
    let y = point.y;
    let z = point.x * -rad.sin() + point.z * rad.cos();

    Vec3::new(x, y, z)
}

fn rotate_point_x(point: Vec3, deg: f32) -> Vec3 {
    let rad = deg * PI / 180.;

    let x = point.x;
    let y = point.z * -rad.sin() + point.y * rad.cos();
    let z = point.z * rad.cos() + point.y * rad.sin();

    Vec3::new(x, y, z)
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

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

    let cube = [
        Vec3::new(-0.5, -0.5, 1.5),
        Vec3::new(0.5, -0.5, 1.5),
        Vec3::new(0.5, 0.5, 1.5),
        Vec3::new(-0.5, 0.5, 1.5),
        Vec3::new(-0.5, -0.5, 2.5),
        Vec3::new(0.5, -0.5, 2.5),
        Vec3::new(0.5, 0.5, 2.5),
        Vec3::new(-0.5, 0.5, 2.5),
    ];
    let triangles = [
        // front (+z facing camera → CCW)
        (0, 1, 2),
        (0, 2, 3),
        // back (flip!)
        (4, 6, 5),
        (4, 7, 6),
        // bottom
        (0, 5, 1),
        (0, 4, 5),
        // top
        (2, 7, 3),
        (2, 6, 7),
        // right
        (1, 6, 2),
        (1, 5, 6),
        // left
        (0, 3, 7),
        (0, 7, 4),
    ];
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let light_dir = Vec3::new(0., 0., 1.).normalized();
    let mut z_buffer = vec![f32::INFINITY; SIZE * SIZE];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        z_buffer.fill(f32::INFINITY);
        buffer.fill(0);

        let speed = 2.;

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

        let mut transformed = cube;

        for v in transformed.iter_mut() {
            v.z -= 2.;
            *v = rotate_point_y(*v, rot_y);
            *v = rotate_point_x(*v, rot_x);
            v.z += 2.;
        }

        for &(i0, i1, i2) in triangles.iter() {
            let ab = transformed[i1] - transformed[i0];
            let ac = transformed[i2] - transformed[i0];

            let normal = ab.cross(ac).normalized();
            let intensity = normal.dot(light_dir).max(0.0);

            let shade = (intensity * 255.0) as u32;
            let color = (shade << 16) | (shade << 8) | shade;

            let p0 = transformed[i0];
            let p1 = transformed[i1];
            let p2 = transformed[i2];

            draw_filled_triangle(&mut buffer, &mut z_buffer, p0, p1, p2, color);
        }

        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
    }
}
