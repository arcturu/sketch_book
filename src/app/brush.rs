use app::color::Color;

#[derive(Clone)]
pub struct Brush {
    pub contour: bool, // TODO temporal
    pub size: f64,
}

impl Brush {
    pub fn new() -> Brush {
        Brush {
            contour: true,
            size: 2.0,
        }
    }

    pub fn get_color(&self) -> Color {
        if self.contour {
            Color::new(0.0, 0.0, 0.0)
        } else {
            Color::new(0.0, 0.0, 1.0)
        }
    }
}
