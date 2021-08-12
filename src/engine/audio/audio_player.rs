use crate::engine::*;

pub struct AudioPlayer {
  local_storage: Rc<web_sys::Storage>,
  local_storage_key: String,
  song_playing: Option<Rc<Audio>>,
  pub settings: AudioPlayerSettings,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AudioPlayerSettings {
  song_volume: F1,
  pub song_muted: bool,
  sound_volume: F1,
  pub sound_muted: bool,
}

fn load_audio_settings(
  local_storage: &web_sys::Storage,
  local_storage_key: &str,
) -> AudioPlayerSettings {
  return LocalStorageUtil::read(local_storage, local_storage_key).unwrap_or_else(
    || -> AudioPlayerSettings {
      return AudioPlayerSettings {
        song_volume: 1.0,
        song_muted: false,
        sound_volume: 1.0,
        sound_muted: false,
      };
    },
  );
}

fn store_audio_settings(
  local_storage: &web_sys::Storage,
  settings: &AudioPlayerSettings,
  local_storage_key: &str,
) {
  LocalStorageUtil::write(local_storage, local_storage_key, settings);
}

impl AudioPlayer {
  pub fn new(local_storage: Rc<web_sys::Storage>, local_storage_key: String) -> AudioPlayer {
    return AudioPlayer {
      local_storage: local_storage.clone(),
      local_storage_key: local_storage_key.clone(),
      song_playing: None,
      settings: load_audio_settings(&local_storage, &local_storage_key),
    };
  }

  pub fn play_song(&mut self, song: &Rc<Audio>) {
    if let Some(song_playing) = self.song_playing.as_ref() {
      if song_playing.is_playing() && RcUtil::eq_ptr(song_playing, song) {
        return;
      }
      song_playing.pause_song();
    }
    self.song_playing = Some(song.clone());
    if !self.settings.song_muted {
      song.play_song(self.settings.song_volume);
    }
  }

  pub fn stop_song(&mut self) {
    if let Some(song_playing) = self.song_playing.as_ref() {
      song_playing.pause_song();
    }
  }

  pub fn resume_song(&mut self) {
    if let Some(song_playing) = self.song_playing.as_ref() {
      if !self.settings.song_muted {
        song_playing.resume_song(self.settings.song_volume);
      }
    }
  }

  pub fn play_sound(&mut self, sound: &Rc<Audio>) {
    if !self.settings.sound_muted {
      sound.play_sound(self.settings.sound_volume);
    }
  }

  pub fn stop_sound(&mut self, sound: &Rc<Audio>) {
    sound.stop_sound();
  }

  pub fn set_song_volume(&mut self, song_volume: F1) {
    self.settings.song_volume = song_volume;
    store_audio_settings(&self.local_storage, &self.settings, &self.local_storage_key);
    if let Some(song_playing) = self.song_playing.as_ref() {
      song_playing.set_volume(song_volume);
    }
  }

  pub fn get_song_volume(&self) -> F1 {
    return self.settings.song_volume;
  }

  pub fn toggle_mute_song(&mut self) {
    self.settings.song_muted = !self.settings.song_muted;
    store_audio_settings(&self.local_storage, &self.settings, &self.local_storage_key);
    if self.settings.song_muted {
      self.stop_song();
    } else {
      self.resume_song();
    }
  }

  pub fn set_sound_volume(&mut self, sound_volume: F1) {
    self.settings.sound_volume = sound_volume;
    store_audio_settings(&self.local_storage, &self.settings, &self.local_storage_key);
  }

  pub fn get_sound_volume(&self) -> F1 {
    return self.settings.sound_volume;
  }

  pub fn toggle_mute_sound(&mut self) {
    self.settings.sound_muted = !self.settings.sound_muted;
    store_audio_settings(&self.local_storage, &self.settings, &self.local_storage_key);
  }
}
