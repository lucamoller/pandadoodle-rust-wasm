use crate::engine::*;
use wasm_bindgen::JsValue;

pub enum DrawArgs {
  Image(DrawImageArgs),
  String(DrawStringArgs),
  GradientBox(DrawGradientBoxArgs),
}

impl DrawArgs {
  pub fn get_depth(&self) -> &F1 {
    return match self {
      DrawArgs::Image(image_args) => &image_args.depth,
      DrawArgs::String(string_args) => &string_args.depth,
      DrawArgs::GradientBox(gradient_box_args) => &gradient_box_args.depth,
    };
  }
}

pub enum DrawSource {
  Canvas(Rc<web_sys::HtmlCanvasElement>),
  Texture(Rc<Texture>),
}

pub struct DrawImageArgs {
  pub source: DrawSource,
  pub position: F2,
  pub size: F2,
  pub depth: F1,
  pub optional: DrawImageOptionalArgs,
}

pub struct DrawImageOptionalArgs {
  pub color: DrawColor,
  pub rotation: F1,
  pub anchor_point: F2,
  pub opacity: F1,
  pub partial_region_offset: F2,
  pub partial_region_size: F2,
  pub composite_operation: Option<String>,
  pub subpixel_precision: bool,
}

pub const PARTIAL_REGION_OFFSET_DEFAULT: F2 = F2 { x: 0.0, y: 0.0 };
pub const PARTIAL_REGION_SIZE_DEFAULT: F2 = F2 { x: 1.0, y: 1.0 };

impl Default for DrawImageOptionalArgs {
  fn default() -> DrawImageOptionalArgs {
    return DrawImageOptionalArgs {
      color: DrawColor::default(),
      rotation: 0.0,
      anchor_point: F2 { x: 0.5, y: 0.5 },
      opacity: 1.0,
      partial_region_offset: PARTIAL_REGION_OFFSET_DEFAULT,
      partial_region_size: PARTIAL_REGION_SIZE_DEFAULT,
      composite_operation: None,
      subpixel_precision: false,
    };
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DrawColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl DrawColor {
  pub fn new(r: &u8, g: &u8, b: &u8) -> DrawColor {
    return DrawColor {
      r: *r,
      g: *g,
      b: *b,
    };
  }

  pub fn as_rgb_js_value(&self) -> JsValue {
    return JsValue::from_str(&format!("rgb({}, {}, {})", self.r, self.g, self.b));
  }
}

impl Default for DrawColor {
  fn default() -> DrawColor {
    return DrawColor {
      r: 255,
      g: 255,
      b: 255,
    };
  }
}

pub struct DrawStringArgs {
  pub text: String,
  pub position: F2,
  pub font_size: F1,
  pub depth: F1,
  pub optional: DrawStringOptionalArgs,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TextAlignment {
  Left,
  Center,
  Right,
}

pub struct DrawStringOptionalArgs {
  pub alignment: TextAlignment,
  pub color: DrawColor,
  pub border: bool,
  pub border_color: DrawColor,
  pub border_scale: F1,
  pub opacity: F1,
  pub text_cache: Option<Rc<TextCache>>,
}

impl Default for DrawStringOptionalArgs {
  fn default() -> DrawStringOptionalArgs {
    return DrawStringOptionalArgs {
      alignment: TextAlignment::Left,
      color: DrawColor::default(),
      border: false,
      border_color: DrawColor { r: 0, g: 0, b: 0 },
      border_scale: 1.0,
      opacity: 1.0,
      text_cache: None,
    };
  }
}

pub struct DrawGradientBoxArgs {
  pub position: F2,
  pub size: F2,
  pub draw_color_start: DrawColor,
  pub alpha_start: F1,
  pub draw_color_end: DrawColor,
  pub alpha_end: F1,
  pub depth: F1,
  pub anchor_point: F2,
}
