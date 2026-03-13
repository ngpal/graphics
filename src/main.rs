use minifb::{Key, Window, WindowOptions};

const SIZE: usize = 600;

struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn project(&self) -> Vec2 {
        Vec2::new(self.x / self.z, self.y / self.z)
    }
}

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

fn main() {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let mut window = Window::new("Cube - ESC to exit", SIZE, SIZE, WindowOptions::default())
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

    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4),
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7),
    ];

    let mut z_offset = 0.0f32;
    let mut x_offset = 0.0f32;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);

        if window.is_key_down(Key::Up) {
            z_offset -= 0.02;
        }

        if window.is_key_down(Key::Down) {
            z_offset += 0.02;
        }

        if window.is_key_down(Key::Right) {
            x_offset -= 0.02;
        }

        if window.is_key_down(Key::Left) {
            x_offset += 0.02
        }

        z_offset = z_offset.max(-1.4);

        let mut transformed = [Vec3::new(0.0, 0.0, 0.0); 8];

        for i in 0..cube.len() {
            transformed[i] = Vec3::new(cube[i].x + x_offset, cube[i].y, cube[i].z + z_offset);
        }

        for v in transformed.iter() {
            draw_circle(&mut buffer, v.project(), 0.01, 0x00FF00);
        }

        for &(p0, p1) in edges.iter() {
            draw_line(
                &mut buffer,
                transformed[p0].project(),
                transformed[p1].project(),
                0x00FF00,
            );
        }

        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
    }
}
