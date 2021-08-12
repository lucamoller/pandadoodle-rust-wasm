use crate::engine::*;

pub trait AppTrait {
  fn process_input_event(&mut self, input_event: InputEvent);

  fn check_fullscreen(&mut self);
}
