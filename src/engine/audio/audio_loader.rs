use crate::engine::*;

pub struct AudioLoader {
  loading: RefCell<Vec<Rc<Audio>>>,
  total_audios: Cell<usize>,
  started_loading: Cell<bool>,
}

pub struct AudioLoaderProgress {
  pub loaded: usize,
  pub total_audios: usize,
}

impl AudioLoader {
  pub fn new() -> AudioLoader {
    return AudioLoader {
      loading: RefCell::new(Vec::new()),
      total_audios: Cell::new(0),
      started_loading: Cell::new(false),
    };
  }

  pub fn start_loading(&self) {
    if self.started_loading.get() {
      panic!("start_loading was already called.");
    }
    self.started_loading.set(true);
    for audio in self.loading.borrow().iter() {
      audio.start_loading();
    }
  }

  // Must be called from user gesture callback.
  pub fn start_loading_ios(&self) {
    if self.started_loading.get() {
      panic!("start_loading was already called.");
    }
    self.started_loading.set(true);
    for audio in self.loading.borrow().iter() {
      audio.start_loading_ios();
    }
  }

  pub fn all_loaded(&self) -> bool {
    self.loading.borrow_mut().retain(|audio| !audio.is_loaded());
    return self.loading.borrow().is_empty();
  }

  pub fn get_progress(&self) -> AudioLoaderProgress {
    return AudioLoaderProgress {
      loaded: self.total_audios.get() - self.loading.borrow().len(),
      total_audios: self.total_audios.get(),
    };
  }

  pub fn register_audio(&self, src: &str) -> Rc<Audio> {
    if self.started_loading.get() {
      panic!("Can't register more audios after start_loading was called.");
    }
    self.total_audios.set(self.total_audios.get() + 1);
    let audio = Audio::new(src);
    self.loading.borrow_mut().push(audio.clone());
    return audio;
  }
}
