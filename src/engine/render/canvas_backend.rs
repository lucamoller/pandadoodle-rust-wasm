use crate::engine::render::draw_args::*;
use crate::engine::*;
use wasm_bindgen::prelude::*;

pub const TEXT_FONT_SIZE_ADJUSTMENT_RATIO: F1 = 0.7;

enum DrawFrom {
  Canvas(Rc<web_sys::HtmlCanvasElement>),
  Image(Rc<Texture>),
}

struct SourceOffset {
  position: F2,
  size: F2,
}

fn get_source_offset(
  args: &DrawImageArgs,
  src_existing_offset: Option<F2>,
  src_size: F2,
) -> Option<SourceOffset> {
  let partial_region_offset = &args.optional.partial_region_offset;
  let partial_region_size = &args.optional.partial_region_size;
  if *partial_region_offset != PARTIAL_REGION_OFFSET_DEFAULT
    || *partial_region_size != PARTIAL_REGION_SIZE_DEFAULT
  {
    let mut offset = SourceOffset {
      position: F2 {
        x: src_size.x * partial_region_offset.x,
        y: src_size.y * partial_region_offset.y,
      },
      size: F2 {
        x: src_size.x * partial_region_size.x,
        y: src_size.y * partial_region_size.y,
      },
    };
    if let Some(src_existing_offset) = src_existing_offset {
      offset.position += &src_existing_offset;
    }
    return Some(offset);
  }

  if let Some(src_existing_offset) = src_existing_offset {
    return Some(SourceOffset {
      position: src_existing_offset,
      size: src_size,
    });
  }
  return None;
}

pub struct Canvas2dDrawBackend {
  canvas_context: Rc<web_sys::CanvasRenderingContext2d>,
  last_opacity: Cell<F1>,
  last_composite_operation: RefCell<String>,
}

impl Canvas2dDrawBackend {
  pub fn new(canvas_context: Rc<web_sys::CanvasRenderingContext2d>) -> Canvas2dDrawBackend {
    canvas_context.set_global_alpha(1.0);
    return Canvas2dDrawBackend {
      canvas_context: canvas_context,
      last_opacity: Cell::new(1.0),
      last_composite_operation: RefCell::new(String::from("source-over")),
    };
  }

  pub fn clear_screen(&self, canvas_size: &F2) {
    self
      .canvas_context
      .reset_transform()
      .expect("reset_transform failed");
    self
      .canvas_context
      .clear_rect(0.0, 0.0, canvas_size.x.into(), canvas_size.y.into());
  }

