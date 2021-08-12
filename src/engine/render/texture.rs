use crate::engine::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct Texture {
  loaded: Cell<bool>,
  image: web_sys::HtmlImageElement,
  pub image_loaded: Shared<bool>,

  pub width: Cell<f64>,
  pub height: Cell<f64>,
  color_cache: RefCell<HashMap<DrawColor, Rc<web_sys::HtmlCanvasElement>>>,
  pub src: String,

  pub color_alpha_cache: Option<ColorAlphaCacheParams>,
  color_alpha_cache_canvas: Option<Rc<web_sys::HtmlCanvasElement>>,

  last_size: Cell<F2>,
  cache_hits: Cell<i32>,
  size_cache: RefCell<Option<Rc<web_sys::HtmlCanvasElement>>>,
}

impl Texture {
  pub fn new(
    document: &web_sys::Document,
    color_alpha_cache: Option<ColorAlphaCacheParams>,
    src: &str,
  ) -> Rc<Texture> {
    let image = document
      .create_element("img")
      .expect("document.create_element failed")
      .dyn_into::<web_sys::HtmlImageElement>()
      .expect("dyn_into::<web_sys::HtmlImageElement> failed");

    let color_alpha_cache_canvas = match color_alpha_cache {
      Some(_) => Some(Rc::new(
        document
          .create_element("canvas")
          .expect("failed to create canvas")
          .dyn_into::<web_sys::HtmlCanvasElement>()
          .expect("failed to dyn_into"),
      )),
      None => None,
    };
    return Rc::new(Texture {
      loaded: Cell::new(false),
      image: image,
      image_loaded: Shared::new(false),
      width: Cell::new(0.0),
      height: Cell::new(0.0),
      color_cache: RefCell::new(HashMap::new()),
      src: String::from(src),
      color_alpha_cache,
      color_alpha_cache_canvas,
      last_size: Cell::default(),
      cache_hits: Cell::new(0),
      size_cache: RefCell::new(None),
    });
  }

  pub fn start_loading(&self) {
    self.image.set_src(&self.src);
    {
      let imaged_loaded = self.image_loaded.clone();
      let closure = Closure::wrap(Box::new(move || {
        imaged_loaded.replace(true);
      }) as Box<dyn FnMut()>);
      self
        .image
        .set_onload(Some(closure.as_ref().unchecked_ref()));
      closure.forget();
    }
  }

  pub fn image(&self) -> &web_sys::HtmlImageElement {
    if !self.image_loaded.get() {
      panic!("Trying to use non-loaded image (src: {})", self.src);
    }
    return &self.image;
  }

  pub fn color_alpha_cache_canvas(&self) -> &Rc<web_sys::HtmlCanvasElement> {
    if !self.image_loaded.get() {
      panic!(
        "Trying to use non-loaded image's color_alpha_cache_canvas (src: {})",
        self.src
      );
    }
    return self.color_alpha_cache_canvas.as_ref().unwrap();
  }

  pub fn finish_loading(&self) -> bool {
    if !self.loaded.get() && self.image_loaded.get() {
      // console_log_with_div!(
      //   "image onload! size: {} x {}",
      //   texture_copy.borrow().image.width(),
      //   texture_copy.borrow().image.height()
      // );
      self.width.set(self.image.width() as f64);
      self.height.set(self.image.height() as f64);

      self.generate_color_alpha_cache_canvas();
      self.loaded.set(true);
    }

    return self.loaded.get();
  }

  pub fn get_size_from_width(&self, width: F1) -> F2 {
    return F2 {
      x: width,
      y: (width * (self.height.get() as F1)) / (self.width.get() as F1),
    };
  }

  pub fn generate_color_alpha_cache_canvas(&self) {
    if let Some(color_alpha_cache_params) = &self.color_alpha_cache {
      let canvas_size = self.get_color_alpha_cache_canvas_size(color_alpha_cache_params);
      let cache_canvas = self.color_alpha_cache_canvas.as_ref().unwrap();
      cache_canvas.set_width(canvas_size.x as u32);
      cache_canvas.set_height(canvas_size.y as u32);

      let canvas_context = cache_canvas
        .get_context("2d")
        .expect("failed to get_context(\"2d\")")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("failed to dyn_into::<web_sys::CanvasRenderingContext2d>()");

      let width = self.width.get() as f64;
      let height = self.height.get() as f64;

      for color in color_alpha_cache_params.colors.iter() {
        let colored_image = self.get_colored_image(&color);
        for i in 1..(color_alpha_cache_params.opacity_levels + 1) {
          let opacity = (i as F1) * (1.0 / color_alpha_cache_params.opacity_levels as F1);
          let coords = self.get_color_alpha_cache_coords(&color, &opacity);

          canvas_context.set_global_alpha(opacity as f64);
          canvas_context
            .draw_image_with_html_canvas_element_and_dw_and_dh(
              &colored_image,
              coords.x.into(),
              coords.y.into(),
              width,
              height,
            )
            .expect("draw_image_with_html_canvas_element_and_dw_and_dh failed");
        }
      }
    }
  }

