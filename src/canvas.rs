extern crate ui;
use ui::{Area, AreaMouseEvent, AreaHandler};

pub struct CanvasArea;

impl AreaHandler for CanvasArea {
    fn mouse_event(&mut self, _area: &Area, area_mouse_event: &AreaMouseEvent) {
        println!("{:?}", area_mouse_event);
    }
}

