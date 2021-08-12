use crate::context::Context;
use crate::context::UiEvent;
use crate::game_ui::*;
use crate::*;

#[derive(Clone, Copy)]
enum LandingPageState {
  Initial,
  WaitingLangingLoaders,
  Ready,
  WaitingLoaders,
  SwitchingToMainMenu,
}

#[derive(Clone, Copy)]
enum LandingPageEvent {
  StartLoad,
  ToggleSound,
  Fullscreen,
}

pub struct LandingPageUiRoot {
  container: Rc<UiContainer>,
  state: Cell<LandingPageState>,
  events: Rc<EventManager<LandingPageEvent>>,
  ui_elements: RefCell<Option<UiElements>>,
  sound_on: Shared<bool>,
}

struct UiElements {
  btn_start_load: Rc<UiButton>,
  btn_start_load_text: Rc<UiText>,
  btn_sound_text: Rc<UiText>,
}

impl LandingPageUiRoot {
  pub fn new(context: &Context) -> LandingPageUiRoot {
    let container = UiContainer::new();
    container.set_visible(false);
    let events = EventManager::new();

    let sound_on =
      !context.audio_player.settings.song_muted || !context.audio_player.settings.sound_muted;

    return LandingPageUiRoot {
      container: container,
      state: Cell::new(LandingPageState::Initial),
      events: events,
      ui_elements: RefCell::new(None),
      sound_on: Shared::new(sound_on),
    };
  }

  pub fn on_landing_loaders_done(&self, context: &mut Context) {
    let loading_div = context
      .window
      .document()
      .unwrap()
      .get_element_by_id("loading-div")
      .unwrap()
      .dyn_into::<web_sys::HtmlDivElement>()
      .expect("dyn_into::<web_sys::HtmlDivElement> failed");
    loading_div
      .style()
      .set_property("display", "none")
      .expect("loading_div.style().set_property failed");
    self.container.set_visible(true);

    let sprite_icon = Rc::new(UiSprite::new(context.texture_manager.icon_original.clone()));
    sprite_icon.set_size_from_width(0.7);
    sprite_icon.set_position(F2 { x: 0.5, y: 0.4 });
    sprite_icon.set_depth(1.0);
    self.container.add_child(sprite_icon);

    let game_title_text = UiText::new();
    game_title_text.set_text(String::from("Panda Doodle"));
    game_title_text.set_font_size(75.0 / 480.0);
    game_title_text.set_position(F2 { x: 0.15, y: 0.83 });
    game_title_text.set_alignment(TextAlignment::Left);
    game_title_text.set_color(DrawColor { r: 0, g: 0, b: 0 });
    game_title_text.set_border(false);
    self.container.add_child(game_title_text.clone());

    let game_title_sub_text = UiText::new();
    game_title_sub_text.set_text(String::from("Drawing Puzzle"));
    game_title_sub_text.set_font_size(40.0 / 480.0);
    game_title_sub_text.set_position(F2 { x: 0.85, y: 0.9 });
    game_title_sub_text.set_alignment(TextAlignment::Right);
    game_title_sub_text.set_color(DrawColor { r: 0, g: 0, b: 0 });
    game_title_sub_text.set_border(false);
    self.container.add_child(game_title_sub_text.clone());

    let btn_sound = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_sound.set_size_x(170.0 / 480.0);
    btn_sound.set_size_y(60.0 / 480.0);
    btn_sound.set_position_x(0.32);
    btn_sound.set_position_y(1.1 /*50.0 / 480.0*/);
    btn_sound.set_event_on_released(self.events.clone(), LandingPageEvent::ToggleSound);
    let btn_sound_text = UiText::new();
    btn_sound_text.set_text(String::from(format!(
      "Sound: {}",
      if self.sound_on.get() { "on" } else { "off" }
    )));
    btn_sound_text.set_font_size(50.0 / 480.0);
    btn_sound_text.set_alignment(TextAlignment::Center);
    btn_sound_text.set_border(true);
    btn_sound.container.add_child(btn_sound_text.clone());
    self.container.add_child(btn_sound.clone());

    let btn_fullscreen = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_fullscreen.set_size_x(170.0 / 480.0);
    btn_fullscreen.set_size_y(60.0 / 480.0);
    btn_fullscreen.set_position_x(1.0 - 0.32);
    btn_fullscreen.set_position_y(1.1 /*50.0 / 480.0*/);
    btn_fullscreen.set_event_on_released(self.events.clone(), LandingPageEvent::Fullscreen);
    let btn_fullscreen_text = UiText::new();
    btn_fullscreen_text.set_text(String::from("Fullscreen"));
    btn_fullscreen_text.set_font_size(50.0 / 480.0);
    btn_fullscreen_text.set_alignment(TextAlignment::Center);
    btn_fullscreen_text.set_border(true);
    btn_fullscreen
      .container
      .add_child(btn_fullscreen_text.clone());
    self.container.add_child(btn_fullscreen.clone());

    let btn_start_load = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_start_load.set_size_x(360.0 / 480.0);
    btn_start_load.set_size_y(120.0 / 480.0);
    btn_start_load.set_position_x(0.5);
    btn_start_load.set_position_y(1.3 /*50.0 / 480.0*/);
    btn_start_load.set_event_on_released(self.events.clone(), LandingPageEvent::StartLoad);

    let ios_audio_loading_started = Shared::new(false);
    let sound_on = self.sound_on.clone();
    btn_start_load
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        if context.get_platform_manager().ios() {
          if !ios_audio_loading_started.get() {
            ios_audio_loading_started.replace(true);
            if sound_on.get() {
              context.audio_manager.audio_loader.start_loading_ios();
            } else {
              context.audio_manager.audio_loader.start_loading();
            }
          }
        }
      }));

    let btn_start_load_text = UiText::new();
    btn_start_load_text.set_text(String::from("Load Game"));
    btn_start_load_text.set_font_size(75.0 / 480.0);
    btn_start_load_text.set_alignment(TextAlignment::Center);
    btn_start_load_text.set_border(true);
    btn_start_load
      .container
      .add_child(btn_start_load_text.clone());
    self.container.add_child(btn_start_load.clone());

    self.ui_elements.replace(Some(UiElements {
      btn_start_load: btn_start_load,
      btn_start_load_text: btn_start_load_text,
      btn_sound_text: btn_sound_text,
    }));
  }
}

