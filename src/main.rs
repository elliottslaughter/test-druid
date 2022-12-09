use std::time::{Duration, Instant};

use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, Slider};
use druid::{
    AppLauncher, Color, Data, Lens, MouseButton, PlatformError, Point, Rect, TimerToken, WidgetExt,
    WindowDesc,
};
use std::sync::Arc;

#[derive(Clone, Data)]
struct AppData {
    rects: Arc<Vec<Rect>>,
}

impl AppData {
    fn new() -> Self {
        Self {
            rects: Arc::new(Vec::new()),
        }
    }
}

struct CanvasWidget {
    timer_id: TimerToken,
    last_update: Instant,
}

impl Widget<AppData> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.max() // fill all available space
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {}
}

fn make_widget() -> impl Widget<AppData> {
    CanvasWidget {
        timer_id: TimerToken::INVALID,
        last_update: Instant::now(),
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(make_widget());
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppData::new())
}
