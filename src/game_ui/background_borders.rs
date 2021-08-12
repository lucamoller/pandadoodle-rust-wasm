use crate::context::Context;
use crate::*;

pub struct BackgroundBorders {}

impl BackgroundBorders {
  pub fn draw(context: &mut Context) {
    let size_gradient_border = context.ui_viewport.viewport_size_on_screen.x * 0.5;
    let size_black_border = (context.screen_size.x - context.ui_viewport.viewport_size_on_screen.x)
      / 2.0
      - size_gradient_border
      + 1.0;

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.pixel.clone()),
      position: F2 { x: 0.0, y: 0.0 },
      size: F2 {
        x: size_black_border,
        y: context.screen_size.y,
      },
      depth: -100.0,
      optional: DrawImageOptionalArgs {
        color: DrawColor { r: 0, g: 0, b: 0 },
        opacity: 1.0,
        anchor_point: F2 { x: 0.0, y: 0.0 },
        ..Default::default()
      },
    });

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.pixel.clone()),
      position: F2 {
        x: context.screen_size.x - size_black_border,
        y: 0.0,
      },
      size: F2 {
        x: size_black_border,
        y: context.screen_size.y,
      },
      depth: -100.0,
      optional: DrawImageOptionalArgs {
        color: DrawColor { r: 0, g: 0, b: 0 },
        opacity: 1.0,
        anchor_point: F2 { x: 0.0, y: 0.0 },
        ..Default::default()
      },
    });

    context
      .draw_manager
      .draw_gradient_box_screen(DrawGradientBoxArgs {
        position: F2 {
          x: size_black_border - 1.0,
          y: 0.0,
        },
        size: F2 {
          x: size_gradient_border,
          y: context.screen_size.y,
        },
        depth: -120.0,
        draw_color_start: DrawColor::new(&0, &0, &0),
        alpha_start: 1.0,
        draw_color_end: DrawColor::new(&0, &0, &0),
        alpha_end: 0.0,
        anchor_point: F2 { x: 0.0, y: 0.0 },
      });

    context
      .draw_manager
      .draw_gradient_box_screen(DrawGradientBoxArgs {
        position: F2 {
          x: context.screen_size.x - (size_black_border - 1.0) - size_gradient_border,
          y: 0.0,
        },
        size: F2 {
          x: size_gradient_border,
          y: context.screen_size.y,
        },
        depth: -120.0,
        draw_color_start: DrawColor::new(&0, &0, &0),
        alpha_start: 0.0,
        draw_color_end: DrawColor::new(&0, &0, &0),
        alpha_end: 1.0,
        anchor_point: F2 { x: 0.0, y: 0.0 },
      });

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.pixel.clone()),
      position: F2 { x: 0.0, y: 0.0 },
      size: F2 {
        x: context.ui_viewport.viewport_position_on_screen.x,
        y: context.screen_size.y,
      },
      depth: context.draw_depths.background + 0.1,
      optional: DrawImageOptionalArgs {
        color: DrawColor { r: 0, g: 0, b: 0 },
        opacity: 0.05,
        anchor_point: F2 { x: 0.0, y: 0.0 },
        ..Default::default()
      },
    });

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.pixel.clone()),
      position: F2 {
        x: context.ui_viewport.viewport_position_on_screen.x
          + context.ui_viewport.viewport_size_on_screen.x,
        y: 0.0,
      },
      size: F2 {
        x: context.ui_viewport.viewport_position_on_screen.x,
        y: context.screen_size.y,
      },
      depth: context.draw_depths.background + 0.1,
      optional: DrawImageOptionalArgs {
        color: DrawColor { r: 0, g: 0, b: 0 },
        opacity: 0.05,
        anchor_point: F2 { x: 0.0, y: 0.0 },
        ..Default::default()
      },
    });
  }
}
