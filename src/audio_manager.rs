use crate::engine::*;

pub struct AudioManager {
  pub back: Rc<Audio>,
  pub book: Rc<Audio>,
  pub click: Rc<Audio>,
  pub collect: Rc<Audio>,
  pub count_point: Rc<Audio>,
  pub song1: Rc<Audio>,
  pub song2: Rc<Audio>,
  pub star1: Rc<Audio>,
  pub star2: Rc<Audio>,
  pub star3: Rc<Audio>,
  pub win: Rc<Audio>,
  pub wrong: Rc<Audio>,

  pub audio_loader: Rc<AudioLoader>,
}

impl AudioManager {
  pub fn new() -> AudioManager {
    let audio_loader = AudioLoader::new();
    return AudioManager {
      back: audio_loader.register_audio("/static/sound/sound_back.wav"),
      book: audio_loader.register_audio("/static/sound/sound_book_lower2.wav"),
      click: audio_loader.register_audio("/static/sound/sound_click.wav"),
      collect: audio_loader.register_audio("/static/sound/sound_collect_lower.wav"),
      count_point: audio_loader.register_audio("/static/sound/sound_countpoint.wav"),
      song1: audio_loader.register_audio("/static/sound/sound_song1.mp3"),
      song2: audio_loader.register_audio("/static/sound/sound_song2.mp3"),
      star1: audio_loader.register_audio("/static/sound/sound_star1_lower.wav"),
      star2: audio_loader.register_audio("/static/sound/sound_star2_lower.wav"),
      star3: audio_loader.register_audio("/static/sound/sound_star3_lower.wav"),
      win: audio_loader.register_audio("/static/sound/sound_win_lower.wav"),
      wrong: audio_loader.register_audio("/static/sound/sound_wrong.wav"),
      audio_loader: Rc::new(audio_loader),
    };
  }

  pub fn is_loaded(&mut self) -> bool {
    return self.audio_loader.all_loaded();
  }
}
