use crate::engine::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
  type ExtendedTextMetrics;

  #[wasm_bindgen(method, getter, js_name = actualBoundingBoxAscent)]
  fn actual_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter, js_name = actualBoundingBoxDescent)]
  fn actual_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter, js_name = fontBoundingBoxAscent)]
  fn font_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter, js_name = fontBoundingBoxDescent)]
  fn font_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter, js_name = actualBoundingBoxLeft)]
  fn actual_bounding_box_left(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter, js_name = actualBoundingBoxRight)]
  fn actual_bounding_box_right(this: &ExtendedTextMetrics) -> f64;

  #[wasm_bindgen(method, getter)]
  fn width(this: &ExtendedTextMetrics) -> f64;
}

pub struct TextCache {
  pub cached_canvas: CachedCanvasBackend,
  pub bounding_box_ascent: Cell<F1>,

  text: RefCell<String>,
  font_size: Cell<F1>,
  alignment: Cell<TextAlignment>,
  color: Cell<DrawColor>,
  border: Cell<bool>,
  border_color: Cell<DrawColor>,
  border_scale: Cell<F1>,
}

impl TextCache {
  pub fn new() -> Rc<TextCache> {
    return Rc::new(TextCache {
      cached_canvas: CachedCanvasBackend::new(&F2 { x: 0.0, y: 0.0 }),
      bounding_box_ascent: Cell::new(0.0),

      text: RefCell::default(),
      font_size: Cell::default(),
      alignment: Cell::new(TextAlignment::Center),
      color: Cell::default(),
      border: Cell::default(),
      border_color: Cell::default(),
      border_scale: Cell::default(),
    });
  }

  pub fn check_update_cache(&self, args: &DrawStringArgs) {
    if *self.text.borrow() != args.text
      || self.font_size.get() != args.font_size
      || self.alignment.get() != args.optional.alignment
      || self.color.get() != args.optional.color
      || self.border.get() != args.optional.border
      || self.border_color.get() != args.optional.border_color
      || self.border_scale.get() != args.optional.border_scale
    {
      self.text.replace(args.text.clone());
      self.font_size.set(args.font_size);
      self.alignment.set(args.optional.alignment);
      self.color.set(args.optional.color);
      self.border.set(args.optional.border);
      self.border_color.set(args.optional.border_color);
      self.border_scale.set(args.optional.border_scale);

      let canvas_context = self.cached_canvas.canvas_context.as_ref();
      canvas_context
        .reset_transform()
        .expect("reset_transform failed");

      canvas_context.set_font("200px Oregano-Regular");

      let scale =
        (args.font_size / 200.0 * super::canvas_backend::TEXT_FONT_SIZE_ADJUSTMENT_RATIO) as f64;

      canvas_context.scale(scale, scale).expect("scale failed");

      canvas_context.set_text_align("left");

      let text_metrics = canvas_context
        .measure_text(args.text.as_str())
        .expect("measure_text failed");
      let text_metrics: ExtendedTextMetrics = text_metrics.unchecked_into();

      let font_bounding_box_ascent = if !text_metrics.font_bounding_box_ascent().is_nan() {
        text_metrics.font_bounding_box_ascent()
      } else {
        193.0
      };
      let font_bounding_box_descent = if !text_metrics.font_bounding_box_descent().is_nan() {
        text_metrics.font_bounding_box_descent()
      } else {
        65.0
      };

      let (extra_border, border_width) = if args.optional.border {
        let border_width = 12.0 * args.optional.border_scale as F1 / 2.75;
        let extra_bottom = 4.0;
        self
          .bounding_box_ascent
          .set(scale as F1 * (border_width + font_bounding_box_ascent as F1));
        (
          F2 {
            x: 2.0 * border_width + 4.0,
            y: 2.0 * border_width + extra_bottom,
          },
          border_width,
        )
      } else {
        self
          .bounding_box_ascent
          .set(scale as F1 * font_bounding_box_ascent as F1);
        (F2 { x: 4.0, y: 0.0 }, 0.0)
      };

      let size = scale as F1
        * F2 {
          x: (text_metrics.actual_bounding_box_left() + text_metrics.actual_bounding_box_right())
            as F1,
          y: (font_bounding_box_descent + font_bounding_box_ascent) as F1,
        }
        + extra_border;
      let position = F2 {
        x: 0.0,
        y: (font_bounding_box_ascent) as F1 + border_width,
      };

      self.cached_canvas.check_canvas_size_changed(&size);
      self.cached_canvas.clear_cache_required.set(true);
      self.cached_canvas.check_clear_cache();

      // canvas_context.set_fill_style(
      //   &DrawColor {
      //     r: 0,
      //     g: 200,
      //     b: 200,
      //   }
      //   .as_rgb_js_value(),
      // );
      // canvas_context.fill_rect(0.0, 0.0, size.x.into(), size.y.into());

      canvas_context
        .reset_transform()
        .expect("reset_transform failed");

      canvas_context.set_font("200px Oregano-Regular");

      let scale =
        (args.font_size / 200.0 * super::canvas_backend::TEXT_FONT_SIZE_ADJUSTMENT_RATIO) as f64;

      canvas_context.scale(scale, scale).expect("scale failed");

      canvas_context.set_text_align("center");

      canvas_context
        .translate((size.x * 0.5 / scale as F1).into(), position.y.into())
        .expect("translate failed");

      if args.optional.border {
        canvas_context.set_fill_style(&JsValue::from_str(&format!(
          "rgb({}, {}, {}",
          args.optional.border_color.r, args.optional.border_color.g, args.optional.border_color.b
        )));

        let border_width = 12.0 * args.optional.border_scale as f64 / 2.75;
        let border_width_diagonal = 0.70 * border_width;
        let extra_bottom = 4.0;
        canvas_context
          .fill_text(args.text.as_str(), border_width, 0.0)
          .expect("fill_text failed");
        canvas_context
          .fill_text(args.text.as_str(), -border_width, 0.0)
          .expect("fill_text failed");
        canvas_context
          .fill_text(args.text.as_str(), 0.0, extra_bottom + border_width)
          .expect("fill_text failed");
        canvas_context
          .fill_text(args.text.as_str(), 0.0, -border_width)
          .expect("fill_text failed");
        canvas_context
          .fill_text(
            args.text.as_str(),
            border_width_diagonal,
            extra_bottom + border_width_diagonal,
          )
          .expect("fill_text failed");
        canvas_context
          .fill_text(
            args.text.as_str(),
            -border_width_diagonal,
            extra_bottom + border_width_diagonal,
          )
          .expect("fill_text failed");
        canvas_context
          .fill_text(
            args.text.as_str(),
            border_width_diagonal,
            -border_width_diagonal,
          )
          .expect("fill_text failed");
        canvas_context
          .fill_text(
            args.text.as_str(),
            -border_width_diagonal,
            -border_width_diagonal,
          )
          .expect("fill_text failed");
      }

      canvas_context.set_fill_style(&JsValue::from_str(&format!(
        "rgb({}, {}, {}",
        args.optional.color.r, args.optional.color.g, args.optional.color.b
      )));

      canvas_context
        .fill_text(args.text.as_str(), 0.0, 0.0)
        .expect("fill_text failed");
    }
  }
}
