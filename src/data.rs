use druid::{Data, Color};

#[derive(Clone, Data)]
pub struct State {
    pub canvas_background: Color,
    pub canvas_primary_elements: Color,
}

impl State {
    pub fn new() -> Self {
        State {
            canvas_background: Color::Rgba32(0x202020ff),
            canvas_primary_elements: Color::WHITE,
        }
    }
}