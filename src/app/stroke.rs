pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
    pub pressure: f64,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub timestamp: i64,
}

pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub finished: bool,
}

impl Stroke {
    pub fn new(initial_size: usize) -> Stroke {
        Stroke {
            points: vec![],
            finished: false,
        }
    }
}