impl EffectManagerTrait<Context> for LandingPageUiRoot {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl UiElementTrait<Context> for LandingPageUiRoot {
  fn get_ui_element(&self) -> &UiElement {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    match self.state.get() {
      LandingPageState::Initial => {
        context.texture_manager.landing_loader.start_loading();
        self.state.set(LandingPageState::WaitingLangingLoaders);
      }
      LandingPageState::WaitingLangingLoaders => {
        if context.texture_manager.landing_loader.all_loaded() {
          self.on_landing_loaders_done(context);
          self.state.set(LandingPageState::Ready);
        }
      }
      LandingPageState::Ready => {
        if context
          .statsig_bindings
          .statsig_gates
          .borrow()
          .auto_load_game
        {
          self.events.add_event(LandingPageEvent::StartLoad);
        }
      }
      LandingPageState::WaitingLoaders => {
        // // Wait for audio and textures
        // let audio_loaded =
        //   context.audio_manager.audio_loader.all_loaded() || context.get_platform_manager().ios();
        // let textures_loaded = context.texture_manager.loader.all_loaded();
        // if audio_loaded && textures_loaded {
        //   context.ui_events.add_event(UiEvent::LoadMainMenu);
        //   self.state.set(LandingPageState::SwitchingToMainMenu);
        // } else {
        //   let audio_loading_progress = context.audio_manager.audio_loader.get_progress();
        //   let texture_loading_progress = context.texture_manager.loader.get_progress();
        //   let pct = (100 * (audio_loading_progress.loaded + texture_loading_progress.loaded))
        //     / (audio_loading_progress.total_audios + texture_loading_progress.total_textures);
        //   self
        //     .ui_elements
        //     .borrow()
        //     .as_ref()
        //     .unwrap()
        //     .btn_start_load_text
        //     .set_text(String::from(format!("Loading... {}%", pct)));
        // }

        // Wait only for textures
        let textures_loaded = context.texture_manager.loader.all_loaded();
        if textures_loaded {
          context.ui_events.add_event(UiEvent::LoadMainMenu);
          self.state.set(LandingPageState::SwitchingToMainMenu);
        } else {
          let texture_loading_progress = context.texture_manager.loader.get_progress();
          let pct =
            (100 * texture_loading_progress.loaded) / texture_loading_progress.total_textures;
          self
            .ui_elements
            .borrow()
            .as_ref()
            .unwrap()
            .btn_start_load_text
            .set_text(String::from(format!("Loading... {}%", pct)));
        }
      }
      LandingPageState::SwitchingToMainMenu => {}
    }

    while let Some(event) = self.events.consume_event() {
      match event {
        LandingPageEvent::StartLoad => {
          // panic!("crash on purpose");
          context.texture_manager.loader.start_loading();
          if !context.platform_manager.ios() {
            context.audio_manager.audio_loader.start_loading();
          }
          self
            .ui_elements
            .borrow()
            .as_ref()
            .unwrap()
            .btn_start_load
            .set_active(false);
          self
            .ui_elements
            .borrow()
            .as_ref()
            .unwrap()
            .btn_start_load_text
            .set_color(DrawColor {
              r: 150,
              g: 150,
              b: 150,
            });
          self.state.set(LandingPageState::WaitingLoaders);

          if !self.sound_on.get() {
            if !context.audio_player.settings.song_muted {
              context.audio_player.toggle_mute_song();
            }
            if !context.audio_player.settings.sound_muted {
              context.audio_player.toggle_mute_sound();
            }
          } else {
            if context.audio_player.settings.song_muted && context.audio_player.settings.sound_muted
            {
              context.audio_player.toggle_mute_song();
              context.audio_player.toggle_mute_sound();
            }
          }
        }
        LandingPageEvent::ToggleSound => {
          self.sound_on.replace(!self.sound_on.get());
          self
            .ui_elements
            .borrow()
            .as_ref()
            .unwrap()
            .btn_sound_text
            .set_text(String::from(format!(
              "Sound: {}",
              if self.sound_on.get() { "on" } else { "off" }
            )));
        }
        LandingPageEvent::Fullscreen => {
          if context.running_as_pwa {
            context
              .alert("Game already running in fullscreen (progressive web app / standalone) mode")
          } else {
            context.fullscreen();
          }
        }
      }
    }

    self.container.update(context);
    if self.sound_on.get() {
    } else {
    }
  }

  fn draw(&self, context: &mut Context) {
    match self.state.get() {
      LandingPageState::Initial | LandingPageState::WaitingLangingLoaders => {}
      _ => {
        BackgroundBorders::draw(context);
      }
    }
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

impl UiRootTrait<Context> for LandingPageUiRoot {
  fn on_back_to(&self, _context: &mut Context) {}
  fn on_back_from(&self, _context: &mut Context) {}
  fn on_navigate_to(&self, _context: &mut Context) {}
  fn on_navigate_from(&self, _context: &mut Context) {}
  fn on_reactivate(&self, _context: &mut Context) {}
}
