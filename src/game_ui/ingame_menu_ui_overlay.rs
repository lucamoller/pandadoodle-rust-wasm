use crate::engine::*;
use crate::game_ui::IngameUiEvent;
use crate::*;

pub struct IngameMenuUiOverlay {
  container: Rc<UiContainer>,
  btn_music: Rc<UiButton>,
  btn_sound: Rc<UiButton>,
  btn_show_fps_text: Rc<UiText>,
}

impl IngameMenuUiOverlay {
  pub fn new(
    context: &Context,
    events: Rc<EventManager<IngameUiEvent>>,
  ) -> Rc<IngameMenuUiOverlay> {
    let container = UiContainer::new();

    let ctn_menu = UiContainer::new();
    ctn_menu.set_position_y(0.54);
    container.add_child(ctn_menu.clone());

    let btn_menu_class = UiElementClass {
      size: Some(F2 { x: 0.2, y: 0.2 }),
      ..Default::default()
    };

    let btn_continue = UiButton::new(
      context.texture_manager.gui_btn_wood_play.clone(),
      context.texture_manager.gui_btn_wood_play_pressed.clone(),
    );
    btn_continue.set_class(&btn_menu_class);
    btn_continue.set_position_x(0.5);
    btn_continue.set_sound_on_released(context.audio_manager.click.clone());
    btn_continue.set_event_on_released(events.clone(), IngameUiEvent::OverlayResumeGame);
    btn_continue
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context
          .audio_player
          .play_sound(&context.audio_manager.click);
        context
          .get_ui_manager_events()
          .add_event(UiManagerEvent::HideUiOverlay);
      }));
    ctn_menu.add_child(btn_continue.clone());

    let btn_skip = UiButton::new(
      context.texture_manager.gui_btn_wood_skip.clone(),
      context.texture_manager.gui_btn_wood_skip_pressed.clone(),
    );
    btn_skip.set_class(&btn_menu_class);
    btn_skip.set_position_x(0.7);
    btn_skip.set_event_on_released(events.clone(), IngameUiEvent::OverlayNext);
    btn_skip
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context
          .audio_player
          .play_sound(&context.audio_manager.click);
        context
          .get_ui_manager_events()
          .add_event(UiManagerEvent::HideUiOverlay);
      }));
    ctn_menu.add_child(btn_skip.clone());

    let btn_select_level = UiButton::new(
      context.texture_manager.gui_btn_wood_menu.clone(),
      context.texture_manager.gui_btn_wood_menu_pressed.clone(),
    );
    btn_select_level.set_class(&btn_menu_class);
    btn_select_level.set_position_x(0.3);
    btn_select_level
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context
          .audio_player
          .play_sound(&context.audio_manager.click);
        context
          .get_ui_manager_events()
          .add_event(UiManagerEvent::HideUiOverlay);
        context
          .get_ui_manager_events()
          .add_event(UiManagerEvent::CloseCurrentPage);
      }));
    ctn_menu.add_child(btn_select_level.clone());

    let ctn_sounds = UiContainer::new();
    ctn_sounds.set_position_y(0.8);
    container.add_child(ctn_sounds.clone());

    let btn_sound_class = UiElementClass {
      size: Some(F2 { x: 0.16, y: 0.16 }),
      ..Default::default()
    };
    let btn_music = UiButton::new(
      context.texture_manager.gui_btn_music.clone(),
      context.texture_manager.gui_btn_music.clone(),
    );
    btn_music.set_class(&btn_sound_class);
    btn_music.set_position_x(0.36);
    btn_music
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context.audio_player.toggle_mute_song();
      }));
    ctn_sounds.add_child(btn_music.clone());
    let btn_sound = UiButton::new(
      context.texture_manager.gui_btn_sound.clone(),
      context.texture_manager.gui_btn_sound.clone(),
    );
    btn_sound.set_class(&btn_sound_class);
    btn_sound.set_position_x(0.64);
    btn_sound
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context.audio_player.toggle_mute_sound();
      }));
    ctn_sounds.add_child(btn_sound.clone());

    let game_mode = context.game_mode.borrow().clone().unwrap();

    let text_best_score = UiText::new();
    text_best_score.use_text_cache();
    text_best_score.set_font_size(50.0 / 480.0);
    text_best_score.set_position(F2 {
      x: 0.5,
      y: 80.0 / 480.0,
    });
    text_best_score.set_alignment(TextAlignment::Center);
    let score = context
      .achievments_manager
      .get_score(game_mode.book.get().number(), game_mode.stage_number.get());
    if score > 0 {
      text_best_score.set_text(format!("Best Score: {}", score));
    }
    text_best_score.set_border(true);
    ctn_sounds.add_child(text_best_score.clone());

    let btn_fullscreen = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_fullscreen.set_size_x(240.0 / 480.0);
    btn_fullscreen.set_size_y(70.0 / 480.0);
    btn_fullscreen.set_position_x(0.5);
    btn_fullscreen.set_position_y(1.09);
    btn_fullscreen.set_event_on_released(events.clone(), IngameUiEvent::OverlayFullscreen);
    let btn_fullscreen_text = UiText::new();
    btn_fullscreen_text.set_text(String::from("Fullscreen"));
    btn_fullscreen_text.use_text_cache();
    btn_fullscreen_text.set_font_size(60.0 / 480.0);
    btn_fullscreen_text.set_alignment(TextAlignment::Center);
    btn_fullscreen_text.set_border(true);
    btn_fullscreen
      .container
      .add_child(btn_fullscreen_text.clone());
    container.add_child(btn_fullscreen.clone());

    let btn_show_fps = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_show_fps.set_size_x(240.0 / 480.0);
    btn_show_fps.set_size_y(70.0 / 480.0);
    btn_show_fps.set_position_x(0.5);
    btn_show_fps.set_position_y(1.24);
    btn_show_fps.set_event_on_released(events.clone(), IngameUiEvent::OverlayShowFps);
    let btn_show_fps_text = UiText::new();
    btn_show_fps_text.set_text(String::from(format!(
      "Show fps: {}",
      if context.show_fps() { "on" } else { "off" }
    )));
    btn_show_fps_text.use_text_cache();
    btn_show_fps_text.set_font_size(60.0 / 480.0);
    btn_show_fps_text.set_alignment(TextAlignment::Center);
    btn_show_fps_text.set_border(true);
    btn_show_fps.container.add_child(btn_show_fps_text.clone());
    container.add_child(btn_show_fps.clone());

    let result = Rc::new(IngameMenuUiOverlay {
      container: container,
      btn_music: btn_music,
      btn_sound: btn_sound,
      btn_show_fps_text: btn_show_fps_text,
    });
    return result;
  }
}

