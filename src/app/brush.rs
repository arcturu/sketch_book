use app::color::Color;

#[derive(Clone, PartialEq)]
pub enum BrushTip {
    Contour,
}

#[derive(Clone)]
pub struct Brush {
    pub tip: BrushTip,
    pub size: f64,
    pub color: Color<f64>,
}

impl Brush {
    pub fn new() -> Brush {
        Brush {
            tip: BrushTip::Contour,
            size: 2.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn get_color(&self) -> &Color<f64> {
        &self.color
    }
}
