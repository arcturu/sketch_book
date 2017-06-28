extern crate ui;

use ui::{Area, InitOptions, Window};

mod canvas_view;
mod stroke;
mod vector;

fn main() {
    let w = 640;
    let h = 480;
    ui::init(InitOptions).unwrap();
    let window = Window::new("SketchBook", w, h, true);
    window.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    let canvas_area = Area::new(Box::new(canvas_view::CanvasArea::new(w as f64, h as f64)));
    window.set_child(canvas_area.into());
    window.show();
    ui::main();
//    ui::uninit();
}

