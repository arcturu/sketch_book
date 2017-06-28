extern crate time;
extern crate ui;
use ui::{Area, AreaMouseEvent, AreaHandler, AreaDrawParams, Image};

use stroke::{Stroke, StrokePoint};

pub struct CanvasImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
    color_depth: u32, // in byte
    strokes: Vec<Stroke>,
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
        }
    }
    pub fn draw_stroke(&mut self) {
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
    pub fn mouse_event(&mut self, e: &AreaMouseEvent) {
        if e.held_1_to_64 == 1 {
            let point = self.get_stroke_point(e);
            let mut new_stroke = match self.strokes.pop() {
                Some(s) => if s.finished { s } else { Stroke::new(10) },
                None => Stroke::new(10),
            };
            new_stroke.points.push(point);
            self.strokes.push(new_stroke);
        } else if self.strokes.len() > 0 {
            self.strokes.last_mut().unwrap().finished = true;
        }
        self.draw_stroke();
    }
}

pub struct CanvasArea {
    canvas_image: CanvasImage,
    width: f64,
    height: f64,
}

impl CanvasArea {
    pub fn new(w: f64, h: f64) -> CanvasArea {
        CanvasArea {
            canvas_image: CanvasImage::new(300, 200), // TODO
            width: w,
            height: h,
        }
    }
}

impl AreaHandler for CanvasArea {
    fn mouse_event(&mut self, area: &Area, area_mouse_event: &AreaMouseEvent) {
//        println!("{:?}", area_mouse_event);
        self.canvas_image.mouse_event(area_mouse_event);
        area.queue_redraw_all();
    }
    fn draw(&mut self, _area: &Area, area_draw_params: &AreaDrawParams) {
        let mut image = Image::new(self.canvas_image.width as f64, self.canvas_image.height as f64);
        image.data = self.canvas_image.data.to_vec(); // deep copy
        area_draw_params.context.draw_image(0.0, 0.0, image.width, image.height, &mut image);
    }
}

