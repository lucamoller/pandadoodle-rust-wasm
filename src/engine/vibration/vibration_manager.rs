use crate::engine::*;

pub struct VibrationManager {
  local_storage: Rc<web_sys::Storage>,
  local_storage_key: String,
  ios: bool,
  navigator: web_sys::Navigator,
  vibration_settings: VibrationManagerSettings,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum VibrationLevel {
  None,
  Low,
  Medium,
  High,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VibrationManagerSettings {
  vibration_level: Cell<VibrationLevel>,
}

impl Default for VibrationManagerSettings {
  fn default() -> VibrationManagerSettings {
    return VibrationManagerSettings {
      vibration_level: Cell::new(VibrationLevel::Medium),
    };
  }
}

impl VibrationManager {
  pub fn new(
    window: &web_sys::Window,
    ios: bool,
    local_storage: Rc<web_sys::Storage>,
    local_storage_key: String,
  ) -> VibrationManager {
    let vibration_settings: VibrationManagerSettings =
      LocalStorageUtil::read(&local_storage, &local_storage_key).unwrap_or_default();
    if ios {
      vibration_settings.vibration_level.set(VibrationLevel::None);
    }
    return VibrationManager {
      local_storage: local_storage.clone(),
      local_storage_key: local_storage_key.clone(),
      ios: ios,
      navigator: window.navigator(),
      vibration_settings: vibration_settings,
    };
  }

  pub fn vibrate(&self) {
    match self.vibration_settings.vibration_level.get() {
      VibrationLevel::None => {}
      VibrationLevel::Low => {
        self.navigator.vibrate_with_duration(10);
      }
      VibrationLevel::Medium => {
        self.navigator.vibrate_with_duration(20);
      }
      VibrationLevel::High => {
        self.navigator.vibrate_with_duration(30);
      }
    }
  }

  pub fn switch_vibration_level(&self, context: &impl ContextTrait) {
    if self.ios {
      context.alert("iOS doesn't support vibrate on browsers.");
      return;
    }
    self.vibration_settings.vibration_level.set(
      match self.vibration_settings.vibration_level.get() {
        VibrationLevel::None => VibrationLevel::Low,
        VibrationLevel::Low => VibrationLevel::Medium,
        VibrationLevel::Medium => VibrationLevel::High,
        VibrationLevel::High => VibrationLevel::None,
      },
    );
    LocalStorageUtil::write(
      &self.local_storage,
      &self.local_storage_key,
      &self.vibration_settings,
    );
  }

  pub fn get_vibration_level(&self) -> &str {
    return match self.vibration_settings.vibration_level.get() {
      VibrationLevel::None => &"None",
      VibrationLevel::Low => &"Low",
      VibrationLevel::Medium => &"Medium",
      VibrationLevel::High => &"High",
    };
  }
}
