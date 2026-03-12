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
    let steps = ((p1.x - p0.x).abs().max((p1.y - p0.y).abs()) * SIZE as f32) as usize;

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

    let mut cam_z: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);

        for v in cube.iter() {
            let p = Vec3::new(v.x, v.y, v.z + cam_z).project();
            draw_circle(&mut buffer, p, 0.01, 0x00FF00);
        }

        for &(p0, p1) in edges.iter() {
            let a = Vec3::new(cube[p0].x, cube[p0].y, cube[p0].z + cam_z).project();
            let b = Vec3::new(cube[p1].x, cube[p1].y, cube[p1].z + cam_z).project();

            draw_line(&mut buffer, a, b, 0x00FF00);
        }

        if window.is_key_down(Key::Up) {
            cam_z += 0.02;
        }

        if window.is_key_down(Key::Down) {
            cam_z -= 0.02;
        }

        window.update_with_buffer(&buffer, SIZE, SIZE).unwrap();
    }
}
