use crate::context::*;
use crate::engine::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;

const N_SLICES: u32 = 40;

pub struct GlowBar {
  pub texture: Rc<Texture>,
  pub time: F1,
  pub position: F2,
  pub size: F2,
  cache_canvas: Rc<web_sys::HtmlCanvasElement>,
  cache_canvas_size: F2,
  cache_draw_backend: Canvas2dDrawBackend,
}

impl GlowBar {
  pub fn new(texture: Rc<Texture>, position: F2, size: F2, context: &Context) -> GlowBar {
    let document = web_sys::window().unwrap().document().unwrap();
    let cache_canvas = document
      .create_element("canvas")
      .expect("failed to create canvas")
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .expect("failed to dyn_into");
    let mut canvas_size = context.get_canvas_size();

    canvas_size.x = 240.0;
    canvas_size.y = size.y * canvas_size.x / size.x;
    cache_canvas.set_width(canvas_size.x as u32);
    cache_canvas.set_height(canvas_size.y as u32);
    let canvas_context = Rc::new(
      cache_canvas
        .get_context("2d")
        .expect("failed to get_context(\"2d\")")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("failed to dyn_into::<web_sys::CanvasRenderingContext2d>()"),
    );

    return GlowBar {
      texture: texture,
      time: 0.0,
      position: position,
      size: size,
      cache_canvas: Rc::new(cache_canvas),
      cache_canvas_size: canvas_size,
      cache_draw_backend: Canvas2dDrawBackend::new(canvas_context),
    };
  }

  pub fn update(&mut self, dt: &F1) {
    self.time += dt * 0.8;
  }

  pub fn draw(&mut self, context: &mut Context) {
    let canvas_total_size = self.cache_canvas_size;
    let mut slice_size_dest_x: u32 = ((canvas_total_size.x as u32) / N_SLICES) + 1;
    let slice_size_dest_y: u32 = canvas_total_size.y as u32;
    let remainder_screen = (canvas_total_size.x as u32) % N_SLICES;

    let mut slice_pos_dest_x: u32 = 0;
    let slice_pos_dest_y: u32 = 0;

    let wave_lenght_width: F1 = 1.5 * self.size.x * 480.0 / (N_SLICES as F1);

    let slice_pos_dest_x_initial = slice_pos_dest_x;

    self
      .cache_draw_backend
      .clear_screen(&self.cache_canvas_size);

    for i in 0..(N_SLICES as u32) {
      let intensity: F1 = 0.4
        + 0.4
          * 0.25
          * (((i as F1) * wave_lenght_width * 0.0015 + self.time * 0.001).sin()
            + ((i as F1) * wave_lenght_width * 0.008 - self.time * 0.0013).sin()
            + ((i as F1) * wave_lenght_width * 0.015 + self.time * 0.0037).sin()
            + ((i as F1) * wave_lenght_width * 0.013 - self.time * 0.0059).sin());

      if i == remainder_screen {
        slice_size_dest_x -= 1;
      }

      self.cache_draw_backend.execute_image_draw(
        &DrawImageArgs {
          source: DrawSource::Texture(self.texture.clone()),
          position: F2 {
            x: slice_pos_dest_x as F1,
            y: slice_pos_dest_y as F1,
          },
          size: F2 {
            x: slice_size_dest_x as F1,
            y: slice_size_dest_y as F1,
          },
          depth: context.draw_depths.ui - 100.0,
          optional: DrawImageOptionalArgs {
            anchor_point: F2 { x: 0.0, y: 0.0 },
            partial_region_offset: F2 {
              x: ((slice_pos_dest_x - slice_pos_dest_x_initial) as F1) / canvas_total_size.x,
              y: 0.0,
            },
            partial_region_size: F2 {
              x: slice_size_dest_x as F1 / canvas_total_size.x,
              y: 1.0,
            },
            opacity: intensity,
            subpixel_precision: true,
            ..Default::default()
          },
        },
        &self.cache_canvas_size,
      );

      slice_pos_dest_x += slice_size_dest_x;
    }

    context.draw_manager.draw_viewport(
      &context.ui_viewport,
      DrawImageArgs {
        source: DrawSource::Canvas(self.cache_canvas.clone()),
        position: self.position,
        size: self.size,
        depth: context.draw_depths.ui - 100.0,
        optional: DrawImageOptionalArgs {
          opacity: 1.0,
          subpixel_precision: true,
          composite_operation: Some(String::from("lighter")),
          ..Default::default()
        },
      },
    );
  }
}
