use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

extern crate ui;

pub trait Widget<T, M> {
    fn get_ui_control(&self) -> ui::Control;
    fn get_components(self) -> Vec<Box<Component<M>>>;
}

pub trait Model<M> {
    fn update(&mut self, message: &M, widget_handler: &mut HandlerType);
}

pub enum HandlerType {
    Button(ButtonHandler),
    Slider(SliderHandler),
    Container(ContainerHandler),
    Area(AreaHandler),
}
impl Clone for HandlerType {
    fn clone(&self) -> HandlerType {
        match self {
            &HandlerType::Button(ref h) => HandlerType::Button(h.clone()),
            &HandlerType::Slider(ref h) => HandlerType::Slider(h.clone()),
            &HandlerType::Container(ref h) => HandlerType::Container(h.clone()),
            &HandlerType::Area(ref h) => HandlerType::Area(h.clone()),
        }
    }
}

pub struct Component<M> {
    pub ui_control: HandlerType,
    pub model: Rc<RefCell<Model<M>>>,
}

impl<M> fmt::Debug for Component<M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty = match self.ui_control {
            HandlerType::Button(_) => "button",
            HandlerType::Slider(_) => "slider",
            HandlerType::Container(_) => "container",
            HandlerType::Area(_) => "area",
        };
        write!(f, "Hi: {}", ty)
    }
}

#[macro_export]
macro_rules! define_widget {
    ($rust_type:ident, $handler_type:ident, $ui_type:ty) => {
        pub struct $rust_type<M> {
            ui_control: $ui_type,
            components: Vec<Box<Component<M>>>,
        }

        pub struct $handler_type {
            ui_control: $ui_type,
        }

        impl<M> Widget<$handler_type, M> for $rust_type<M> {
            #[inline]
            fn get_ui_control(&self) -> ui::Control {
                self.ui_control.clone().into()
            }

            fn get_components(self) -> Vec<Box<Component<M>>> {
                self.components
            }
        }

        impl Clone for $handler_type {
            fn clone(&self) -> $handler_type {
                $handler_type {
                    ui_control: self.ui_control.clone(),
                }
            }
        }

        impl<M> $rust_type<M> {
            pub fn set_model(mut self, model: Rc<RefCell<Model<M>>>) -> $rust_type<M> {
                self.components.push(
                    Box::new(Component {
                        ui_control: HandlerType::$rust_type($handler_type { ui_control: self.ui_control.clone() }),
                        model: model,
                    })
                );
                self
            }
        }
    }
}

pub struct Window<M> {
    ui_window: ui::Window,
    components: Vec<Box<Component<M>>>
}

pub struct WindowHandler {
    ui_window: ui::Window,
}

impl WindowHandler {
    pub fn destroy(&self) {
        unsafe {
            &self.ui_window.destroy();
        }
    }
}

impl<M> Window<M> {
    pub fn new(title: &str, width: i32, height: i32, has_menubar: bool) -> Window<M> {
        Window {
            ui_window: ui::Window::new(title, width, height, has_menubar),
            components: Vec::new(),
        }
    }

    pub fn from_raw_ui_window(ui_window: &ui::Window) -> WindowHandler {
        WindowHandler {
            ui_window: ui_window.clone(),
        }
    }

    pub fn set_title(self, title: &str) -> Window<M> {
        self.ui_window.set_title(title);
        self
    }

    pub fn on_closing<F: 'static>(self, callback: F) -> Window<M> where F: Fn(&WindowHandler) -> bool {
        self.ui_window.on_closing(Box::new(move |ui_window| {
            callback(&WindowHandler{ ui_window: ui_window.clone() })
        }));
        self
    }

    pub fn set_child<T, W: Widget<T, M>>(mut self, widget: W) -> Window<M> {
        self.ui_window.set_child(widget.get_ui_control());
        self.components.extend(widget.get_components());
        self
    }

    pub fn show(self) -> Self {
        self.ui_window.show();
        self
    }

    pub fn get_components(self) -> Vec<Box<Component<M>>> {
        self.components
    }
}

define_widget!(Container, ContainerHandler, ui::BoxControl);
impl<M> Container<M> {
    pub fn new_vertical() -> Container<M> {
        Container {
            ui_control: ui::BoxControl::new_vertical(),
            components: Vec::new(),
        }
    }

    pub fn new_horizontal() -> Container<M> {
        Container {
            ui_control: ui::BoxControl::new_horizontal(),
            components: Vec::new(),
        }
    }

    pub fn set_padding(self, set_padding: bool) -> Container<M> {
        self.ui_control.set_padded(set_padding);
        self
    }

    pub fn append<T, W: Widget<T, M>>(mut self, widget: W) -> Container<M> {
        self.ui_control.append(widget.get_ui_control(), false);
        self.components.extend(widget.get_components());
        self
    }

    pub fn append_stretchy<T, W: Widget<T, M>>(mut self, widget: W) -> Container<M> {
        self.ui_control.append(widget.get_ui_control(), true);
        self.components.extend(widget.get_components());
        self
    }
}

