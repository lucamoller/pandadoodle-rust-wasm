use crate::context::Context;
use crate::*;

pub struct BackgroundCanvas {}

impl BackgroundCanvas {
  pub fn draw(context: &mut Context, book: Book) {
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(context.texture_manager.background_canvas.clone()),
        position: F2 {
          x: 0.5,
          y: context.game_viewport.viewport_yx_ratio / 2.0,
        },
        size: context
          .texture_manager
          .background_canvas
          .get_size_from_width(1.0),
        depth: context.draw_depths.background,
        optional: DrawImageOptionalArgs::default(),
      },
    );

    let (texture_bot, texture_top) = match book {
      Book::Panda => (None, None),
      Book::Cat => (
        Some(context.texture_manager.background_cat_bot.clone()),
        Some(context.texture_manager.background_cat_top.clone()),
      ),
      Book::Wolf => (
        Some(context.texture_manager.background_wolf_bot.clone()),
        Some(context.texture_manager.background_wolf_top.clone()),
      ),
      Book::Rabbit => (
        Some(context.texture_manager.background_rabbit_bot.clone()),
        Some(context.texture_manager.background_rabbit_top.clone()),
      ),
      Book::Panda2 => (None, None),
    };

    let canvas_size = context
      .texture_manager
      .background_canvas
      .get_size_from_width(1.0);

    if let Some(texture_bot) = texture_bot {
      let size = texture_bot.get_size_from_width(1.0);
      context.draw_manager.draw_viewport(
        &context.game_viewport,
        DrawImageArgs {
          source: DrawSource::Texture(texture_bot),
          position: F2 {
            x: 0.5,
            y: (context.game_viewport.viewport_yx_ratio + canvas_size.y) * 0.5 + 1.0 / 320.0,
          },
          size: size,
          depth: context.draw_depths.background - 0.1,
          optional: DrawImageOptionalArgs {
            anchor_point: F2 { x: 0.5, y: 1.0 },
            ..Default::default()
          },
        },
      );
    }

    if let Some(texture_top) = texture_top {
      let size = texture_top.get_size_from_width(1.0);
      context.draw_manager.draw_viewport(
        &context.game_viewport,
        DrawImageArgs {
          source: DrawSource::Texture(texture_top),
          position: F2 {
            x: 0.5,
            y: (context.game_viewport.viewport_yx_ratio - canvas_size.y) * 0.5 - 1.0 / 320.0,
          },
          size: size,
          depth: context.draw_depths.background - 0.1,
          optional: DrawImageOptionalArgs {
            anchor_point: F2 { x: 0.5, y: 0.0 },
            ..Default::default()
          },
        },
      );
    }
  }
}
