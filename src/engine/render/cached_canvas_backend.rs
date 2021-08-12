use crate::engine::*;
use wasm_bindgen::JsCast;

pub struct CachedCanvasBackend {
  pub canvas: Rc<web_sys::HtmlCanvasElement>,
  pub canvas_context: Rc<web_sys::CanvasRenderingContext2d>,
  pub canvas_size: Cell<F2>,
  pub draw_backend: Canvas2dDrawBackend,
  pub clear_cache_required: Cell<bool>,
}

impl CachedCanvasBackend {
  pub fn new(canvas_size: &F2) -> CachedCanvasBackend {
    let document = web_sys::window().unwrap().document().unwrap();
    let cache_canvas = document
      .create_element("canvas")
      .expect("failed to create canvas")
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .expect("failed to dyn_into");
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

    return CachedCanvasBackend {
      canvas: Rc::new(cache_canvas),
      canvas_context: canvas_context.clone(),
      canvas_size: Cell::new(*canvas_size),
      draw_backend: Canvas2dDrawBackend::new(canvas_context),
      clear_cache_required: Cell::new(true),
    };
  }

  pub fn check_canvas_size_changed(&self, expected_canvas_size: &F2) {
    if self.canvas_size.get() != *expected_canvas_size {
      self.canvas_size.set(*expected_canvas_size);
      self.clear_cache_required.set(true);
      self.canvas.set_width(expected_canvas_size.x as u32);
      self.canvas.set_height(expected_canvas_size.y as u32);
    }
  }

  pub fn check_clear_cache(&self) -> bool {
    let cache_cleared = self.clear_cache_required.get();
    if cache_cleared {
      self.clear_cache_required.set(false);
      self.draw_backend.clear_screen(&self.canvas_size.get());
    }
    return cache_cleared;
  }
}
