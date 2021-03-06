use reactive::widget::{AreaDrawParams};
#[derive(Clone, Debug)]
pub enum Message {
    BrushToggleButton,
    StrokeCloseButton,
    ClearCanvasButton,
    OutputButton,
    BrushSliderUpdate(i64),
    CanvasMouseEvent{x: f64, y: f64, down: bool, up: bool, dragging: bool},
}

