extern crate ui;

use std::rc::Rc;
use std::cell::RefCell;

mod reactive;
use reactive::widget::{Window, WindowHandler, Container, Button, ButtonHandler, Slider, SliderHandler, Area, AreaHandler, AreaDrawParams};
use reactive::widget::{Model, HandlerType, Component}; // TODO move to good place

mod message;
use message::Message;

mod app;
use app::canvas;

pub struct PoheModel {
    value: i64,
}

impl PoheModel {
    pub fn new() -> PoheModel {
        PoheModel {
            value: 0,
        }
    }
}

impl Model<Message> for PoheModel {
    fn update(&mut self, message: &Message, widget_handler: &mut HandlerType) {
    }
}

pub struct HoeModel {
    count: i64,
}

impl HoeModel {
    pub fn new() -> HoeModel {
        HoeModel {
            count: 0,
        }
    }
}

impl Model<Message> for HoeModel {
    fn update(&mut self, message: &Message, widget_handler: &mut HandlerType) {
    }
}

fn main() {
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    reactive::init();
    let components = Window::new("SketchBook", width as i32, height as i32, false)
        .on_closing(|w| {
            reactive::quit();
            false
        })
        .set_child(
            Container::new_horizontal()
                .set_padding(true)
                .append(Container::new_vertical()
                    .append(Button::new("toggle")
                        .on_click(|button| {
                            reactive::emit(Message::BrushToggleButton);
                        })
                        .set_model(Rc::new(RefCell::new(HoeModel::new()))))
                    .append(Button::new("close stroke")
                        .on_click(|button| {
                            reactive::emit(Message::StrokeCloseButton);
                        }))
                    .append(Slider::new(0, 10)
                        .on_change(|slider| {
                            reactive::emit(Message::BrushSliderUpdate(slider.value()));
                        })
                        .set_model(Rc::new(RefCell::new(PoheModel::new())))))
                .append(Area::new(Rc::new(RefCell::new(canvas::CanvasModel::new(width, height)))))
        )
        .show()
        .get_components();
    println!("{:?}", components);
    reactive::main(components);
}

