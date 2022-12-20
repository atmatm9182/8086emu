use crate::AppState;
use druid::{
    widget::{Controller},
    Widget,
};

use crate::delegate::SHOULD;

pub struct InputController;

impl<W: Widget<AppState>> Controller<AppState, W> for InputController {
    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &druid::Env) {
        if old_data.input != data.input {
            ctx.submit_command(SHOULD);
        }
        child.update(ctx, old_data, data, env)
    }
}
