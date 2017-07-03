extern crate ui;
extern crate time;

use message::Message;
use reactive;
use reactive::widget::{HandlerType, Model, AreaDrawParams, AreaMouseEvent, AreaHandler, AreaCallbacks};

use app::stroke::{Stroke, StrokePoint};
use app::vector::{Vec2d};
use app::config::{Config};
use app::brush::{Brush};

pub struct CanvasImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
    color_depth: u32, // in byte
    strokes: Vec<Stroke>,
    current_brush: Brush,
}

impl CanvasImage {
    pub fn new(w: u32, h: u32) -> CanvasImage {
        let color_depth = 1; // 1 byte = 0..255 per color
        CanvasImage {
            data: vec![255; (w * h * color_depth * 4) as usize],
            width: w,
            height: h,
            color_depth: color_depth,
            strokes: vec![],
            current_brush: Brush::new(),
        }
    }
    pub fn draw_stroke_dots(&mut self) {
        for s in &self.strokes {
            for p in &s.points {
                if (p.x as u32) < self.width && (p.y as u32) < self.height {
                    let i = ((p.y as usize) * (self.width as usize) + (p.x as usize)) * 4;
                    for j in 0..3 {
                        self.data[i + j] = 0;
                    }
                }
            }
        }
    }
    pub fn fill_circle(&mut self, c: Vec2d, r: f64) {
        let r_int = r as i32;
        for offset_y in -r_int..r_int {
            for offset_x in -r_int..r_int {
                let cx_int = c.x as i32;
                let cy_int = c.y as i32;
                let x_int = (c.x as i32) + (offset_x as i32);
                let y_int = (c.y as i32) + (offset_y as i32);
                if (x_int - cx_int).pow(2) + (y_int - cy_int).pow(2) < r_int.pow(2)
                   && x_int >= 0 && x_int < self.width as i32
                   && y_int >= 0 && y_int < self.height as i32 {
                    let col = self.current_brush.get_color();
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 0) as usize] = (col.r * 255.0) as u8;
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 1) as usize] = (col.g * 255.0) as u8;
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 2) as usize] = (col.b * 255.0) as u8;
                }
            }
        }
    }
    pub fn draw_line_with_circle(&mut self, p0: Vec2d, p1: Vec2d, r: f64) {
        let d = p1 - p0;
        for t in 0..(d.len() as usize) + 1 {
            self.fill_circle(p0 + d.normalize().smul(t as f64), r);
        }
    }
    pub fn draw_stroke_sweep_circle(&mut self, r: f64) {
        for i in 0..self.strokes.len() {
            for j in 1..self.strokes[i].points.len() {
                let p0 = Vec2d::new(self.strokes[i].points[j-1].x as f64, self.strokes[i].points[j-1].y as f64);
                let p1 = Vec2d::new(self.strokes[i].points[j].x as f64, self.strokes[i].points[j].y as f64);
                self.draw_line_with_circle(p0, p1, r);
            }
        }
    }
    pub fn draw_stroke(&mut self) {
        let r = self.current_brush.size;
        self.draw_stroke_sweep_circle(r);
    }
    pub fn draw_stroke_incremental(&mut self) {
        if self.strokes.len() > 0 {
            let r = self.current_brush.size;
            let last = self.strokes.len() - 1;
            let len = self.strokes[last].points.len();
            if len == 1 {
                let x = self.strokes[last].points[len-1].x;
                let y = self.strokes[last].points[len-1].y;
                self.fill_circle(Vec2d::new(x, y), r);
            } else if len > 1 {
                let x0 = self.strokes[last].points[len-2].x;
                let y0 = self.strokes[last].points[len-2].y;
                let x1 = self.strokes[last].points[len-1].x;
                let y1 = self.strokes[last].points[len-1].y;
                self.draw_line_with_circle(Vec2d::new(x0, y0), Vec2d::new(x1, y1), r);
            }
        }
    }
    pub fn get_stroke_point(&self, e: &AreaMouseEvent) -> StrokePoint {
        // TODO support canvas resizing
        StrokePoint {
            x: e.x,
            y: e.y,
            pressure: 0.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            timestamp: time::now().to_timespec().sec,
        }
    }
    pub fn mouse_event(&mut self, e: &AreaMouseEvent) -> bool {
        if e.held_1_to_64 == 1 {
            let point = self.get_stroke_point(e);
            let mut new_stroke = match self.strokes.pop() {
                Some(s) => if !s.finished { s } else { Stroke::new(10, self.current_brush.clone()) },
                None => Stroke::new(10, self.current_brush.clone()),
            };
            new_stroke.points.push(point);
            self.strokes.push(new_stroke);
            self.draw_stroke_incremental();
            return true
        } else if self.strokes.len() > 0 {
            self.strokes.last_mut().unwrap().finished = true;
            return false
        }
        return false
    }
}


pub struct CanvasModel {
    canvas_image: CanvasImage,
//    config: Rc<Config>,
    width: f64,
    height: f64,
}

impl Model<Message> for CanvasModel {
    fn update(&mut self, message: &Message, widget_handler: &mut HandlerType) {
        match message {
            &Message::BrushSliderUpdate(size) => {
                self.canvas_image.current_brush.size = size as f64;
            },
            &Message::BrushToggleButton => {
                self.canvas_image.current_brush.contour = !self.canvas_image.current_brush.contour;
                // TODO end stroke if valid and begen new one
            }
            _ => (),
        }
        if let &mut HandlerType::Area(ref area) = widget_handler {
            area.queue_redraw_all();
        }
    }
}

impl AreaCallbacks for CanvasModel {
    fn on_draw(&mut self, area: &AreaHandler, area_draw_params: &AreaDrawParams) {
        let mut image = ui::Image::new(self.canvas_image.width as f64, self.canvas_image.height as f64);
        image.data = self.canvas_image.data.to_vec(); // deep copy
        area_draw_params.context.draw_image(0.0, 0.0, image.width, image.height, &mut image);
    }

    fn on_mouse_event(&mut self, area: &AreaHandler, area_mouse_event: &AreaMouseEvent) {
        if self.canvas_image.mouse_event(area_mouse_event) {
            area.queue_redraw_all();
        }
    }
}

impl CanvasModel {
    pub fn new(w: f64, h: f64) -> CanvasModel {
        CanvasModel {
            canvas_image: CanvasImage::new(w as u32, h as u32),
            width: w,
            height: h,
        }
    }
}