impl UiOverlayTrait<Context> for IngameMenuUiOverlay {}

impl UiRootTrait<Context> for IngameMenuUiOverlay {
  fn on_press_back(&self, context: &mut Context) -> InputState {
    // console_log_with_div!("IngameMenuUiOverlay on_press_back");
    context
      .get_ui_manager_events()
      .add_event(UiManagerEvent::HideUiOverlay);
    return InputState::Consumed;
  }
}

impl EffectManagerTrait<Context> for IngameMenuUiOverlay {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl UiElementTrait<Context> for IngameMenuUiOverlay {
  fn get_ui_element(&self) -> &UiElement {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    self.container.update(context);
    let btn_music_texture = match context.audio_player.settings.song_muted {
      true => &context.texture_manager.gui_btn_music_off,
      false => &context.texture_manager.gui_btn_music,
    };
    self.btn_music.set_texture(btn_music_texture.clone());
    self
      .btn_music
      .set_texture_pressed(btn_music_texture.clone());

    let btn_sound_texture = match context.audio_player.settings.sound_muted {
      true => &context.texture_manager.gui_btn_sound_off,
      false => &context.texture_manager.gui_btn_sound,
    };
    self.btn_sound.set_texture(btn_sound_texture.clone());
    self
      .btn_sound
      .set_texture_pressed(btn_sound_texture.clone());

    self.btn_show_fps_text.set_text(String::from(format!(
      "Show fps: {}",
      if context.show_fps() { "on" } else { "off" }
    )));
  }

  fn draw(&self, context: &mut Context) {
    self.container.draw(context);
  }

  fn get_touched_element(
    &self,
    context: &mut Context,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<Context>>> {
    return self.container.get_touched_element(context, ui_touch);
  }
}
