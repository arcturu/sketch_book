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
//    strokes: Vec<Stroke>,
//    current_brush: Brush,
}

impl CanvasImage {
    pub fn new(w: u32, h: u32, init_value: u8) -> CanvasImage {
        let color_depth = 1; // 1 byte = 0..255 per color
        CanvasImage {
            data: vec![init_value; (w * h * color_depth * 4) as usize],
            width: w,
            height: h,
            color_depth: color_depth,
        }
    }
//    pub fn draw_stroke_dots(&mut self) {
//        for s in &self.strokes {
//            for p in &s.points {
//                if (p.x as u32) < self.width && (p.y as u32) < self.height {
//                    let i = ((p.y as usize) * (self.width as usize) + (p.x as usize)) * 4;
//                    for j in 0..3 {
//                        self.data[i + j] = 0;
//                    }
//                }
//            }
//        }
//    }
    pub fn fill_circle(&mut self, c: Vec2d, brush: &Brush) {
        let r_int = brush.size as i32;
        for offset_y in -r_int..r_int {
            for offset_x in -r_int..r_int {
                let cx_int = c.x as i32;
                let cy_int = c.y as i32;
                let x_int = (c.x as i32) + (offset_x as i32);
                let y_int = (c.y as i32) + (offset_y as i32);
                if (x_int - cx_int).pow(2) + (y_int - cy_int).pow(2) < r_int.pow(2)
                   && x_int >= 0 && x_int < self.width as i32
                   && y_int >= 0 && y_int < self.height as i32 {
                    let col = brush.get_color();
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 0) as usize] = (col.r * 255.0) as u8;
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 1) as usize] = (col.g * 255.0) as u8;
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 2) as usize] = (col.b * 255.0) as u8;
                    self.data[((y_int * (self.width as i32) + x_int) * 4 + 3) as usize] = 255;
                }
            }
        }
    }
    pub fn draw_line_with_circle(&mut self, p0: Vec2d, p1: Vec2d, brush: &Brush) {
        let d = p1 - p0;
        for t in 0..(d.len() as usize) + 1 {
            self.fill_circle(p0 + d.normalize().smul(t as f64), brush);
        }
    }
    pub fn draw_stroke_sweep_circle(&mut self, strokes: &Vec<Stroke>, brush: &Brush) {
        for i in 0..strokes.len() {
            for j in 1..strokes[i].points.len() {
                let p0 = Vec2d::new(strokes[i].points[j-1].x as f64, strokes[i].points[j-1].y as f64);
                let p1 = Vec2d::new(strokes[i].points[j].x as f64, strokes[i].points[j].y as f64);
                self.draw_line_with_circle(p0, p1, brush);
            }
        }
    }
    pub fn draw_stroke(&mut self, strokes: &Vec<Stroke>, brush: &Brush) {
        self.draw_stroke_sweep_circle(strokes, brush);
    }
    pub fn draw_stroke_incremental(&mut self, strokes: &Vec<Stroke>, brush: &Brush) {
        if strokes.len() > 0 {
            let last = strokes.len() - 1;
            let len = strokes[last].points.len();
            if len == 1 {
                let x = strokes[last].points[len-1].x;
                let y = strokes[last].points[len-1].y;
                self.fill_circle(Vec2d::new(x, y), brush);
            } else if len > 1 {
                let x0 = strokes[last].points[len-2].x;
                let y0 = strokes[last].points[len-2].y;
                let x1 = strokes[last].points[len-1].x;
                let y1 = strokes[last].points[len-1].y;
                self.draw_line_with_circle(Vec2d::new(x0, y0), Vec2d::new(x1, y1), brush);
            }
        }
    }
}

pub enum BlendMode {
    Normal,
}

pub struct Layer {
    visible: bool,
    blend_mode: BlendMode,
    image: CanvasImage,
    strokes: Vec<Stroke>,
}

impl Layer {
    pub fn new(width: u32, height: u32, init_value: u8) -> Layer {
        Layer {
            visible: true,
            blend_mode: BlendMode::Normal,
            image: CanvasImage::new(width, height, init_value),
            strokes: vec![],
        }
    }
    pub fn mouse_event(&mut self, brush: &Brush, e: StrokePoint) -> bool {
        if e.dragging {
            let mut new_stroke = match self.strokes.pop() {
                Some(s) => if !s.finished { s } else {
                    self.strokes.push(s);
                    Stroke::new(10, brush.clone())
                },
                None => Stroke::new(10, brush.clone()),
            };
            new_stroke.points.push(e);
            self.strokes.push(new_stroke);
            self.image.draw_stroke_incremental(&self.strokes, brush);
            return true
        } else if self.strokes.len() > 0 {
            self.strokes.last_mut().unwrap().finished = true;
            return false
        }
        return false
    }

