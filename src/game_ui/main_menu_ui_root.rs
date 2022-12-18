use crate::context::Context;
use crate::context::UiEvent;
use crate::game_ui::*;
use crate::*;

#[derive(Clone, Copy)]
enum MainMenuEvent {
  Play,
  Options,
  OptionsBack,
  OptionsVibrate,
  OptionsReset,
}

pub struct MainMenuUiRoot {
  pub container: Rc<UiContainer>,
  effect_manager: EffectManager,
  glow_bar_top: Shared<GlowBar>,
  glow_bar_bottom: Shared<GlowBar>,
  ctn_menu: Rc<UiContainer>,
  ctn_options: Rc<UiContainer>,
  btn_music: Rc<UiButton>,
  btn_sound: Rc<UiButton>,
  btn_vibrate_text: Rc<UiText>,
  effect_show_options: Rc<Effect<SetEffect>>,
  effect_hide_options: Rc<Effect<SetEffect>>,
  events: Rc<EventManager<MainMenuEvent>>,
}

impl MainMenuUiRoot {
  pub fn new(context: &Context) -> MainMenuUiRoot {
    let screen_center = &context.ui_viewport.screen_center;
    let screen_bottom_right = &context.ui_viewport.screen_bottom_right_corner;
    let sprite_scroll_position = screen_center;

    let container = UiContainer::new();
    let effect_manager = EffectManager::new();
    let events = EventManager::new();

    let sprite_scroll = Rc::new(UiSprite::new(context.texture_manager.gui_scroll.clone()));
    sprite_scroll.set_size_from_width(1.0);
    sprite_scroll.set_position(*sprite_scroll_position);
    sprite_scroll.set_depth(1.0);
    sprite_scroll.set_subpixel_precision(true);
    container.add_child(sprite_scroll);

    let ctn_menu = UiContainer::new();
    ctn_menu.set_position_y(screen_center.y);
    ctn_menu.set_depth(-5.0);
    container.add_child(ctn_menu.clone());

    let text_pandadoodle = UiText::new();
    text_pandadoodle.set_text(String::from("Panda Doodle"));
    text_pandadoodle.use_text_cache();
    text_pandadoodle.set_font_size(110.0 / 480.0);
    text_pandadoodle.set_alignment(TextAlignment::Center);
    text_pandadoodle.set_position_x(0.5);
    text_pandadoodle.set_position_y(-90.0 / 480.0);
    text_pandadoodle.set_border(true);
    ctn_menu.add_child(text_pandadoodle);

    let btn_play = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_play.set_size_x(240.0 / 480.0);
    btn_play.set_size_y(82.0 / 480.0);
    btn_play.set_position_x(0.5);
    btn_play.set_position_y(50.0 / 480.0);
    btn_play.set_event_on_released(events.clone(), MainMenuEvent::Play);
    btn_play.set_sound_on_released(context.audio_manager.click.clone());
    let btn_play_text = UiText::new();
    btn_play_text.set_text(String::from("Play"));
    btn_play_text.use_text_cache();
    btn_play_text.set_font_size(75.0 / 480.0);
    btn_play_text.set_alignment(TextAlignment::Center);
    btn_play_text.set_border(true);
    btn_play.container.add_child(btn_play_text);
    ctn_menu.add_child(btn_play.clone());

    let btn_options = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_options.set_size_x(240.0 / 480.0);
    btn_options.set_size_y(82.0 / 480.0);
    btn_options.set_position_x(0.5);
    btn_options.set_position_y(150.0 / 480.0);
    btn_options.set_event_on_released(events.clone(), MainMenuEvent::Options);
    btn_options.set_sound_on_released(context.audio_manager.click.clone());
    let btn_options_text = UiText::new();
    btn_options_text.set_text(String::from("Options"));
    btn_options_text.use_text_cache();
    btn_options_text.set_font_size(75.0 / 480.0);
    btn_options_text.set_alignment(TextAlignment::Center);
    btn_options_text.set_border(true);
    btn_options.container.add_child(btn_options_text);
    ctn_menu.add_child(btn_options.clone());

    let ctn_options = UiContainer::new();
    ctn_options.set_position_y(screen_center.y - 350.0 / 480.0);
    ctn_options.set_depth(-5.0);
    ctn_options.set_active(false);
    ctn_options.set_opacity(0.0);
    container.add_child(ctn_options.clone());

    let btn_sound = UiButton::new(
      context.texture_manager.gui_btn_sound.clone(),
      context.texture_manager.gui_btn_sound.clone(),
    );
    btn_sound.set_size(F2 {
      x: 90.0 / 480.0,
      y: 90.0 / 480.0,
    });
    btn_sound.set_position(F2 {
      x: 90.0 / 480.0,
      y: 240.0 / 480.0,
    });
    {
      btn_sound
        .on_released_event
        .add(Box::new(move |context, _ui_touch| {
          context.audio_player.toggle_mute_sound();
        }));
    }
    ctn_options.add_child(btn_sound.clone());

    let btn_music = UiButton::new(
      context.texture_manager.gui_btn_music.clone(),
      context.texture_manager.gui_btn_music.clone(),
    );
    btn_music.set_size(F2 {
      x: 90.0 / 480.0,
      y: 90.0 / 480.0,
    });
    btn_music.set_position(F2 {
      x: 90.0 / 480.0,
      y: 350.0 / 480.0,
    });
    {
      btn_music
        .on_released_event
        .add(Box::new(move |context, _ui_touch| {
          context.audio_player.toggle_mute_song();
        }));
    }
    ctn_options.add_child(btn_music.clone());

    let sld_sound_bar = Rc::new(UiSprite::new(context.texture_manager.gui_volume.clone()));
    sld_sound_bar.set_size(F2 {
      x: 250.0 / 480.0,
      y: 15.0 / 480.0,
    });
    sld_sound_bar.set_color(DrawColor::new(&0, &200, &50));
    sld_sound_bar.set_depth(-1.0);
    let sld_sound_cursor = Rc::new(UiSprite::new(context.texture_manager.gui_cursor.clone()));
    sld_sound_cursor.set_size_from_width(30.0 / 480.0);
    sld_sound_cursor.set_depth(-2.0);
    let sld_sound = UiSlider::new(sld_sound_bar, sld_sound_cursor);
    sld_sound.set_position(F2 {
      x: 300.0 / 480.0,
      y: 240.0 / 480.0,
    });
    sld_sound
      .value
      .replace(context.audio_player.get_sound_volume());
    sld_sound
      .on_changed_event
      .add(Box::new(move |context, value, new_value| {
        if context.platform_manager.ios() {
          context.alert("iOS doesn't support audio element volume on browsers.");
          return;
        }
        value.replace(*new_value);
        context.audio_player.set_sound_volume(*new_value);
      }));
    ctn_options.add_child(sld_sound.clone());

    let sld_music_bar = Rc::new(UiSprite::new(context.texture_manager.gui_volume.clone()));
    sld_music_bar.set_size(F2 {
      x: 250.0 / 480.0,
      y: 15.0 / 480.0,
    });
    sld_music_bar.set_color(DrawColor::new(&0, &200, &50));
    sld_music_bar.set_depth(-1.0);
    let sld_music_cursor = Rc::new(UiSprite::new(context.texture_manager.gui_cursor.clone()));
    sld_music_cursor.set_size_from_width(30.0 / 480.0);
    sld_music_cursor.set_depth(-2.0);
    let sld_music = UiSlider::new(sld_music_bar, sld_music_cursor);
    sld_music.set_position(F2 {
      x: 300.0 / 480.0,
      y: 350.0 / 480.0,
    });
    sld_music
      .value
      .replace(context.audio_player.get_song_volume());
    sld_music
      .on_changed_event
      .add(Box::new(move |context, value, new_value| {
        if context.platform_manager.ios() {
          context.alert("iOS doesn't support audio element volume on browsers.");
          return;
        }
        value.replace(*new_value);
        context.audio_player.set_song_volume(*new_value);
      }));
    ctn_options.add_child(sld_music.clone());

    let btn_vibrate = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_vibrate.set_size(F2 {
      x: 360.0 / 480.0,
      y: 60.0 / 480.0,
    });
    btn_vibrate.set_position(F2 {
      x: 240.0 / 480.0,
      y: 446.0 / 480.0,
    });
    btn_vibrate.set_event_on_released(events.clone(), MainMenuEvent::OptionsVibrate);
    btn_vibrate.set_sound_on_released(context.audio_manager.click.clone());
    let btn_vibrate_text = UiText::new();
    btn_vibrate_text.set_text(format!(
      "Vibrate: {}",
      context.vibration_manager.get_vibration_level()
    ));
    btn_vibrate_text.set_font_size(50.0 / 480.0);
    btn_vibrate_text.set_alignment(TextAlignment::Center);
    btn_vibrate_text.set_border(true);
    btn_vibrate.container.add_child(btn_vibrate_text.clone());
    ctn_options.add_child(btn_vibrate.clone());

    let btn_reset = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_reset.set_size(F2 {
      x: 360.0 / 480.0,
      y: 60.0 / 480.0,
    });
    btn_reset.set_position(F2 {
      x: 240.0 / 480.0,
      y: 524.0 / 480.0,
    });
    btn_reset.set_event_on_released(events.clone(), MainMenuEvent::OptionsReset);
    btn_reset.set_sound_on_released(context.audio_manager.click.clone());
    let btn_reset_text = UiText::new();
    btn_reset_text.set_text(String::from("Reset Progress"));
    btn_reset_text.set_font_size(50.0 / 480.0);
    btn_reset_text.set_alignment(TextAlignment::Center);
    btn_reset_text.set_border(true);
    btn_reset.container.add_child(btn_reset_text);
    ctn_options.add_child(btn_reset.clone());

    let btn_back = UiButton::new(
      context.texture_manager.gui_btn_back.clone(),
      context.texture_manager.gui_btn_back_pressed.clone(),
    );
    btn_back.set_size_from_x(60.0 / 480.0);
    btn_back.set_position(F2 {
      x: 40.0 / 480.0,
      y: screen_bottom_right.y - 60.0 / 480.0 - ctn_options.get_position().borrow().y,
    });
    btn_back.set_event_on_released(events.clone(), MainMenuEvent::OptionsBack);
    btn_back.set_sound_on_released(context.audio_manager.click.clone());
    ctn_options.add_child(btn_back.clone());

    let effect_show_options = Effect::new_within_effect_manager(SetEffect::new(), &effect_manager);
    Effect::new_within_set_effect(
      VectorAffectorF1::new(ctn_menu.get_opacity()).set_start_and_end(1.0, 0.0, 300.0),
      0.0,
      &effect_show_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF1::new(ctn_options.get_opacity()).set_start_and_end(0.0, 1.0, 300.0),
      600.0,
      &effect_show_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(btn_play.get_position()).set_start_and_end(
        F2 {
          x: 0.5,
          y: btn_play.get_position().borrow().y,
        },
        F2 {
          x: 1.25,
          y: btn_play.get_position().borrow().y,
        },
        1000.0,
      ),
      0.0,
      &effect_show_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(btn_options.get_position()).set_start_and_end(
        F2 {
          x: 0.5,
          y: btn_options.get_position().borrow().y,
        },
        F2 {
          x: 1.25,
          y: btn_options.get_position().borrow().y,
        },
        1000.0,
      ),
      0.0,
      &effect_show_options,
    );

    let effect_hide_options = Effect::new_within_effect_manager(SetEffect::new(), &effect_manager);
    Effect::new_within_set_effect(
      VectorAffectorF1::new(ctn_menu.get_opacity()).set_start_and_end(0.0, 1.0, 300.0),
      300.0,
      &effect_hide_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF1::new(ctn_options.get_opacity()).set_start_and_end(1.0, 0.0, 300.0),
      0.0,
      &effect_hide_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(btn_play.get_position())
        .set_start_and_end(
          F2 {
            x: 1.25,
            y: btn_play.get_position().borrow().y,
          },
          F2 {
            x: 0.5,
            y: btn_play.get_position().borrow().y,
          },
          1000.0,
        )
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_hide_options,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(btn_options.get_position())
        .set_start_and_end(
          F2 {
            x: 1.25,
            y: btn_options.get_position().borrow().y,
          },
          F2 {
            x: 0.5,
            y: btn_options.get_position().borrow().y,
          },
          1000.0,
        )
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_hide_options,
    );

    return MainMenuUiRoot {
      container: container,
      effect_manager,
      glow_bar_top: Shared::new(GlowBar::new(
        context.texture_manager.gui_glow_top.clone(),
        F2 {
          x: 0.5,
          y: sprite_scroll_position.y - 0.3820,
        },
        F2 {
          x: 1.0,
          y: 70.0 / 480.0,
        },
        context,
      )),
      glow_bar_bottom: Shared::new(GlowBar::new(
        context.texture_manager.gui_glow_bot.clone(),
        F2 {
          x: 0.5,
          y: sprite_scroll_position.y + 0.4945,
        },
        F2 {
          x: 1.0,
          y: 70.0 / 480.0,
        },
        context,
      )),
      ctn_menu: ctn_menu.clone(),
      ctn_options: ctn_options.clone(),
      btn_music: btn_music,
      btn_sound: btn_sound,
      btn_vibrate_text: btn_vibrate_text,
      effect_show_options: effect_show_options,
      effect_hide_options: effect_hide_options,
      events: events.clone(),
    };
  }
}

