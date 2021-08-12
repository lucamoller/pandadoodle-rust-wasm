#[derive(PartialEq, Copy, Clone)]
pub enum TouchEventType {
  Pressed,
  Moved,
  Released,
}

#[derive(Copy, Clone)]
pub struct TouchEvent {
  pub id: i32,
  pub x: i32,
  pub y: i32,
  pub event_type: TouchEventType,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MouseEventType {
  Pressed,
  Moved,
  Released,
}

#[derive(Copy, Clone)]
pub struct MouseEvent {
  pub x: i32,
  pub y: i32,
  pub pressed_buttons: u16,
  pub event_type: MouseEventType,
}

#[derive(Copy, Clone)]
pub enum InputEvent {
  Touch(TouchEvent),
  Mouse(MouseEvent),
  KeyDown(u32),
  KeyUp(u32),
  BackButton,
}
