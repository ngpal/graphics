use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

pub fn trace(ray: &Ray) -> u32 {
    let sphere_center = Vec3::new(0.0, 0.0, 3.0);
    let radius = 1.0;

    let oc = ray.origin - sphere_center;

    let a = ray.dir.dot(ray.dir);
    let b = 2.0 * oc.dot(ray.dir);
    let c = oc.dot(oc) - radius * radius;

    let disc = b * b - 4.0 * a * c;

    if disc < 0.0 {
        return 0x000022; // background
    }

    let t = (-b - disc.sqrt()) / (2.0 * a);

    if t <= 0.0 {
        return 0x000022;
    }

    let hit = ray.origin + ray.dir.scaled(t);
    let normal = (hit - sphere_center).normalized();

    let light = Vec3::new(1.0, 1.0, -1.0).normalized();
    let intensity = normal.dot(light).max(0.0);

    let c = (intensity * 255.0) as u32;
    c << 16
}
