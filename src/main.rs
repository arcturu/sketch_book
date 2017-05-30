extern crate ui;
use ui::{InitOptions, Window};

fn main() {
    ui::init(InitOptions).unwrap();
    let window = Window::new("SketchBook", 640, 480, true);
    window.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));
    window.show();
    ui::main();
//    ui::uninit();
}
