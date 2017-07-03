use app::vector::Vec2d;
use app::brush::Brush;
use std::ops::{Index, Sub};

#[derive(Clone)]
pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
    pub pressure: f64,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub timestamp: i64,
    pub dragging: bool,
}

#[derive(Clone)]
pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub brush: Brush,
    pub finished: bool,
}

impl Index<usize> for Stroke {
    type Output = StrokePoint;
    fn index(&self, i: usize) -> &StrokePoint {
        &self.points[i]
    }
}

impl Stroke {
    pub fn new(initial_size: usize, brush: Brush) -> Stroke {
        Stroke {
            points: vec![],
            brush: brush,
            finished: false,
        }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    pub fn extend(&mut self, rhs: Stroke) {
        self.points.extend(rhs.points);
    }

    pub fn push(&mut self, item: StrokePoint) {
        self.points.push(item);
    }
}
