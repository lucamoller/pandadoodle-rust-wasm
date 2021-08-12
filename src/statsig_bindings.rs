use crate::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = ["window", "statsig_rustbindings"], js_name = getStatsigEnabled)]
  fn get_statsig_enabled() -> bool;

  #[wasm_bindgen(js_namespace = ["window", "statsig_rustbindings"], js_name = getStatsigGates)]
  fn get_statsig_gates() -> String;

  #[wasm_bindgen(js_namespace = ["window", "statsig_rustbindings"], js_name = logEvent)]
  fn log_event(event_name: &str);

  #[wasm_bindgen(js_namespace = ["window", "statsig_rustbindings"], js_name = logEventWithValue)]
  fn log_event_with_i32_value(event_name: &str, event_value: i32);
}

#[derive(Serialize, Deserialize, Default)]
pub struct StatsigGates {
  #[serde(default)]
  pub auto_load_game: bool,
  #[serde(default)]
  pub auto_load_first_stage: bool,
  #[serde(default)]
  pub skip_score_animation: bool,
}

pub struct StatsigBindings {
  pub statsig_enabled: Cell<bool>,
  pub statsig_gates: RefCell<StatsigGates>,
}

impl StatsigBindings {
  pub fn new() -> StatsigBindings {
    return StatsigBindings {
      statsig_enabled: Cell::new(false),
      statsig_gates: RefCell::default(),
    };
  }

  pub fn check_statsig_enabled(&self) {
    if !self.statsig_enabled.get() {
      let statsig_enabled = get_statsig_enabled();
      if statsig_enabled {
        self.statsig_enabled.set(statsig_enabled);
        self
          .statsig_gates
          .replace(serde_json::from_str(&get_statsig_gates()).unwrap_or_default());
      }
    }
  }

  pub fn log_event(&self, event_name: &str) {
    if self.statsig_enabled.get() {
      log_event(event_name);
    }
  }

  pub fn log_event_with_i32_value(&self, event_name: &str, event_value: i32) {
    if self.statsig_enabled.get() {
      log_event_with_i32_value(event_name, event_value);
    }
  }
}
