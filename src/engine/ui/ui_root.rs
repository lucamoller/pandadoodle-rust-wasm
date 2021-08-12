use crate::engine::*;

pub trait UiRootTrait<C: ContextTrait + ?Sized>: UiElementTrait<C> {
  fn on_back_to(&self, _context: &mut C) {}
  fn on_back_from(&self, _context: &mut C) {}
  fn on_navigate_to(&self, _context: &mut C) {}
  fn on_navigate_from(&self, _context: &mut C) {}
  fn on_reactivate(&self, _context: &mut C) {}
  // Called when the press button is pressed. Return true to let UiManager go
  // back to the previus UiRoot. Return false to block UiManager from going back.
  fn on_press_back(&self, _context: &mut C) -> InputState {
    return InputState::Available;
  }

  fn process_touch_game(&self, _context: &mut C, _ui_touch: &ScreenTouch) -> bool {
    return false;
  }
}
