use crate::context::Context;
use crate::*;

pub struct BackgroundWood {}

impl BackgroundWood {
  pub fn draw(context: &mut Context) {
    let size_extra_background = context.ui_viewport.viewport_size_on_screen.x * 1.0;

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.background_wood.clone()),
      position: context.screen_size * 0.5,
      size: F2 {
        x: if context.ui_viewport.viewport_size_on_screen.x + size_extra_background
          < context.screen_size.x
        {
          context.ui_viewport.viewport_size_on_screen.x + size_extra_background
        } else {
          context.screen_size.x
        },
        y: context.screen_size.y,
      },
      depth: context.draw_depths.background + 0.1,
      optional: DrawImageOptionalArgs::default(),
    });
  }
}
