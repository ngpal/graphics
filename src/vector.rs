use std::f32::consts::PI;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn scaled(&self, scalar: f32) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
            .sqrt()
            .abs()
    }

    pub fn normalized(&self) -> Vec3 {
        self.scaled(1. / self.magnitude())
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[derive(Copy, Clone)]
pub struct Quat {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quat {
    pub fn identity() -> Self {
        Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from_axis_angle(axis: Vec3, deg: f32) -> Self {
        let half = deg * PI / 180.0 * 0.5;
        let s = half.sin();
        Self { w: half.cos(), x: axis.x * s, y: axis.y * s, z: axis.z * s }
    }

    pub fn rotate(&self, v: Vec3) -> Vec3 {
        let qv = Vec3::new(self.x, self.y, self.z);
        let t = qv.cross(v).scaled(2.0);
        v + t.scaled(self.w) + qv.cross(t)
    }
}

impl Mul for Quat {
    type Output = Quat;

    fn mul(self, r: Quat) -> Quat {
        Quat {
            w: self.w*r.w - self.x*r.x - self.y*r.y - self.z*r.z,
            x: self.w*r.x + self.x*r.w + self.y*r.z - self.z*r.y,
            y: self.w*r.y - self.x*r.z + self.y*r.w + self.z*r.x,
            z: self.w*r.z + self.x*r.y - self.y*r.x + self.z*r.w,
        }
    }
}
