use std::ops::{Add, Sub};

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
        self.sdiv(self.len())
    }
}
