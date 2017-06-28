pub struct CanvasImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
    depth: u32, // in byte
}

impl CanvasImage {
    pub fn new(w: u32, h: u32) -> CanvasImage {
        let depth = 1; // 1 byte = 0..255 per color
        CanvasImage {
            data: vec![0; (w * h * depth * 4) as usize],
            width: w,
            height: h,
            depth: depth,
        }
    }
    pub fn process_stroke(s: &Stroke) {

    }
}