  pub fn execute_image_draw(&self, args: &DrawImageArgs, canvas_size: &F2) {
    let mut center_position = args.position;
    let mut top_left_rel_position = F2 {
      x: -args.size.x * args.optional.anchor_point.x,
      y: -args.size.y * args.optional.anchor_point.y,
    };
    let mut size = args.size;

    // Do not draw images that are completely outside the screen.
    if (center_position.x + top_left_rel_position.x > canvas_size.x)
      || (center_position.y + top_left_rel_position.y > canvas_size.y)
      || (center_position.x + top_left_rel_position.x + args.size.x < 0.0)
      || (center_position.y + top_left_rel_position.y + args.size.y < 0.0)
    {
      return;
    }
    if args.optional.opacity == 0.0 {
      return;
    }

    let (mut draw_from, mut source_offset) = match &args.source {
      DrawSource::Canvas(canvas) => (
        DrawFrom::Canvas(canvas.clone()),
        get_source_offset(
          args,
          None,
          F2 {
            x: canvas.width() as F1,
            y: canvas.height() as F1,
          },
        ),
      ),
      DrawSource::Texture(texture) => {
        let texture_size = F2 {
          x: texture.width.get() as F1,
          y: texture.height.get() as F1,
        };

        if args.optional.color == DrawColor::default() {
          (
            DrawFrom::Image(texture.clone()),
            get_source_offset(args, None, texture_size),
          )
        } else {
          if texture.color_alpha_cache.is_some() {
            (
              DrawFrom::Canvas(texture.color_alpha_cache_canvas().clone()),
              get_source_offset(
                args,
                Some(
                  texture
                    .get_color_alpha_cache_coords(&args.optional.color, &args.optional.opacity),
                ),
                texture_size,
              ),
            )
          } else {
            (
              DrawFrom::Canvas(texture.get_colored_image(&args.optional.color)),
              get_source_offset(args, None, texture_size),
            )
          }
        }
      }
    };

    // Optimization for better performance in Firefox: avoid floating-point coordinates.
    if !args.optional.subpixel_precision {
      center_position.round();
      top_left_rel_position.round();
      size.round();
      if let Some(source_offset) = &mut source_offset {
        source_offset.position.round();
        source_offset.size.round();
      }
    }
    if size.x == 0.0 || size.y == 0.0 {
      return;
    }

    // Optimization for better performance in Firefox: cache scaled images.
    if !args.optional.subpixel_precision {
      let mut draw_from_replacement = None;
      if source_offset.is_none() {
        if let DrawFrom::Image(texture) = &draw_from {
          draw_from_replacement = texture.get_size_cache(size);
        }
      }
      if let Some(draw_from_replacement) = draw_from_replacement {
        draw_from = DrawFrom::Canvas(draw_from_replacement);
      }
    }

    self
      .canvas_context
      .reset_transform()
      .expect("reset_transform failed");

    if args.optional.opacity != self.last_opacity.get() {
      self.last_opacity.set(args.optional.opacity);
      self
        .canvas_context
        .set_global_alpha(self.last_opacity.get() as f64);
    }

    let composite_operation = match &args.optional.composite_operation {
      Some(specified_operation) => specified_operation.clone(),
      None => String::from("source-over"),
    };
    if composite_operation != *self.last_composite_operation.borrow() {
      self.last_composite_operation.replace(composite_operation);
      self
        .canvas_context
        .set_global_composite_operation(&self.last_composite_operation.borrow())
        .expect("set_global_composite_operation failed");
    }

    self
      .canvas_context
      .translate(center_position.x.into(), center_position.y.into())
      .expect("translate failed");

    if args.optional.rotation != 0.0 {
      self
        .canvas_context
        .rotate(args.optional.rotation.into())
        .expect("rotate failed");
    }

    match draw_from {
      DrawFrom::Image(texture) => match source_offset {
        Some(source_offset) => {
          self
            .canvas_context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
              texture.image(),
              source_offset.position.x.into(),
              source_offset.position.y.into(),
              source_offset.size.x.into(),
              source_offset.size.y.into(),
              top_left_rel_position.x.into(),
              top_left_rel_position.y.into(),
              size.x.into(),
              size.y.into(),
            )
            .expect(
              "draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh failed",
            );
        }
        None => {
          self
            .canvas_context
            .draw_image_with_html_image_element_and_dw_and_dh(
              texture.image(),
              top_left_rel_position.x.into(),
              top_left_rel_position.y.into(),
              size.x.into(),
              size.y.into(),
            )
            .expect("draw_image_with_html_image_element_and_dw_and_dh failed");
        }
      },
      DrawFrom::Canvas(canvas) => match source_offset {
        Some(source_offset) => {
          self
          .canvas_context
          .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            canvas.as_ref(),
            source_offset.position.x.into(),
            source_offset.position.y.into(),
            source_offset.size.x.into(),
            source_offset.size.y.into(),
            top_left_rel_position.x.into(),
            top_left_rel_position.y.into(),
            size.x.into(),
            size.y.into(),
          )
          .expect(
            "draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh failed",
          );
        }
        None => {
          self
            .canvas_context
            .draw_image_with_html_canvas_element_and_dw_and_dh(
              canvas.as_ref(),
              top_left_rel_position.x.into(),
              top_left_rel_position.y.into(),
              size.x.into(),
              size.y.into(),
            )
            .expect("draw_image_with_html_canvas_element_and_dw_and_dh failed");
        }
      },
    }
  }

  pub fn execute_string_draw(
    &self,
    args: &DrawStringArgs,
    device_pixel_ratio: &F1,
    canvas_size: &F2,
  ) {
    let mut center_position = args.position;
    if let None = args.optional.text_cache.as_ref() {
      center_position.y += args.font_size * 0.25 * TEXT_FONT_SIZE_ADJUSTMENT_RATIO;
    }
    self
      .canvas_context
      .reset_transform()
      .expect("reset_transform failed");

    if args.optional.opacity != self.last_opacity.get() {
      self.last_opacity.set(args.optional.opacity);
      self
        .canvas_context
        .set_global_alpha(self.last_opacity.get() as f64);
    }

    self
      .canvas_context
      .translate(center_position.x.into(), center_position.y.into())
      .expect("translate failed");

    if let Some(text_cache) = args.optional.text_cache.as_ref() {
      text_cache.check_update_cache(args);

      let cached_canvas_size = text_cache.cached_canvas.canvas_size.get();
      let size = text_cache.cached_canvas.canvas_size.get();

      let top_left_rel_position_x = match args.optional.alignment {
        TextAlignment::Left => 0.0,
        TextAlignment::Center => -cached_canvas_size.x * 0.5,
        TextAlignment::Right => -cached_canvas_size.x,
      };
      let top_left_rel_position_y = -cached_canvas_size.y * 0.5 - 4.0 * device_pixel_ratio;

      // Do not draw things that are completely outside the screen.
      if (center_position.x + top_left_rel_position_x > canvas_size.x)
        || (center_position.y + top_left_rel_position_y > canvas_size.y)
        || (center_position.x + top_left_rel_position_x + size.x < 0.0)
        || (center_position.y + top_left_rel_position_y + size.y < 0.0)
      {
        return;
      }

      self
        .canvas_context
        .draw_image_with_html_canvas_element_and_dw_and_dh(
          text_cache.cached_canvas.canvas.as_ref(),
          top_left_rel_position_x.into(),
          top_left_rel_position_y.into(),
          size.x.into(),
          size.y.into(),
        )
        .expect("draw_image_with_html_canvas_element_and_dw_and_dh failed");

      return;
    }

    self.canvas_context.set_font("200px Oregano-Regular");

    let scale = (args.font_size / 200.0 * TEXT_FONT_SIZE_ADJUSTMENT_RATIO) as f64;
    self
      .canvas_context
      .scale(scale, scale)
      .expect("scale failed");

    {
      // Do not draw things that are completely outside the screen.
      let text_metrics = self
        .canvas_context
        .measure_text(args.text.as_str())
        .expect("measure_text failed");
      let width = text_metrics.width() as F1;

      let top_left_rel_position_x = match args.optional.alignment {
        TextAlignment::Left => 0.0,
        TextAlignment::Center => -width * 0.5,
        TextAlignment::Right => -width,
      };
      if (center_position.x + top_left_rel_position_x > canvas_size.x)
        || (center_position.x + top_left_rel_position_x + width < 0.0)
      {
        return;
      }
    }

    self
      .canvas_context
      .set_text_align(match args.optional.alignment {
        TextAlignment::Left => "left",
        TextAlignment::Center => "center",
        TextAlignment::Right => "right",
      });

    if args.optional.border {
      self
        .canvas_context
        .set_fill_style(&JsValue::from_str(&format!(
          "rgb({}, {}, {}",
          args.optional.border_color.r, args.optional.border_color.g, args.optional.border_color.b
        )));

      let border_width = 12.0 * args.optional.border_scale as f64 / 2.75;
      let border_width_diagonal = 0.70 * border_width;
      let extra_bottom = 4.0;
      self
        .canvas_context
        .fill_text(args.text.as_str(), border_width, 0.0)
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(args.text.as_str(), -border_width, 0.0)
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(args.text.as_str(), 0.0, extra_bottom + border_width)
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(args.text.as_str(), 0.0, -border_width)
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(
          args.text.as_str(),
          border_width_diagonal,
          extra_bottom + border_width_diagonal,
        )
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(
          args.text.as_str(),
          -border_width_diagonal,
          extra_bottom + border_width_diagonal,
        )
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(
          args.text.as_str(),
          border_width_diagonal,
          -border_width_diagonal,
        )
        .expect("fill_text failed");
      self
        .canvas_context
        .fill_text(
          args.text.as_str(),
          -border_width_diagonal,
          -border_width_diagonal,
        )
        .expect("fill_text failed");
    }

    self
      .canvas_context
      .set_fill_style(&JsValue::from_str(&format!(
        "rgb({}, {}, {}",
        args.optional.color.r, args.optional.color.g, args.optional.color.b
      )));
    self
      .canvas_context
      .fill_text(args.text.as_str(), 0.0, 0.0)
      .expect("fill_text failed");
  }

  pub fn execute_gradient_box_draw(&mut self, args: &DrawGradientBoxArgs, canvas_size: &F2) {
    let center_position = args.position;
    let top_left_rel_position = F2 {
      x: -args.size.x * args.anchor_point.x,
      y: -args.size.y * args.anchor_point.y,
    };
    if (center_position.x + top_left_rel_position.x > canvas_size.x)
      || (center_position.y + top_left_rel_position.y > canvas_size.y)
      || (center_position.x + top_left_rel_position.x + args.size.x < 0.0)
      || (center_position.y + top_left_rel_position.y + args.size.y < 0.0)
    {
      return;
    }
    self
      .canvas_context
      .reset_transform()
      .expect("reset_transform failed");

    if 1.0 != self.last_opacity.get() {
      self.last_opacity.set(1.0);
      self
        .canvas_context
        .set_global_alpha(self.last_opacity.get() as f64);
    }

    let composite_operation = String::from("source-over");
    if composite_operation != *self.last_composite_operation.borrow() {
      self.last_composite_operation.replace(composite_operation);
      self
        .canvas_context
        .set_global_composite_operation(&self.last_composite_operation.borrow())
        .expect("set_global_composite_operation failed");
    }
    self
      .canvas_context
      .translate(center_position.x.into(), center_position.y.into())
      .expect("translate failed");

    let gradient = self.canvas_context.create_linear_gradient(
      top_left_rel_position.x.into(),
      0.0,
      (top_left_rel_position.x + args.size.x).into(),
      0.0,
    );
    gradient
      .add_color_stop(
        0.0,
        &format!(
          "rgba({}, {}, {}, {}",
          args.draw_color_start.r,
          args.draw_color_start.g,
          args.draw_color_start.b,
          args.alpha_start
        ),
      )
      .expect("add_color_stop failed");
    gradient
      .add_color_stop(
        1.0,
        &format!(
          "rgba({}, {}, {}, {}",
          args.draw_color_end.r, args.draw_color_end.g, args.draw_color_end.b, args.alpha_end
        ),
      )
      .expect("add_color_stop failed");
    self.canvas_context.set_fill_style(&gradient);
    self.canvas_context.fill_rect(
      top_left_rel_position.x.into(),
      top_left_rel_position.y.into(),
      args.size.x.into(),
      args.size.y.into(),
    );
  }
}
