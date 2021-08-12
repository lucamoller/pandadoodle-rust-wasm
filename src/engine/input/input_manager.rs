use crate::engine::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const LOG_INPUT_EVENTS: bool = false;
const MAX_TOUCHES_TRACKED: usize = 10;

#[derive(Clone)]
struct LastTouchCoords {
  pub x: i32,
  pub y: i32,
}

pub struct InputManager {
  last_touches: Rc<RefCell<Vec<LastTouchCoords>>>,
  touch_indexes: RefCell<Vec<i32>>,
}

impl InputManager {
  pub fn register(
    app_shared: Shared<dyn AppTrait>,
    window: &web_sys::Window,
    canvas: &web_sys::HtmlCanvasElement,
    history: Rc<web_sys::History>,
  ) {
    let input_manager = Rc::new(InputManager {
      last_touches: Rc::new(RefCell::new(vec![
        LastTouchCoords { x: 0, y: 0 };
        MAX_TOUCHES_TRACKED
      ])),
      touch_indexes: RefCell::new(vec![-1; MAX_TOUCHES_TRACKED]),
    });
    register_back_button_listener(window, app_shared.clone(), history.clone());
    register_touch_start(canvas, app_shared.clone(), input_manager.clone());
    register_touch_move(canvas, app_shared.clone(), input_manager.clone());
    register_touch_end(canvas, app_shared.clone(), input_manager.clone());
    register_mouse_down(canvas, app_shared.clone());
    register_mouse_move(canvas, app_shared.clone());
    register_mouse_up(canvas, app_shared.clone());
    register_context_menu(canvas);
  }

  fn get_index_for_touch(&self, touch_id: i32) -> usize {
    let mut touch_indexes = self.touch_indexes.borrow_mut();
    for (index, mapped_touch_id) in touch_indexes.iter().enumerate() {
      if touch_id == *mapped_touch_id {
        return index;
      }
    }

    for index in 0..touch_indexes.len() {
      if touch_indexes[index] == -1 {
        touch_indexes[index] = touch_id;
        return index;
      }
    }
    return 10;
  }

  fn update_unused_indexes(&self, touch_ids: Vec<i32>) -> Vec<usize> {
    let mut missing = Vec::new();
    let mut touch_indexes = self.touch_indexes.borrow_mut();
    for index in 0..touch_indexes.len() {
      if touch_indexes[index] != -1 && !touch_ids.contains(&touch_indexes[index]) {
        missing.push(index);
        touch_indexes[index] = -1;
      }
    }
    return missing;
  }
}

fn register_back_button_listener(
  window: &web_sys::Window,
  app_shared: Shared<dyn AppTrait>,
  history: Rc<web_sys::History>,
) {
  let history = history.clone();
  history
    .push_state(&JsValue::from_str("gameMainPage"), "Game Main Page")
    .expect("history.push_state failed");
  history
    .push_state(&JsValue::from_str("gameRunning"), "Game Running")
    .expect("history.push_state failed");

  let app_shared = app_shared.clone();
  let closure = Closure::wrap(Box::new(move |event: web_sys::PopStateEvent| {
    if let Some(_state) = event.state().as_string() {
      // console_log_with_div!("onpopstate event! state: {}", state);
      app_shared
        .borrow_mut()
        .process_input_event(InputEvent::BackButton)
      // em.events.borrow_mut().push(InputEvent::BackButton);
    } else {
      // console_log_with_div!("onpopstate event! no state string");
      history.back().expect("history.back failed");
    }
  }) as Box<dyn FnMut(_)>);
  window
    .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
    .expect("window.add_event_listener_with_callback popstate failed");
  closure.forget();
}

fn register_touch_start(
  canvas: &web_sys::HtmlCanvasElement,
  app_shared: Shared<dyn AppTrait>,
  input_manager: Rc<InputManager>,
) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
    event.prevent_default();
    let mut app_mut_ref = app_shared.borrow_mut();
    let app = &mut *app_mut_ref;
    match event.touches().get(0) {
      Some(touch) => {
        let touch_index = input_manager.get_index_for_touch(touch.identifier());
        let input_event = InputEvent::Touch(TouchEvent {
          id: touch_index as i32,
          x: touch.client_x(),
          y: touch.client_y(),
          event_type: TouchEventType::Pressed,
        });
        input_manager.last_touches.borrow_mut()[touch_index] = LastTouchCoords {
          x: touch.client_x(),
          y: touch.client_y(),
        };
        app.process_input_event(input_event);

        if LOG_INPUT_EVENTS {
          console_log_with_div!(
            "touchstart - id {}, index {}, x {}, y {}",
            touch.identifier(),
            input_manager.get_index_for_touch(touch.identifier()),
            touch.client_x(),
            touch.client_y()
          );
        }
      }
      None => {}
    };
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback touchstart failed");
  closure.forget();
}

