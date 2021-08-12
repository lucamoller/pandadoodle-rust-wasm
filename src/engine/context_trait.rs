use crate::engine::*;

pub trait ContextTrait
where
  Self: 'static,
{
  fn get_dt(&self) -> &F1;

  fn get_latest_timestamp(&self) -> &F1;

  fn get_draw_cycle(&self) -> &u32;

  fn draw_ui_viewport(&mut self, args: DrawImageArgs);

  fn draw_string_ui_viewport(&mut self, args: DrawStringArgs);

  fn get_draw_manager(&mut self) -> &mut DrawManager;

  fn get_screen_viewport(&self) -> &Rc<Viewport>;

  fn get_ui_viewport(&self) -> &Rc<Viewport>;

  fn get_game_viewport(&self) -> &Rc<Viewport>;

  fn get_screen_size(&self) -> &F2;

  fn get_ui_viewport_screen_center(&self) -> F2 {
    return self
      .get_ui_viewport()
      .screen_to_viewport(&(self.get_screen_size() * 0.5));
  }

  fn get_ui_viewport_screen_size(&self) -> F2 {
    return self
      .get_ui_viewport()
      .screen_to_viewport_ratio(self.get_screen_size());
  }

  fn get_pixel_texture(&self) -> Rc<Texture>;

  fn get_front_board_depth(&self) -> F1;

  fn get_ui_manager_events(&self) -> &Rc<EventManager<UiManagerEventGeneric<Self>>>;

  fn play_sound(&mut self, sound: &Rc<Audio>);

  fn window(&self) -> &web_sys::Window;

  fn get_platform_manager(&self) -> &PlatformManager;

  fn alert(&self, message: &str) {
    self
      .window()
      .alert_with_message(message)
      .expect("window.alert_with_message failed");
  }

  fn fullscreen(&self) {
    if self.get_platform_manager().ios() {
      self.alert(
        "In order to run fullscreen, iOS requires the web app to be added to the home screen: \n
- Tap the share icon (the square with an arrow pointing up out of it) at the bottom of the screen.
- Scroll down and tap \"Add to Home Screen\".",
      );
      return;
    }
    // console_log_with_div!("going fullscreen");
    self
      .window()
      .document()
      .unwrap()
      .document_element()
      .unwrap()
      .request_fullscreen()
      .expect("request_fullscreen failed");
  }

  fn local_storage(&self) -> &web_sys::Storage;

  fn show_fps(&self) -> bool;
  fn toggle_show_fps(&mut self);
}
