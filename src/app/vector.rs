use std::ops::{Add, Sub};
use app::color::{Color};

#[derive(Copy, Clone, Debug)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Vec2d {
        Vec2d { x: x, y: y }
    }
    pub fn smul(&self, c: f64) -> Vec2d {
        Vec2d { x: self.x * c, y: self.y * c }
    }
    pub fn sdiv(&self, c: f64) -> Vec2d {
        Vec2d { x: self.x / c, y: self.y / c }
    }
    pub fn dot(self, other: Vec2d) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn len(&self) -> f64 {
        self.norm().sqrt()
    }
    pub fn normalize(&self) -> Vec2d {
        if self.len() != 0.0 {
            self.sdiv(self.len())
        } else {
            self.clone()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3d {
        Vec3d { x: x, y: y, z: z }
    }
    pub fn from_vec2d(v: Vec2d) -> Vec3d {
        Vec3d { x: v.x, y: v.y, z: 0.0 }
    }
    pub fn smul(&self, c: f64) -> Vec3d {
        Vec3d { x: self.x * c, y: self.y * c, z: self.z * c }
    }
    pub fn sdiv(&self, c: f64) -> Vec3d {
        Vec3d { x: self.x / c, y: self.y / c, z: self.z / c }
    }
    pub fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f64 {
        self.norm().sqrt()
    }
    pub fn normalize(&self) -> Vec3d {
        if self.len() != 0.0 {
            self.sdiv(self.len())
        } else {
            self.clone()
        }
    }
    pub fn to_color(self) -> Color<u8> {
        let s = self.normalize().smul(127.5);
        Color {
            r: (s.x + 127.5) as u8,
            g: (s.y + 127.5) as u8,
            b: (s.z + 127.5) as u8,
            a: 255,
        }
    }
}