define_widget!(Button, ButtonHandler, ui::Button);
impl<M> Button<M> {
    pub fn new(text: &str) -> Button<M> {
        Button {
            ui_control: ui::Button::new(text),
            components: Vec::new(),
        }
    }

    pub fn on_click<F: 'static>(self, callback: F) -> Button<M> where F: Fn(&ButtonHandler) -> () {
        self.ui_control.on_clicked(Box::new(move |ui_button| {
            callback(&mut ButtonHandler { ui_control: ui_button.clone() })
        }));
        self
    }
}

define_widget!(Slider, SliderHandler, ui::Slider);
impl<M> Slider<M> {
    pub fn new(min: i64, max: i64) -> Slider<M> {
        Slider {
            ui_control: ui::Slider::new(min, max),
            components: Vec::new(),
        }
    }

    pub fn on_change<F: 'static>(self, callback: F) -> Slider<M> where F: Fn(&SliderHandler) -> () {
        self.ui_control.on_changed(Box::new(move |ui_slider| {
            callback(&mut SliderHandler { ui_control: ui_slider.clone() })
        }));
        self
    }
}

impl SliderHandler {
    pub fn set_value(&self, value: i64) {
        self.ui_control.set_value(value);
    }

    pub fn value(&self) -> i64 {
        self.ui_control.value()
    }
}

pub struct Area<M> {
    ui_control: ui::Area,
    components: Vec<Box<Component<M>>>,
}

pub struct AreaHandler {
    ui_control: ui::Area,
}

impl Clone for AreaHandler {
    fn clone(&self) -> AreaHandler {
        AreaHandler {
            ui_control: self.ui_control.clone(),
        }
    }
}

impl AreaHandler {
    pub fn queue_redraw_all(&self) {
        self.ui_control.queue_redraw_all();
    }
}

impl<M> Widget<AreaHandler, M> for Area<M> { fn get_ui_control(&self) -> ui::Control {
        self.ui_control.clone().into()
    }
    fn get_components(self) -> Vec<Box<Component<M>>> {
        self.components
    }
}

impl<M> Area<M> {
    pub fn set_model(mut self, model: Rc<RefCell<Model<M>>>) -> Area<M> {
        self.components.push(
            Box::new(Component {
                ui_control: HandlerType::Area(AreaHandler { ui_control: self.ui_control.clone() }),
                model: model,
            })
        );
        self
    }
}

pub type AreaDrawParams = ui::AreaDrawParams;
pub type AreaMouseEvent = ui::AreaMouseEvent;
pub type AreaKeyEvent = ui::AreaKeyEvent;

pub trait AreaCallbacks {
    fn on_draw(&mut self, area: &AreaHandler, area_draw_params: &AreaDrawParams) {}
    fn on_mouse_event(&mut self, area: &AreaHandler, area_mouse_event: &AreaMouseEvent) {}
    fn on_mouse_crossed(&mut self, area: &AreaHandler, left: bool) {}
    fn on_drag_broken(&mut self, area: &AreaHandler) {}
    fn on_key_event(&mut self, area: &AreaHandler, area_key_event: &AreaKeyEvent) -> bool {
        false
    }
}

impl<M: 'static> Area<M> {
    pub fn new<T>(callbacks: Rc<RefCell<T>>) -> Area<M> where T: AreaCallbacks + Model<M> + 'static {
        let ui_control = ui::Area::new(Box::new(AreaWrapper::new(callbacks.clone())));
        Area {
            ui_control: ui_control.clone(),
            components: vec![
                Box::new(Component {
                    ui_control: HandlerType::Area(AreaHandler { ui_control: ui_control }),
                    model: callbacks.clone(),
                })
            ],
        }
    }
}

struct AreaWrapper {
    callbacks: Rc<RefCell<AreaCallbacks>>,
}

impl AreaWrapper {
    fn new(callbacks: Rc<RefCell<AreaCallbacks>>) -> AreaWrapper {
        AreaWrapper {
            callbacks: callbacks,
        }
    }
}

impl ui::AreaHandler for AreaWrapper {
    fn draw(&mut self, area: &ui::Area, area_draw_params: &ui::AreaDrawParams) {
        self.callbacks.borrow_mut().on_draw(&AreaHandler {ui_control: area.clone()}, area_draw_params);
    }
    fn mouse_event(&mut self, area: &ui::Area, area_mouse_event: &ui::AreaMouseEvent) {
        self.callbacks.borrow_mut().on_mouse_event(&AreaHandler {ui_control: area.clone()}, area_mouse_event);
    }
    fn mouse_crossed(&mut self, area: &ui::Area, left: bool) {
        self.callbacks.borrow_mut().on_mouse_crossed(&AreaHandler {ui_control: area.clone()}, left);
    }
    fn drag_broken(&mut self, area: &ui::Area) {
        self.callbacks.borrow_mut().on_drag_broken(&AreaHandler {ui_control: area.clone()});
    }
    fn key_event(&mut self, area: &ui::Area, area_key_event: &ui::AreaKeyEvent) -> bool {
        self.callbacks.borrow_mut().on_key_event(&AreaHandler {ui_control: area.clone()}, area_key_event)
    }
}
