use crate::vector::Vec2;

pub struct Rasterizer {
    pub buffer: Vec<u32>,
    pub z_buffer: Vec<f32>,
    pub size: usize,
}

impl Rasterizer {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size * size],
            z_buffer: vec![f32::INFINITY; size * size],
            size,
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
        self.z_buffer.fill(f32::INFINITY);
    }

    fn to_screen(&self, v: Vec2) -> (i32, i32) {
        let half = self.size as f32 / 2.0;

        let x = (v.x * half + half) as i32;
        let y = (-v.y * half + half) as i32;

        (x, y)
    }

    fn screen_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= self.size as i32 || y < 0 || y >= self.size as i32 {
            return None;
        }

        Some(x as usize + y as usize * self.size)
    }

    /// Rasterize a 2D triangle. Accepts projected screen-space points and per-vertex depth.
    pub fn draw_triangle_2d(
        &mut self,
        p0: Vec2, z0: f32,
        p1: Vec2, z1: f32,
        p2: Vec2, z2: f32,
        color: u32,
    ) {
        let (x0, y0) = self.to_screen(p0);
        let (x1, y1) = self.to_screen(p1);
        let (x2, y2) = self.to_screen(p2);

        let min_x = x0.min(x1).min(x2).max(0);
        let max_x = x0.max(x1).max(x2).min(self.size as i32 - 1);

        let min_y = y0.min(y1).min(y2).max(0);
        let max_y = y0.max(y1).max(y2).min(self.size as i32 - 1);

        let edge = |ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32| {
            (cx - ax) * (by - ay) - (cy - ay) * (bx - ax)
        };

        let area = edge(x0, y0, x1, y1, x2, y2) as f32;

        if area == 0.0 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let w0 = edge(x1, y1, x2, y2, x, y);
                let w1 = edge(x2, y2, x0, y0, x, y);
                let w2 = edge(x0, y0, x1, y1, x, y);

                if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                    let w0f = w0 as f32 / area;
                    let w1f = w1 as f32 / area;
                    let w2f = w2 as f32 / area;

                    // Interpolate depth from the three vertex depths.
                    let z = z0 * w0f + z1 * w1f + z2 * w2f;

                    if let Some(idx) = self.screen_to_index(x, y) {
                        if z < self.z_buffer[idx] {
                            self.z_buffer[idx] = z;
                            self.buffer[idx] = color;
                        }
                    }
                }
            }
        }
    }

    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, color: u32) {
        let (x0, y0) = self.to_screen(p0);
        let (x1, y1) = self.to_screen(p1);

        let dx = (x1 - x0) as f32;
        let dy = (y1 - y0) as f32;

        let steps = dx.abs().max(dy.abs()).max(1.0) as usize;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;

            let x = x0 as f32 + dx * t;
            let y = y0 as f32 + dy * t;

            if let Some(idx) = self.screen_to_index(x as i32, y as i32) {
                self.buffer[idx] = color;
            }
        }
    }

    pub fn draw_circle(&mut self, center: Vec2, r: f32, color: u32) {
        let step = 2.0 / self.size as f32;

        let radius_px = (r / step) as i32;

        for dy in -radius_px..=radius_px {
            for dx in -radius_px..=radius_px {
                let sx = center.x + dx as f32 * step;
                let sy = center.y + dy as f32 * step;

                if (sx - center.x).powi(2) + (sy - center.y).powi(2) <= r * r {
                    let (x, y) = self.to_screen(Vec2::new(sx, sy));
                    if let Some(idx) = self.screen_to_index(x, y) {
                        self.buffer[idx] = color;
                    }
                }
            }
        }
    }
}
