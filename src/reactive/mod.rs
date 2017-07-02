extern crate ui;

pub mod widget;

use std::cell::RefCell;
use reactive::widget::Component;
use message::Message;

pub fn init() {
    ui::init(ui::InitOptions).unwrap();
}

thread_local! {
    static COMPONENTS: RefCell<Vec<Box<Component<Message>>>> = RefCell::new(Vec::new());
}

pub fn emit(message: Message) {
    COMPONENTS.with(|cs| {
        for c in cs.borrow_mut().iter_mut() {
            let mut ui_control = c.ui_control.clone();
            if let Ok(mut model) = c.model.try_borrow_mut() {
                model.update(&message, &mut ui_control);
            }
        }
    });
}

pub fn main(components: Vec<Box<Component<Message>>>) {
    COMPONENTS.with(move |cs| { cs.borrow_mut().extend(components) });
    ui::main();
}

pub fn quit() {
    ui::quit();
}

