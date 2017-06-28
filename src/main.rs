extern crate ui;

use ui::{Area, InitOptions, Window};

mod canvas_view;
mod stroke;

fn main() {
    ui::init(InitOptions).unwrap();
    let window = Window::new("SketchBook", 640, 480, true);
    window.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    let canvas_area = Area::new(Box::new(canvas_view::CanvasArea::new(640.0, 480.0)));
    window.set_child(canvas_area.into());
    window.show();
    ui::main();
//    ui::uninit();
}

