extern crate ui;
use ui::{Area, AreaMouseEvent, AreaHandler, AreaDrawParams, Image};

pub struct CanvasArea;

impl AreaHandler for CanvasArea {
    fn mouse_event(&mut self, _area: &Area, area_mouse_event: &AreaMouseEvent) {
        println!("{:?}", area_mouse_event);
    }
    fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
        let x: f64 = 256.0;
        let y: f64 = 256.0;
        let mut image: Image = Image::new(x, y);
        let mut col: u8 = 0;
        for _ in 0..((x * y) as usize) {
            for _ in 0..3 {
                image.data.push(col);
            }
            image.data.push(255);
            col = ((col as u32 + 1) % 256) as u8;
        }
        area_draw_params.context.draw_image(0.0, 0.0, x, y, &mut image);
    }
}

