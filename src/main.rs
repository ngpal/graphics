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

// screen coords to memory coords
fn s2m(coords: Vec2) -> u32 {
    // downscale by size; -1->1 to -SIZE/2->SIZE/2
    let half_size = SIZE as f32 / 2.;
    let res = (coords.x * half_size, -coords.y * half_size);
    // -SIZE/2->SIZE/2 to 0-SIZE
    let res = (res.0 + half_size, res.1 + half_size);
    // 1d coords
    (res.0 + res.1 * SIZE as f32) as u32
}

fn draw_circle(buffer: &mut [u32], c: Vec2, r: f32, color: u32) {
    let step = 2.0 / SIZE as f32;

    for dy in -((r / step) as i32)..=((r / step) as i32) {
        for dx in -((r / step) as i32)..=((r / step) as i32) {
            let sx = c.x + dx as f32 * step;
            let sy = c.y + dy as f32 * step;

            if (sx - c.x) * (sx - c.x) + (sy - c.y) * (sy - c.y) <= r * r {
                let idx = s2m(Vec2::new(sx, sy)) as usize;
                if idx < buffer.len() {
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

    let steps = dx.abs().max(dy.abs()) as usize;

    for i in 0..=steps {
        let t = i as f32 / steps as f32;

        let x = p0.x + (p1.x - p0.x) * t;
        let y = p0.y + (p1.y - p0.y) * t;

        let idx = s2m(Vec2::new(x, y)) as usize;
        if idx < buffer.len() {
            buffer[idx] = color;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; SIZE * SIZE];

    let mut window = Window::new("Test - ESC to exit", SIZE, SIZE, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
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

    for v in cube.iter() {
        draw_circle(&mut buffer, v.project(), 0.01, 0x00FF00);
    }

    for &(p0, p1) in edges.iter() {
        draw_line(
            &mut buffer,
            cube[p0].project(),
            cube[p1].project(),
            0x00FF00,
        );
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
    }
}
