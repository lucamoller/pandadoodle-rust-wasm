use crate::engine::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = loadAudio)]
  fn load_audio(src: &str, ios: bool);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = play)]
  fn play(src: &str);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = pause)]
  fn pause(src: &str);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = isPlaying)]
  fn is_playing(src: &str) -> bool;

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = setVolume)]
  fn set_volume(src: &str, volume: f64);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = setLoop)]
  fn set_loop(src: &str, looped: bool);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = setSeek)]
  fn set_seek(src: &str, position_secs: f64);

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = getSeek)]
  fn get_seek(src: &str) -> f64;

  #[wasm_bindgen(js_namespace = ["window", "howler_bindings"], js_name = isLoaded)]
  fn is_loaded(src: &str) -> bool;

}

pub struct Audio {
  pub src: String,
}

impl Audio {
  pub fn new(src: &str) -> Rc<Audio> {
    return Rc::new(Audio {
      src: String::from(src),
    });
  }

  pub fn start_loading(&self) {
    load_audio(&self.src, false);
  }

  // Must be called from user gesture callback.
  pub fn start_loading_ios(&self) {
    load_audio(&self.src, true);
  }

  pub fn is_loaded(&self) -> bool {
    return is_loaded(&self.src);
  }

  pub fn play_song(&self, song_volume: F1) {
    set_volume(&self.src, song_volume as f64);
    set_seek(&self.src, 0.0);
    set_loop(&self.src, true);
    play(&self.src);
  }

  pub fn resume_song(&self, song_volume: F1) {
    if !self.is_playing() {
      set_volume(&self.src, song_volume as f64);
      play(&self.src);
    }
  }

  pub fn pause_song(&self) {
    pause(&self.src);
  }

  pub fn is_playing(&self) -> bool {
    return is_playing(&self.src);
  }

  pub fn current_time(&self) -> F1 {
    return get_seek(&self.src) as F1;
  }

  pub fn play_sound(&self, sound_volume: F1) {
    set_volume(&self.src, sound_volume as f64);
    set_seek(&self.src, 0.0);
    set_loop(&self.src, false);
    play(&self.src);
  }

  pub fn stop_sound(&self) {
    pause(&self.src);
  }

  pub fn set_volume(&self, volume: F1) {
    set_volume(&self.src, volume as f64);
  }
}

// HTML5 audio implementation
// use crate::engine::*;
// use futures::future::TryFutureExt;
// use wasm_bindgen::prelude::*;

// pub struct Audio {
//   pub audio_element: Rc<web_sys::HtmlAudioElement>,
//   pub src: String,
//   pub initial_song: Cell<bool>,
// }

// impl Audio {
//   pub fn new(src: &str) -> Rc<Audio> {
//     return Rc::new(Audio {
//       audio_element: Rc::new(
//         web_sys::HtmlAudioElement::new().expect("failed to create web_sys::HtmlAudioElement"),
//       ),
//       src: String::from(src),
//       initial_song: Cell::new(false),
//     });
//   }

//   pub fn start_loading(&self) {
//     self.audio_element.set_preload("auto");
//     self.audio_element.set_src(&self.src);
//   }

//   // Must be called from user gesture callback.
//   pub fn start_loading_ios(&self) {
//     self.audio_element.set_preload("auto");
//     self.audio_element.set_src(&self.src);
//     let play_promise = self
//       .audio_element
//       .play()
//       .expect("audio_element.play failed");

//     if !self.initial_song.get() {
//       let audio_element = self.audio_element.clone();
//       let _ = wasm_bindgen_futures::future_to_promise(
//         wasm_bindgen_futures::JsFuture::from(play_promise).and_then(move |_| {
//           audio_element.pause().expect("audio_element.pause failed");
//           return futures::future::ok(JsValue::null());
//         }),
//       );
//     }
//   }

//   pub fn is_loaded(&self) -> bool {
//     return self.audio_element.ready_state() == 4;
//   }

//   pub fn play_song(&self, song_volume: F1) {
//     self.audio_element.set_volume(song_volume as f64);
//     self.audio_element.set_current_time(0.0);
//     self.audio_element.set_loop(true);
//     let _ = self
//       .audio_element
//       .play()
//       .expect("Failed self.audio_element.play");
//   }

//   pub fn resume_song(&self, song_volume: F1) {
//     self.audio_element.set_volume(song_volume as f64);
//     let _ = self
//       .audio_element
//       .play()
//       .expect("Failed self.audio_element.play");
//   }

//   pub fn pause_song(&self) {
//     let _ = self
//       .audio_element
//       .pause()
//       .expect("Failed self.audio_element.pause");
//   }

//   pub fn is_playing(&self) -> bool {
//     return !self.audio_element.paused() && self.audio_element.current_time() != 0.0;
//   }

//   pub fn current_time(&self) -> F1 {
//     return self.audio_element.current_time() as F1;
//   }

//   pub fn play_sound(&self, sound_volume: F1) {
//     self.audio_element.set_volume(sound_volume as f64);
//     self.audio_element.set_current_time(0.0);
//     self.audio_element.set_loop(false);
//     let _ = self
//       .audio_element
//       .play()
//       .expect("Failed self.audio_element.play");
//   }

//   pub fn stop_sound(&self) {
//     let _ = self
//       .audio_element
//       .pause()
//       .expect("Failed self.audio_element.play");
//   }

//   pub fn set_volume(&self, volume: F1) {
//     self.audio_element.set_volume(volume as f64);
//   }
// }
