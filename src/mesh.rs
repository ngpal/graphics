use crate::vector::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<(usize, usize, usize)>,
}

/// A unit cube centred at the origin.
pub fn cube() -> Mesh {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5), // 0
        Vec3::new(0.5, -0.5, -0.5),  // 1
        Vec3::new(0.5, 0.5, -0.5),   // 2
        Vec3::new(-0.5, 0.5, -0.5),  // 3
        Vec3::new(-0.5, -0.5, 0.5),  // 4
        Vec3::new(0.5, -0.5, 0.5),   // 5
        Vec3::new(0.5, 0.5, 0.5),    // 6
        Vec3::new(-0.5, 0.5, 0.5),   // 7
    ];

    let triangles = vec![
        // front
        (0, 1, 2),
        (0, 2, 3),
        // back
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

    Mesh { vertices, triangles }
}
