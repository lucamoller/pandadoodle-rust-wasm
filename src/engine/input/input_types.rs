use crate::engine::*;

const LEFT_CLICK_MASK: u16 = 1;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputState {
  Available,
  Consumed,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TouchType {
  Pressed,
  Moved,
  Released,
}

#[derive(Debug)]
pub struct ScreenTouch {
  pub id: i32,
  pub position: F2,
  pub touch_type: TouchType,
}

impl ScreenTouch {
  pub fn from_event(event: &InputEvent) -> Option<ScreenTouch> {
    return match event {
      InputEvent::Touch(touch_event) => Some(ScreenTouch {
        id: touch_event.id,
        position: F2 {
          x: touch_event.x as F1,
          y: touch_event.y as F1,
        },
        touch_type: match touch_event.event_type {
          TouchEventType::Pressed => TouchType::Pressed,
          TouchEventType::Moved => TouchType::Moved,
          TouchEventType::Released => TouchType::Released,
        },
      }),
      InputEvent::Mouse(mouse_event) => {
        if mouse_event.pressed_buttons & LEFT_CLICK_MASK == LEFT_CLICK_MASK
          || (mouse_event.pressed_buttons & LEFT_CLICK_MASK != LEFT_CLICK_MASK
            && mouse_event.event_type == MouseEventType::Released)
        {
          Some(ScreenTouch {
            id: 0,
            position: F2 {
              x: mouse_event.x as F1,
              y: mouse_event.y as F1,
            },
            touch_type: match mouse_event.event_type {
              MouseEventType::Pressed => TouchType::Pressed,
              MouseEventType::Moved => TouchType::Moved,
              MouseEventType::Released => TouchType::Released,
            },
          })
        } else {
          None
        }
      }
      _ => None,
    };
  }
}

pub struct UiTouch {
  pub id: i32,
  pub position: F2,
  pub delta: F2,
  pub touch_type: TouchType,
}

impl UiTouch {
  pub fn from_screen_touch<C: ContextTrait + ?Sized>(
    screen_touch: &ScreenTouch,
    context: &mut C,
  ) -> UiTouch {
    return UiTouch {
      id: screen_touch.id,
      position: context
        .get_ui_viewport()
        .screen_to_viewport(&screen_touch.position),
      delta: F2 { x: 0.0, y: 0.0 },
      touch_type: screen_touch.touch_type,
    };
  }
}

#[derive(Debug)]
pub struct GameTouch {
  pub id: i32,
  pub position: F2,
  pub touch_type: TouchType,
}

impl GameTouch {
  pub fn from_screen_touch<C: ContextTrait + ?Sized>(
    screen_touch: &ScreenTouch,
    context: &mut C,
  ) -> GameTouch {
    return GameTouch {
      id: screen_touch.id,
      position: context
        .get_game_viewport()
        .screen_to_viewport(&screen_touch.position),
      touch_type: screen_touch.touch_type,
    };
  }
}
