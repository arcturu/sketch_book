extern crate ui;
use ui::{Area, AreaMouseEvent, AreaHandler, InitOptions, Window};

struct CanvasArea;

impl AreaHandler for CanvasArea {
    fn mouse_event(&mut self, _area: &Area, area_mouse_event: &AreaMouseEvent) {
        println!("{:?}", area_mouse_event);
    }
}

fn main() {
    ui::init(InitOptions).unwrap();
    let window = Window::new("SketchBook", 640, 480, true);
    window.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    let canvas_area = Area::new(Box::new(CanvasArea));
    window.set_child(canvas_area.into());
    window.show();
    ui::main();
//    ui::uninit();
}
