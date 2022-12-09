use std::sync::Arc;
use std::time::{Duration, Instant};

use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, Slider};
use druid::{
    AppLauncher, Color, Data, Lens, MouseButton, PlatformError, Point, Rect, TimerToken, WidgetExt,
    WindowDesc,
};

use rand::Rng;

#[derive(Copy, Clone, Data, PartialEq)]
struct ColorRect {
    // These are UNSCALED rectangles in the range [0,1)
    r: druid::Rect,
    vx: f64,
    vy: f64,
    c: Color,
}

#[derive(Clone, Data)]
struct AppData {
    rects: Arc<Vec<ColorRect>>,
}

impl AppData {
    fn new(rects: Vec<ColorRect>) -> Self {
        Self {
            rects: Arc::new(rects),
        }
    }

    fn step(&mut self) {
        for r in Arc::make_mut(&mut self.rects) {
            r.r =
                r.r.with_origin(r.r.origin() + (r.vx / 1000.0, r.vy / 1000.0));
        }
    }
}

struct CanvasWidget {
    timer_id: TimerToken,
    last_update: Instant,
}

const INTERVAL: u64 = 1000 / 60; // we're trying to run this at a constant 60 FPS

impl Widget<AppData> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let deadline = Duration::from_millis(INTERVAL);
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    data.step();
                    ctx.request_paint();
                    let deadline = Duration::from_millis(INTERVAL);
                    let now = Instant::now();
                    println!("FPS: {}", 1.0/now.duration_since(self.last_update).as_secs_f64());
                    self.last_update = now;
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _env: &Env) {
        if data.rects != old_data.rects {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.max() // fill all available space
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        let size: Size = ctx.size();
        for r in data.rects.iter() {
            ctx.fill(
                &Rect::new(
                    r.r.x0 * size.width,
                    r.r.y0 * size.height,
                    r.r.x1 * size.width,
                    r.r.y1 * size.height,
                ),
                &r.c,
            );
        }
    }
}

fn make_widget() -> impl Widget<AppData> {
    CanvasWidget {
        timer_id: TimerToken::INVALID,
        last_update: Instant::now(),
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_widget());
    let mut rects = Vec::new();
    const N: u32 = 6000;
    let mut rng = rand::thread_rng();
    for _ in 0..N {
        let sx: f64 = rng.gen();
        let sy: f64 = rng.gen();
        rects.push(ColorRect {
            r: Rect::from_origin_size(
                Point::new(rng.gen(), rng.gen()),
                Size::new(sx/4.0, sy/4.0),
            ),
            vx: rng.gen_range(-1.0..1.0),
            vy: rng.gen_range(-1.0..1.0),
            c: Color::rgb8(rng.gen(), rng.gen(), rng.gen()),
        });
    }
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::new(rects))
}
