use crate::{mesh::Mesh, vector::Vec3};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

pub fn trace(ray: &Ray, mesh: &Mesh) -> u32 {
    let mut closest_t = f32::INFINITY;
    let mut hit_normal = None;

    for &(i0, i1, i2) in &mesh.triangles {
        let v0 = mesh.vertices[i0];
        let v1 = mesh.vertices[i1];
        let v2 = mesh.vertices[i2];

        if let Some(t) = intersect_triangle(ray, v0, v1, v2) {
            if t < closest_t {
                closest_t = t;

                let normal = (v1 - v0).cross(v2 - v0).normalized();
                hit_normal = Some(normal);
            }
        }
    }

    if let Some(normal) = hit_normal {
        let light = Vec3::new(1.0, 1.0, -1.0).normalized();
        let intensity = normal.dot(light).max(0.0);

        let c = (intensity * 255.0) as u32;
        return c << 16;
    }

    0x000022
}

pub fn intersect_triangle(ray: &Ray, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
    let eps = 1e-6;

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;

    let h = ray.dir.cross(edge2);
    let a = edge1.dot(h);

    if a.abs() < eps {
        return None; // parallel
    }

    let f = 1.0 / a;
    let s = ray.origin - v0;

    let u = f * s.dot(h);
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = s.cross(edge1);
    let v = f * ray.dir.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = f * edge2.dot(q);

    if t > eps { Some(t) } else { None }
}