    pub fn composite(&self, data: &mut Vec<u8>) {
        match self.blend_mode {
            BlendMode::Normal => {
                for i in 0..(data.len() / 4 / self.image.color_depth as usize) {
                    let j = i * 4;
                    let a_back = data[j+3] as f64 / 255.0;
                    let a_front = self.image.data[j+3] as f64 / 255.0;
                    data[j+0] = (self.image.data[j+0] as f64 * a_front + data[j+0] as f64 * a_back * (1.0 - a_front)) as u8;
                    data[j+1] = (self.image.data[j+1] as f64 * a_front + data[j+1] as f64 * a_back * (1.0 - a_front)) as u8;
                    data[j+2] = (self.image.data[j+2] as f64 * a_front + data[j+2] as f64 * a_back * (1.0 - a_front)) as u8;
                    data[j+3] = ((a_front + a_back * (1.0 - a_front)) * 255.0) as u8;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.strokes = vec![];
        self.image.data = vec![0; (self.image.width * self.image.height * self.image.color_depth * 4) as usize];
    }
}

pub struct CanvasModel {
    layers: Vec<Layer>,
    active_layer: usize,
//    config: Rc<Config>,
    width: f64,
    height: f64,
    current_brush: Brush, // TODO move it to config
}

fn connect_closed_points(mut strokes: Vec<Stroke>) -> Vec<Stroke> {
    let mut min = ::std::f64::INFINITY;
    let mut min_i = 0;
    let mut min_j = 0;
    let mut min_k = 0;
    let mut min_l = 0;
    for i in 0..strokes.len() {
        for j in i..strokes.len() {
            if i == j { continue; }
            if strokes[i].len() < 2 || strokes[j].len() < 2 {
                println!("short stroke!"); // TODO FIXME
                continue;
            }
            for k in 0..2 {
                for l in 0..2 {
                    let m1 = if k == 0 { &strokes[i][0] } else { &strokes[i][strokes[i].len()-1] };
                    let m2 = if l == 0 { &strokes[j][0] } else { &strokes[j][strokes[j].len()-1] };
                    let d = (Vec2d::new(m1.x, m1.y) - Vec2d::new(m2.x, m2.y)).norm();
                    if d < min {
                        min = d; min_i = i; min_j = j; min_k = k; min_l = l;
                    }
                }
            }
        }
    }
    let mut s1;
    let mut s2;
    if min_i < min_j {
        s2 = strokes.swap_remove(min_j);
        s1 = strokes.swap_remove(min_i);
    } else {
        s1 = strokes.swap_remove(min_i);
        s2 = strokes.swap_remove(min_j);
    }
    if min_k > min_l {
        s1.extend(s2);
        strokes.push(s1);
    } else if min_k == min_l {
        if min_k == 1 {
            s1.reverse();
            s2.extend(s1);
            strokes.push(s2);
        } else {
            s2.reverse();
            s1.extend(s2);
            strokes.push(s1);
        }
    } else {
        s2.extend(s1);
        strokes.push(s2);
    }
    strokes
}

// FIXME error handling
fn get_closed_stroke(strokes: &Vec<Stroke>) -> Vec<Stroke> {
    let mut working = strokes.clone();
    while working.len() > 1 {
        working = connect_closed_points(working);
    }
    let first = working[0][0].clone();
    working[0].push(first);
    working
}

impl Model<Message> for CanvasModel {
    fn update(&mut self, message: &Message, widget_handler: &mut HandlerType) {
        match message {
            &Message::BrushSliderUpdate(size) => {
                self.current_brush.size = size as f64;
            },
            &Message::BrushToggleButton => {
                // TODO end stroke if valid and begen new one
            },
            &Message::StrokeCloseButton => { // FIXME fixed layer assignment
                let st = get_closed_stroke(&self.layers[1].strokes);
                self.layers[0].image.draw_stroke(&st, &self.current_brush)
            },
            &Message::ClearCanvasButton => {
                for l in &mut self.layers {
                    l.clear();
                }
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
        // TODO: efficiently!!!
        let mut image = ui::Image::new(self.width, self.height);
        let mut first = true;
        for l in &mut self.layers {
            if !l.visible { continue; }
            if first {
                image.data = l.image.data.to_vec();
                first = false;
            } else {
                l.composite(&mut image.data);
            }
        }
        area_draw_params.context.draw_image(0.0, 0.0, image.width, image.height, &mut image);
    }

    fn on_mouse_event(&mut self, area: &AreaHandler, area_mouse_event: &AreaMouseEvent) {
        let dragging = (area_mouse_event.held_1_to_64 != 0) | (area_mouse_event.down != 0);
        let point = StrokePoint {
            x: area_mouse_event.x,
            y: area_mouse_event.y,
            pressure: 0.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            timestamp: time::now().to_timespec().sec,
            dragging: dragging,
        };
        if self.layers[self.active_layer].mouse_event(&self.current_brush, point) {
            area.queue_redraw_all();
        }
    }
}

impl CanvasModel {
    pub fn new(w: f64, h: f64) -> CanvasModel {
        CanvasModel {
            layers: vec![Layer::new(w as u32, h as u32, 0), Layer::new(w as u32, h as u32, 0)],
            current_brush: Brush::new(),
            active_layer: 1,
            width: w,
            height: h,
        }
    }
}
