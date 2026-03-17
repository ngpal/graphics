use std::fs;

use crate::vector::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn from_obj_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("failed to read obj file");

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                // vertex
                "v" => {
                    if parts.len() < 4 {
                        continue;
                    }

                    let x: f32 = parts[1].parse().unwrap();
                    let y: f32 = parts[2].parse().unwrap();
                    let z: f32 = parts[3].parse().unwrap();

                    vertices.push(Vec3::new(x, y, z));
                }

                // face
                "f" => {
                    if parts.len() < 4 {
                        continue;
                    }

                    let parse_index = |s: &str| {
                        s.split('/').next().unwrap().parse::<usize>().unwrap() - 1 // OBJ is 1-based
                    };

                    let i0 = parse_index(parts[1]);
                    let i1 = parse_index(parts[2]);
                    let i2 = parse_index(parts[3]);

                    triangles.push((i0, i1, i2));

                    // handle quads (f a b c d → two triangles)
                    if parts.len() == 5 {
                        let i3 = parse_index(parts[4]);
                        triangles.push((i0, i2, i3));
                    }
                }

                _ => {}
            }
        }

        Self {
            vertices,
            triangles,
        }
    }
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

    Mesh {
        vertices,
        triangles,
    }
}
