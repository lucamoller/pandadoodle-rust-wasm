#![feature(coerce_unsized)]
#![feature(trait_alias)]
#![feature(unsize)]
#[macro_use]
mod logging;
mod audio_manager;
mod context;
mod draw_depths;
mod engine;
mod game;
mod game_ui;
mod texture_manager;

use crate::engine::*;
pub use context::*;
use game::game_mode::GameMode;
use game_ui::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

// Binding some types to the applications's Context implementation.
pub type EffectManager = engine::EffectManagerGeneric<Context>;
pub type ChainedEffect = ChainedEffectGeneric<Context>;
pub type Effect<T> = engine::EffectGeneric<Context, T>;
pub type SetEffect = SetEffectGeneric<Context>;

pub type EntityBase = engine::EntityBaseGeneric<Context>;
pub type EntityManager<T> = engine::EntityManagerGeneric<Context, T>;

pub type UiButton = UiButtonGeneric<Context>;
pub type UiContainer = UiContainerGeneric<Context>;
pub type UiElement = UiElementGeneric<Context>;
pub type UiManager = UiManagerGeneric<Context>;
pub type UiManagerEvent = UiManagerEventGeneric<Context>;
pub type UiPivot = UiPivotGeneric<Context>;
pub type UiSlider = UiSliderGeneric<Context>;
pub type UiSprite = UiSpriteGeneric<Context>;
pub type UiText = UiTextGeneric<Context>;
pub type UiTouchable = UiTouchableGeneric<Context>;

struct App {
  context: Context,
  ui_manager: GameUiManager,
  fps_tracker: FpsTracker,
}

impl App {
  fn process_artificial_input_events(&mut self) {
    while let Some(input_event) = self.context.artificial_input_events.consume_event() {
      self.process_input_event(input_event);
    }
  }

  fn draw(&mut self) {
    self.ui_manager.draw(&mut self.context);
  }
}

fn run_loop(app_shared: Shared<App>) {
  let closure = Closure::once(Box::new(move |timestamp: f64| {
    {
      let mut app_mut_ref = app_shared.borrow_mut();
      let mut app = &mut *app_mut_ref;
      app.context.update_timestamp(timestamp as F1);
      app.context.check_screen_updated();
      app.fps_tracker.update_fps(&mut app.context);
      app.process_artificial_input_events();
      app.ui_manager.update(&mut app.context);
      app.context.draw_cycle += 1;
      app.draw();
      app.context.draw_manager.execute_draws();
    }
    run_loop(app_shared);
  }) as Box<dyn FnOnce(f64)>);

  web_sys::window()
    .unwrap()
    .request_animation_frame(closure.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
  closure.forget();
}

fn register_on_error_listener(window: &web_sys::Window) {
  let closure = Closure::wrap(Box::new(move |msg: String| {
    console_log_with_div!("on_error!\nmsg: {}", msg,);
    if msg.to_ascii_lowercase().contains("script error") {
      web_sys::window().unwrap().alert_with_message(
          "Oops, it seems something went wrong! If you're using an uncommon browser, you can try using Chrome.")
          .expect("window.alert_with_message failed");
    }
  }) as Box<dyn Fn(_)>);
  window.set_onerror(Some(closure.as_ref().unchecked_ref()));
  closure.forget();
}

fn register_on_visibility_change_listener(window: &web_sys::Window, app_shared: Shared<App>) {
  let closure = Closure::wrap(Box::new(move || {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mut app_mut_ref = app_shared.borrow_mut();
    let app = &mut *app_mut_ref;
    if document.hidden() {
      app.context.audio_player.stop_song();
    } else {
      app.context.audio_player.resume_song();
    }
  }) as Box<dyn Fn()>);
  let document = window.document().unwrap();
  document.set_onvisibilitychange(Some(closure.as_ref().unchecked_ref()));
  closure.forget();
}

fn register_input_listeners(window: &web_sys::Window, app_shared: Shared<App>) {
  let context = &mut app_shared.borrow_mut().context;
  InputManager::register(
    app_shared.clone(),
    window,
    &context.canvas,
    context.history.clone(),
  );
}

impl AppTrait for App {
  fn process_input_event(&mut self, input_event: InputEvent) {
    if let Some(screen_touch) = ScreenTouch::from_event(&input_event) {
      if screen_touch.id != 0 {
        return;
      }
      self
        .ui_manager
        .process_touch(&mut self.context, &screen_touch);
    }
    if let InputEvent::BackButton = input_event {
      let process_back_button_result = self.ui_manager.process_back_button(&mut self.context);
      if process_back_button_result == InputState::Consumed {
        self
          .context
          .history
          .push_state(&JsValue::from_str("gameRunning"), "Game Running")
          .expect("history.push_state failed");
      } else {
        self.context.history.back().expect("history.back failed");
      }
    }
  }

  fn check_fullscreen(&mut self) {
    if self.context.running_as_pwa && !self.context.get_platform_manager().ios() {
      self.context.fullscreen();
    }
  }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  let window = Rc::new(web_sys::window().unwrap());
  let context = Context::new(window.clone());
  let ui_manager = GameUiManager::new(&context);
  let app_shared = Shared::new(App {
    context: context,
    ui_manager: ui_manager,
    fps_tracker: FpsTracker::new(),
  });

  register_on_error_listener(&window);
  register_on_visibility_change_listener(&window, app_shared.clone());
  register_input_listeners(&window, app_shared.clone());

  console::log_1(&JsValue::from_str("Starting run_loop(app_shared)!"));
  run_loop(app_shared);
  Ok(())
}