  pub fn get_color_alpha_cache_canvas_size(
    &self,
    color_alpha_cache_params: &ColorAlphaCacheParams,
  ) -> F2 {
    return F2 {
      x: (self.width.get() as F1) * (color_alpha_cache_params.colors.len() as F1),
      y: (self.height.get() as F1) * (color_alpha_cache_params.opacity_levels as F1),
    };
  }

  pub fn get_color_alpha_cache_coords(&self, color: &DrawColor, opacity: &F1) -> F2 {
    let color_alpha_cache_params = self.color_alpha_cache.as_ref().unwrap();

    let color_index = color_alpha_cache_params
      .colors
      .iter()
      .position(|cached_color| cached_color == color);
    if color_index.is_none() {
      panic!(
        "Trying to use color that is not specificed in color_alpha_cache_params: {:?}",
        color
      );
    }
    let color_index = color_index.unwrap();

    return F2 {
      x: (self.width.get() as F1) * (color_index as F1),
      y: (self.height.get() as F1)
        * ((opacity * color_alpha_cache_params.opacity_levels as F1).round() - 1.0),
    };
  }

  pub fn get_colored_image(&self, color: &DrawColor) -> Rc<web_sys::HtmlCanvasElement> {
    if !self.image_loaded.get() {
      panic!(
        "Trying to use non-loaded image's get_colored_image (src: {})",
        self.src
      );
    }
    let mut color_cache = self.color_cache.borrow_mut();
    if !color_cache.contains_key(&color) {
      let document = web_sys::window().unwrap().document().unwrap();
      let tmp_canvas = document
        .create_element("canvas")
        .expect("failed to create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("failed to dyn_into");

      tmp_canvas.set_width(self.width.get() as u32);
      tmp_canvas.set_height(self.height.get() as u32);

      let tmp_canvas_context = tmp_canvas
        .get_context("2d")
        .expect("failed to get_context(\"2d\")")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("failed to dyn_into::<web_sys::CanvasRenderingContext2d>()");

      tmp_canvas_context.set_fill_style(&color.as_rgb_js_value());
      tmp_canvas_context.fill_rect(0.0, 0.0, self.width.get().into(), self.height.get().into());

      tmp_canvas_context
        .set_global_composite_operation("multiply")
        .expect("failed to set_global_composite_operation");
      tmp_canvas_context
        .draw_image_with_html_image_element_and_dw_and_dh(
          &self.image,
          0.0,
          0.0,
          self.width.get().into(),
          self.height.get().into(),
        )
        .expect("draw_image_with_html_image_element_and_dw_and_dh failed");

      tmp_canvas_context
        .set_global_composite_operation("destination-in")
        .expect("failed to set_global_composite_operation");
      tmp_canvas_context
        .draw_image_with_html_image_element_and_dw_and_dh(
          &self.image,
          0.0,
          0.0,
          self.width.get().into(),
          self.height.get().into(),
        )
        .expect("draw_image_with_html_image_element_and_dw_and_dh failed");

      color_cache.insert(*color, Rc::new(tmp_canvas));
    }

    return color_cache.get(color).unwrap().clone();
  }

  pub fn get_size_cache(&self, size: F2) -> Option<Rc<web_sys::HtmlCanvasElement>> {
    if size != self.last_size.get() {
      self.cache_hits.set(0);
      self.last_size.set(size);
      return None;
    }

    self.cache_hits.set(self.cache_hits.get() + 1);
    if self.cache_hits.get() == 1 {
      if self.size_cache.borrow().is_none() {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
          .create_element("canvas")
          .expect("failed to create canvas")
          .dyn_into::<web_sys::HtmlCanvasElement>()
          .expect("failed to dyn_into");
        self.size_cache.replace(Some(Rc::new(canvas)));
      }
      let canvas = self.size_cache.borrow().as_ref().unwrap().clone();
      canvas.set_width(size.x as u32);
      canvas.set_height(size.y as u32);

      let canvas_context = canvas
        .get_context("2d")
        .expect("failed to get_context(\"2d\")")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("failed to dyn_into::<web_sys::CanvasRenderingContext2d>()");

      canvas_context
        .draw_image_with_html_image_element_and_dw_and_dh(
          &self.image,
          0.0,
          0.0,
          size.x.into(),
          size.y.into(),
        )
        .expect("draw_image_with_html_image_element_and_dw_and_dh failed");

      return Some(canvas);
    }
    return self.size_cache.borrow().clone();
  }
}
