extern crate ui;
use ui::{Area, AreaMouseEvent, AreaHandler, AreaDrawParams, Image};

pub struct CanvasArea {
    data: Vec<u8>,
    x: f64,
    y: f64,
}

impl CanvasArea {
    pub fn new(x: f64, y: f64) -> CanvasArea {
        let mut data:Vec<u8> = Vec::with_capacity((x * y * 4.0) as usize);
        for _ in 0..((x * y) as usize) {
            for _ in 0..4 {
                data.push(255);
            }
        }
        CanvasArea {
            data: data,
            x: x,
            y: y,
        }
    }
}

impl AreaHandler for CanvasArea {
    fn mouse_event(&mut self, area: &Area, area_mouse_event: &AreaMouseEvent) {
        println!("{:?}", area_mouse_event);
        if area_mouse_event.held_1_to_64 == 0 {
            return;
        }
        let i: usize = ((area_mouse_event.y as usize) * (self.x as usize) + (area_mouse_event.x as usize)) * 4;
        for j in 0..3 {
            self.data[i + j] = 0;
        }
        area.queue_redraw_all();
    }
    fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
        let mut image: Image = Image::new(self.x, self.y);
        image.data = self.data.to_vec();
        area_draw_params.context.draw_image(0.0, 0.0, self.x, self.y, &mut image);
        println!("draw {} {}", self.x, self.y);
    }
}

