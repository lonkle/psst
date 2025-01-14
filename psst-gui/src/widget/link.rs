use crate::{ui::theme, widget::ExClick};
use druid::{
    widget::{prelude::*, ControllerHost},
    Color, Data, KeyOrValue, MouseEvent, Point, WidgetPod,
};

pub struct Link<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    border_color: KeyOrValue<Color>,
    border_width: KeyOrValue<f64>,
    corner_radius: KeyOrValue<f64>,
}

impl<T: Data> Link<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        Self {
            inner: WidgetPod::new(inner).boxed(),
            border_color: theme::LINK_HOT_COLOR.into(),
            border_width: 0.0.into(),
            corner_radius: 0.0.into(),
        }
    }

    pub fn border(
        mut self,
        color: impl Into<KeyOrValue<Color>>,
        width: impl Into<KeyOrValue<f64>>,
    ) -> Self {
        self.set_border(color, width);
        self
    }

    pub fn set_border(
        &mut self,
        color: impl Into<KeyOrValue<Color>>,
        width: impl Into<KeyOrValue<f64>>,
    ) {
        self.border_color = color.into();
        self.border_width = width.into();
    }

    pub fn circle(mut self) -> Self {
        self.set_rounded(64.0);
        self
    }

    pub fn rounded(mut self, radius: impl Into<KeyOrValue<f64>>) -> Self {
        self.set_rounded(radius);
        self
    }

    pub fn set_rounded(&mut self, radius: impl Into<KeyOrValue<f64>>) {
        self.corner_radius = radius.into();
    }
}

impl<T: Data> Widget<T> for Link<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let background = if ctx.is_hot() {
            env.get(theme::LINK_HOT_COLOR)
        } else {
            env.get(theme::LINK_COLD_COLOR)
        };
        let border_color = self.border_color.resolve(env);
        let border_width = self.border_width.resolve(env);
        let visible_background = background.as_rgba_u32() & 0x00000FF > 0;
        let visible_border = border_color.as_rgba_u32() & 0x000000FF > 0 && border_width > 0.0;
        if visible_background || visible_border {
            let corner_radius = self.corner_radius.resolve(env);
            let rounded_rect = ctx
                .size()
                .to_rect()
                .inset(-border_width / 2.0)
                .to_rounded_rect(corner_radius);
            if visible_border {
                ctx.stroke(rounded_rect, &border_color, border_width);
            }
            if visible_background {
                ctx.fill(rounded_rect, &background);
            }
        }
        self.inner.paint(ctx, data, env);
    }
}

pub trait LinkExt<T: Data>: Widget<T> + Sized + 'static {
    fn link(self) -> Link<T> {
        Link::new(self)
    }

    fn on_ex_click(
        self,
        f: impl Fn(&mut EventCtx, &MouseEvent, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, ExClick<T>> {
        ControllerHost::new(self, ExClick::new(f))
    }
}

impl<T: Data, W: Widget<T> + 'static> LinkExt<T> for W {}
