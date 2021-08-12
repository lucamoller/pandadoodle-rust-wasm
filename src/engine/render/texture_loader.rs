use crate::engine::*;

pub struct TextureLoader {
  document: Rc<web_sys::Document>,
  loading: Vec<Rc<Texture>>,
  started_loading: bool,
  total_textures: Cell<usize>,
}

pub struct TextureLoaderProgress {
  pub loaded: usize,
  pub total_textures: usize,
}

#[derive(Default)]
pub struct TextureParams {
  pub src: String,
  pub optional: TextureParamsOptional,
}

#[derive(Default)]
pub struct TextureParamsOptional {
  pub color_alpha_cache: Option<ColorAlphaCacheParams>,
}

pub struct ColorAlphaCacheParams {
  pub opacity_levels: usize,
  pub colors: Vec<DrawColor>,
}

impl TextureLoader {
  pub fn new(document: Rc<web_sys::Document>) -> TextureLoader {
    return TextureLoader {
      document: document,
      loading: Vec::new(),
      started_loading: false,
      total_textures: Cell::new(0),
    };
  }

  pub fn start_loading(&mut self) {
    if self.started_loading {
      panic!("start_loading was already called.");
    }
    self.started_loading = true;
    for texture in self.loading.iter() {
      texture.start_loading();
    }
  }

  pub fn has_started_loading(&self) -> bool {
    return self.started_loading;
  }

  pub fn all_loaded(&mut self) -> bool {
    self.loading.retain(|texture| !texture.finish_loading());
    return self.loading.is_empty();
  }

  pub fn get_progress(&self) -> TextureLoaderProgress {
    return TextureLoaderProgress {
      loaded: self.total_textures.get() - self.loading.len(),
      total_textures: self.total_textures.get(),
    };
  }

  pub fn register(&mut self, texture_params: TextureParams) -> Rc<Texture> {
    if self.started_loading {
      panic!("Can't register more textures after start_loading was called.");
    }
    self.total_textures.set(self.total_textures.get() + 1);
    let texture = Texture::new(
      self.document.as_ref(),
      texture_params.optional.color_alpha_cache,
      &texture_params.src,
    );
    self.loading.push(texture.clone());
    return texture;
  }
}