impl EffectManagerTrait<Context> for MainMenuUiRoot {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl UiElementTrait<Context> for MainMenuUiRoot {
  fn get_ui_element(&self) -> &UiElement {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    while let Some(event) = self.events.consume_event() {
      match event {
        MainMenuEvent::Play => {
          context.ui_events.add_event(UiEvent::LoadMenuChooseStage);
        }
        MainMenuEvent::Options => {
          self.effect_show_options.start();
          self.ctn_options.set_active(true);
          self.ctn_menu.set_active(false);
        }
        MainMenuEvent::OptionsBack => {
          self.effect_hide_options.start();
          self.ctn_options.set_active(false);
          self.ctn_menu.set_active(true);
        }
        MainMenuEvent::OptionsVibrate => {
          context.vibration_manager.switch_vibration_level(context);
          context.vibration_manager.vibrate();
          self.btn_vibrate_text.set_text(format!(
            "Vibrate: {}",
            context.vibration_manager.get_vibration_level()
          ));
        }
        MainMenuEvent::OptionsReset => {}
      }
    }

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

    self.glow_bar_top.borrow_mut().update(context.get_dt());
    self.glow_bar_bottom.borrow_mut().update(context.get_dt());
    self.container.update(context);
  }

  fn draw(&self, context: &mut Context) {
    BackgroundWood::draw(context);
    BackgroundBorders::draw(context);
    self.glow_bar_top.borrow_mut().draw(context);
    self.glow_bar_bottom.borrow_mut().draw(context);
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

impl UiRootTrait<Context> for MainMenuUiRoot {
  fn on_back_to(&self, context: &mut Context) {
    context.audio_player.play_song(&context.audio_manager.song2);
  }
  fn on_back_from(&self, _context: &mut Context) {}
  fn on_navigate_to(&self, context: &mut Context) {
    context.audio_player.play_song(&context.audio_manager.song2);
  }
  fn on_navigate_from(&self, _context: &mut Context) {}
  fn on_reactivate(&self, _context: &mut Context) {}
}
