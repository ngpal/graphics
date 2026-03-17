use crate::vector::{Vec2, Vec3};

pub struct Camera {
    pub position: Vec3,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    /// Project a world-space point into NDC.
    /// Returns None if the point is behind the camera (z <= 0).
    pub fn project(&self, v: Vec3) -> Option<(Vec2, f32)> {
        let rel = v - self.position;

        const NEAR: f32 = 0.2;
        if rel.z <= NEAR {
            return None;
        }

        let projected = Vec2::new(rel.x / rel.z, rel.y / rel.z);
        Some((projected, rel.z))
    }
}