fn register_touch_move(
  canvas: &web_sys::HtmlCanvasElement,
  app_shared: Shared<dyn AppTrait>,
  input_manager: Rc<InputManager>,
) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
    event.prevent_default();
    for i in 0..event.touches().length() {
      match event.touches().get(i) {
        Some(touch) => {
          let touch_index = input_manager.get_index_for_touch(touch.identifier());
          let input_event = InputEvent::Touch(TouchEvent {
            id: touch_index as i32,
            x: touch.client_x(),
            y: touch.client_y(),
            event_type: TouchEventType::Moved,
          });
          input_manager.last_touches.borrow_mut()[touch_index] = LastTouchCoords {
            x: touch.client_x(),
            y: touch.client_y(),
          };
          app_shared.borrow_mut().process_input_event(input_event);
          if LOG_INPUT_EVENTS {
            console_log_with_div!(
              "touchmove: id {}, index {}, x {}, y {}",
              touch.identifier(),
              input_manager.get_index_for_touch(touch.identifier()),
              touch.client_x(),
              touch.client_y()
            );
          }
        }
        None => {}
      };
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback touchmove failed");
  closure.forget();
}

fn register_touch_end(
  canvas: &web_sys::HtmlCanvasElement,
  app_shared: Shared<dyn AppTrait>,
  input_manager: Rc<InputManager>,
) {
  let app_shared = app_shared.clone();
  let input_manager = input_manager.clone();
  let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
    let mut app_mut_ref = app_shared.borrow_mut();
    let app = &mut *app_mut_ref;
    app.check_fullscreen();

    event.prevent_default();

    let mut touch_ids = vec![];
    for i in 0..event.touches().length() {
      match event.touches().get(i) {
        Some(touch) => {
          touch_ids.push(touch.identifier());
        }
        None => {}
      };
    }
    let missing_indexes = input_manager.update_unused_indexes(touch_ids);
    for missing_index in missing_indexes.iter() {
      let input_event = InputEvent::Touch(TouchEvent {
        id: *missing_index as i32,
        x: input_manager.last_touches.borrow()[*missing_index].x,
        y: input_manager.last_touches.borrow()[*missing_index].y,
        event_type: TouchEventType::Released,
      });
      app.process_input_event(input_event);
    }

    if LOG_INPUT_EVENTS {
      console_log_with_div!("touchend: {}", event.touches().length());
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback touchend failed");
  closure.forget();
}

fn register_mouse_down(canvas: &web_sys::HtmlCanvasElement, app_shared: Shared<dyn AppTrait>) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    event.prevent_default();
    let input_event = InputEvent::Mouse(MouseEvent {
      x: event.client_x(),
      y: event.client_y(),
      pressed_buttons: event.buttons(),
      event_type: MouseEventType::Pressed,
    });
    app_shared.borrow_mut().process_input_event(input_event);
    if LOG_INPUT_EVENTS {
      console_log_with_div!(
        "mousedown: {}, {}, {}",
        event.client_x(),
        event.client_y(),
        event.buttons()
      );
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback mousedown failed");
  closure.forget();
}

fn register_mouse_move(canvas: &web_sys::HtmlCanvasElement, app_shared: Shared<dyn AppTrait>) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    event.prevent_default();
    if event.buttons() == 0 {
      return;
    }
    let input_event = InputEvent::Mouse(MouseEvent {
      x: event.client_x(),
      y: event.client_y(),
      pressed_buttons: event.buttons(),
      event_type: MouseEventType::Moved,
    });
    app_shared.borrow_mut().process_input_event(input_event);
    if LOG_INPUT_EVENTS {
      console_log_with_div!(
        "mousemove: {}, {}, {}",
        event.client_x(),
        event.client_y(),
        event.buttons(),
      );
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback mousemove failed");
  closure.forget();
}

fn register_mouse_up(canvas: &web_sys::HtmlCanvasElement, app_shared: Shared<dyn AppTrait>) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    event.prevent_default();
    let input_event = InputEvent::Mouse(MouseEvent {
      x: event.client_x(),
      y: event.client_y(),
      pressed_buttons: event.buttons(),
      event_type: MouseEventType::Released,
    });
    app_shared.borrow_mut().process_input_event(input_event);
    if LOG_INPUT_EVENTS {
      console_log_with_div!(
        "mouseup: {}, {}, {}",
        event.client_x(),
        event.client_y(),
        event.buttons()
      );
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback mouseup failed");
  closure.forget();
}

fn register_context_menu(canvas: &web_sys::HtmlCanvasElement) {
  let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    event.prevent_default();
    if LOG_INPUT_EVENTS {
      console_log_with_div!("contextmenu event");
    }
  }) as Box<dyn FnMut(_)>);
  canvas
    .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())
    .expect("canvas.add_event_listener_with_callback contextmenu failed");
  closure.forget();
}
