use std::sync::Arc;

use druid::{Color, Data, Point};

use crate::backend::music::Project;

#[derive(Clone, Data)]
pub struct State {
    pub mouse_pos: Point,
    pub colors: Colors,
    pub projects: Arc<Vec<Project>>,
}

impl State {
    pub fn new() -> Self {
        State {
            mouse_pos: Point::ZERO,
            colors: Colors::new(),
            projects: Arc::new(Vec::new()),
        }
    }
}

#[derive(Clone, Data)]
pub struct Colors {
    pub canvas_background: Color,
    pub canvas_primary_elements: Color,
}

impl Colors {
    pub fn new() -> Self {
        Colors {
            canvas_background: Color::Rgba32(0x202020ff),
            canvas_primary_elements: Color::WHITE,
        }
    }
}
