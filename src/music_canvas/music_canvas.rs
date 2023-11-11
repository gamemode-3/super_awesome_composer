use crate::data::State;
use druid::kurbo::BezPath;
use druid::{Env, Event, EventCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget};

pub struct MusicCanvas;

impl MusicCanvas {
    pub fn new() -> Self {
        MusicCanvas
    }
}

impl Widget<State> for MusicCanvas {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut State, _env: &Env) {
        // TODO: Handle mouse and keyboard input events here
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {
        // TODO: Handle updates to the canvas here
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, _env: &Env) {
        let full_canvas = ctx.size().to_rect();
        ctx.fill(full_canvas, &data.canvas_background);
        let mut path = BezPath::new();
        path.move_to((50.0, 50.0));
        path.quad_to((150.0, 150.0), (250.0, 50.0)); // Adjust the control points to bend the line

        ctx.stroke(path, &data.canvas_primary_elements, 3.0)
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &State,
        _env: &Env,
    ) {
        // TODO: Handle lifecycle events here
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &State,
        _env: &Env,
    ) -> Size {
        bc.max()
    }
}
