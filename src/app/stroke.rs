use app::brush::Brush;

pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
    pub pressure: f64,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub timestamp: i64,
    pub dragging: bool,
}

pub struct Stroke {
    pub points: Vec<StrokePoint>,
    pub brush: Brush,
    pub finished: bool,
}

impl Stroke {
    pub fn new(initial_size: usize, brush: Brush) -> Stroke {
        Stroke {
            points: vec![],
            brush: brush,
            finished: false,
        }
    }
}
