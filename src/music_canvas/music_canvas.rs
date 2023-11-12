use std::f64::consts::PI;

use crate::data::State;
use druid::kurbo::BezPath;
use druid::{Env, Event, EventCtx, PaintCtx, Rect, RenderContext, Size, UpdateCtx, Widget, Point};
use druid_shell::kurbo::{Circle, CircleSegment, Line};

struct Spacing;

pub struct MusicCanvas {
    project_selected: usize,
    mouse_pos: Point,
}

impl MusicCanvas {
    pub fn new() -> Self {
        MusicCanvas {
            project_selected: 0,
            mouse_pos: Point::ZERO,
        }
    }

    fn paint_note_segment(
        &mut self,
        ctx: &mut PaintCtx,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        width: f64,
        color: druid::Color,
    ) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let cx = (x0 + x1) / 2.0;
        let cy = (y0 + y1) / 2.0;

        let mut offset = 0.0;
        if dx.abs() < dy.abs() {
            offset = (dy.signum() * (dx.abs() - dy.abs())) / 2.0;
            dy = dx.abs() * dy.signum();
        }

        let num = dx * dx + dy * dy + 2.0 * dy * width;
        let denom = 4.0 * dy;

        let (r1, r2) = if denom != 0.0 {
            let r2 = num / denom;
            let r1 = r2 - 40.0;

            let sweep_angle = (dx / (r1 + r2)).asin();

            let circle = CircleSegment::new(
                (cx - dx / 2.0, cy - dy / 2.0 + r1 + offset + width),
                r2,
                r1,
                3.0 / 2.0 * PI,
                sweep_angle,
            );
            ctx.fill(circle, &color);
            let circle = CircleSegment::new(
                (cx + dx / 2.0, cy + dy / 2.0 - r1 - offset),
                r2,
                r1,
                1.0 / 2.0 * PI,
                sweep_angle,
            );
            ctx.fill(circle, &color);

            (r1, r2)
        } else if dx == 0.0 {
            let circle = CircleSegment::new(
                (cx - dx / 2.0, cy - dy / 2.0 + offset + width / 2.0),
                width / 2.0,
                0.0,
                -dy.signum() * 1.0 / 2.0 * PI,
                dy.signum() * PI / 2.0,
            );
            ctx.fill(circle, &color);
            let circle = CircleSegment::new(
                (cx + dx / 2.0, cy + dy / 2.0 - offset + width / 2.0),
                width / 2.0,
                0.0,
                -dy.signum() * 3.0 / 2.0 * PI,
                dy.signum() * PI / 2.0,
            );
            ctx.fill(circle, &color);

            let r = width / 2.0;
            (-r, r)
        } else {
            let rect = Rect::new(cx - dx / 2.0, cy, cx + dx / 2.0, cy + width);
            ctx.fill(rect, &color);
            return;
        };

        if offset != 0.0 {
            let rect = Rect::new(
                cx - width / 2.0,
                cy + dy / 2.0 - offset - r1,
                cx + width / 2.0,
                cy - dy / 2.0 + offset + r2,
            );
            ctx.fill(rect, &color);
        } else {
            ctx.fill(Circle::new((cx, cy + width / 2.0), width / 2.0), &color);
        }
    }

    fn calculate_spacing(&self, data: &State) -> Spacing {
        data.projects[self.project_selected];

        Spacing
    }
}

impl Widget<State> for MusicCanvas {
    fn event(&mut self, _ctx: &mut EventCtx, event: &Event, data: &mut State, _env: &Env) {
        match event {
            Event::MouseMove(ev) => {
                data.mouse_pos = ev.pos;
            }
            _ => (),
        };
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &State, _data: &State, _env: &Env) {
        // TODO: Handle updates to the canvas here
        ctx.request_paint();
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &State, _env: &Env) {
        let width = ctx.size().width;
        let height = ctx.size().height;

        let mouse_x = data.mouse_pos.x;
        let mouse_y = data.mouse_pos.y;

        
        

        let line = Line::new((0.0, height / 2.0), (mouse_x, mouse_y));
        ctx.stroke(line, &data.colors.canvas_primary_elements, 1.0);
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
