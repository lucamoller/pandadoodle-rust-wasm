use crate::context::Context;
use crate::game::stages_data::*;
use crate::*;

pub struct IngameUiRoot {
  container: Rc<UiContainer>,
  effect_manager: EffectManager,
  events: Rc<EventManager<IngameUiEvent>>,
  game_root_entity_manager: EntityManager<GameMode>,
  game_mode: Rc<GameMode>,

  victory_ui: VictoryUi,
  book_intro: Rc<BookIntro>,
}

#[derive(Clone, Copy)]
pub enum IngameUiEvent {
  ButtonUndo,
  ButtonRestart,
  ButtonMenu,
  ButtonVictoryRestart,
  ButtonVictoryNext,
  ButtonVictoryMenu,
  EffectVictoryRestart,
  EffectVictoryNext,
  Victory(VictoryParams),
  OverlayResumeGame,
  OverlayNext,
  OverlayFullscreen,
  OverlayShowFps,
}

#[derive(Clone, Copy)]
pub struct VictoryParams {
  pub score: F1,
  pub stars: usize,
}

pub struct VictoryUi {
  container: Rc<UiContainer>,
  image_star1: Rc<UiSprite>,
  image_star2: Rc<UiSprite>,
  image_star3: Rc<UiSprite>,
  image_victory_title: Rc<UiSprite>,
  text_score_count: Rc<UiText>,
  text_best_score: Rc<UiText>,
  effect_star_animation: Rc<Effect<SetEffect>>,
  effect_show_best_score: Rc<Effect<VectorAffectorF1>>,
  effect_hide_victory_restart: Rc<Effect<VectorAffectorF1>>,
  effect_hide_victory_next: Rc<Effect<VectorAffectorF1>>,
  effect_twinkle_next: Rc<Effect<ChainedEffect>>,
  start_score_count: Shared<bool>,
  score: Cell<F1>,
  score_counted: Cell<F1>,
  best_score: Cell<bool>,
}

impl VictoryUi {
  fn new(
    context: &Context,
    events: Rc<EventManager<IngameUiEvent>>,
    effect_manager: &EffectManager,
  ) -> VictoryUi {
    let container = UiContainer::new();
    container.set_depth(-5.0);
    container.set_visible(false);

    let image_star1 = Rc::new(UiSprite::new(context.texture_manager.star_empty.clone()));
    image_star1.set_position(F2 {
      x: 0.5 - 0.3,
      y: 300.0 / 480.0,
    });
    image_star1.set_subpixel_precision(true);
    container.add_child(image_star1.clone());
    let image_star2 = Rc::new(UiSprite::new(context.texture_manager.star_empty.clone()));
    image_star2.set_position(F2 {
      x: 0.5,
      y: 300.0 / 480.0,
    });
    image_star2.set_subpixel_precision(true);
    container.add_child(image_star2.clone());
    let image_star3 = Rc::new(UiSprite::new(context.texture_manager.star_empty.clone()));
    image_star3.set_position(F2 {
      x: 0.5 + 0.3,
      y: 300.0 / 480.0,
    });
    image_star3.set_subpixel_precision(true);
    container.add_child(image_star3.clone());

    let image_victory_title = Rc::new(UiSprite::new(context.texture_manager.gui_awesome.clone()));
    image_victory_title.set_position(F2 {
      x: 0.5,
      y: 180.0 / 480.0,
    });
    image_victory_title.set_size_from_width(260.0 / 480.0);
    container.add_child(image_victory_title.clone());

    let text_score_count = UiText::new();
    text_score_count.use_text_cache();
    text_score_count.set_font_size(45.0 / 480.0);
    text_score_count.set_border(false);
    text_score_count.set_alignment(TextAlignment::Left);
    text_score_count.set_color(DrawColor { r: 0, g: 0, b: 0 });
    text_score_count.set_position(F2 {
      x: 120.0 / 480.0,
      y: 420.0 / 480.0,
    });
    container.add_child(text_score_count.clone());

    let text_best_score = UiText::new();
    text_best_score.use_text_cache();
    text_best_score.set_font_size(45.0 / 480.0);
    text_best_score.set_border(false);
    text_best_score.set_alignment(TextAlignment::Center);
    text_best_score.set_text(String::from("Best Score!!"));
    text_best_score.set_color(DrawColor { r: 0, g: 0, b: 0 });
    text_best_score.set_position(F2 {
      x: 240.0 / 480.0,
      y: 470.0 / 480.0,
    });
    text_best_score.set_opacity(0.0);
    container.add_child(text_best_score.clone());

    let container_buttons = UiContainer::new();
    container_buttons.set_position(F2 {
      x: 0.5,
      y: 580.0 / 480.0,
    });
    container.add_child(container_buttons.clone());

    let class_buttons_end = UiElementClass {
      size: Some(F2 {
        x: 80.0 / 480.0,
        y: 80.0 / 480.0,
      }),
      ..Default::default()
    };

    let button_menu = UiButton::new(
      context.texture_manager.gui_btn_wood_menu.clone(),
      context.texture_manager.gui_btn_wood_menu_pressed.clone(),
    );
    button_menu.set_class(&class_buttons_end);
    button_menu.set_position_x(-100.0 / 480.0);
    button_menu.set_event_on_released(events.clone(), IngameUiEvent::ButtonVictoryMenu);
    container_buttons.add_child(button_menu.clone());

    let button_restart = UiButton::new(
      context.texture_manager.gui_btn_wood_restart.clone(),
      context.texture_manager.gui_btn_wood_restart_pressed.clone(),
    );
    button_restart.set_class(&class_buttons_end);
    button_restart.set_position_x(0.0);
    button_restart.set_event_on_released(events.clone(), IngameUiEvent::ButtonVictoryRestart);
    container_buttons.add_child(button_restart.clone());

    let button_next = UiButton::new(
      context.texture_manager.gui_btn_wood_skip.clone(),
      context.texture_manager.gui_btn_wood_skip_pressed.clone(),
    );
    button_next.set_class(&class_buttons_end);
    button_next.set_position_x(100.0 / 480.0);
    button_next.set_depth(-4.0);
    button_next.set_event_on_released(events.clone(), IngameUiEvent::ButtonVictoryNext);
    container_buttons.add_child(button_next.clone());

    let image_next_bright = Rc::new(UiSprite::new(
      context.texture_manager.gui_img_twinkle_next.clone(),
    ));
    image_next_bright.set_class(&class_buttons_end);
    image_next_bright.set_position_x(100.0 / 480.0);
    image_next_bright.set_depth(-5.0);
    container_buttons.add_child(image_next_bright.clone());

    let effect_twinkle_next =
      Effect::new_within_effect_manager(ChainedEffect::new(), effect_manager);
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(image_next_bright.get_opacity()).set_start_and_end(0.0, 0.0, 2000.0),
      &effect_twinkle_next,
    );
    for _ in 0..10 {
      Effect::new_within_chained_effect(
        VectorAffectorF1::new(image_next_bright.get_opacity()).set_start_and_end(0.0, 1.0, 200.0),
        &effect_twinkle_next,
      );
      Effect::new_within_chained_effect(
        VectorAffectorF1::new(image_next_bright.get_opacity()).set_start_and_end(1.0, 1.0, 500.0),
        &effect_twinkle_next,
      );
      Effect::new_within_chained_effect(
        VectorAffectorF1::new(image_next_bright.get_opacity()).set_start_and_end(1.0, 0.0, 200.0),
        &effect_twinkle_next,
      );
      Effect::new_within_chained_effect(
        VectorAffectorF1::new(image_next_bright.get_opacity()).set_start_and_end(0.0, 0.0, 500.0),
        &effect_twinkle_next,
      );
    }

    let effect_star_animation = Effect::new_within_effect_manager(SetEffect::new(), effect_manager);
    let star_size = F2 {
      x: 120.0 / 480.0,
      y: 120.0 / 480.0,
    };

    Effect::new_within_set_effect(
      VectorAffectorF2::new(image_star1.size.clone())
        .set_end(star_size, 400.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      300.0,
      &effect_star_animation,
    );

    Effect::new_within_set_effect(
      VectorAffectorF2::new(image_star2.size.clone())
        .set_end(star_size, 400.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      500.0,
      &effect_star_animation,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(image_star3.size.clone())
        .set_end(star_size, 400.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      700.0,
      &effect_star_animation,
    );
    Effect::new_within_set_effect(
      VectorAffectorF1::new(image_victory_title.get_opacity().clone()).set_end(1.0, 500.0),
      0.0,
      &effect_star_animation,
    );
    Effect::new_within_set_effect(
      VectorAffectorF1::new(text_score_count.get_opacity().clone()).set_end(1.0, 400.0),
      1000.0,
      &effect_star_animation,
    );

    let start_score_count = Shared::new(false);
    {
      let start_score_count = start_score_count.clone();
      effect_star_animation
        .end_event
        .add(Box::new(move |_context| {
          start_score_count.replace(true);
        }));
    }

    let effect_show_best_score = Effect::new_within_effect_manager(
      VectorAffectorF1::new(text_best_score.get_opacity()).set_end(1.0, 1000.0),
      effect_manager,
    );

    let effect_hide_victory_restart = Effect::new_within_effect_manager(
      VectorAffectorF1::new(container.get_opacity()).set_end(0.0, 500.0),
      effect_manager,
    );
    effect_hide_victory_restart
      .add_event_on_end(events.clone(), IngameUiEvent::EffectVictoryRestart);

    let effect_hide_victory_next = Effect::new_within_effect_manager(
      VectorAffectorF1::new(container.get_opacity()).set_end(0.0, 500.0),
      effect_manager,
    );
    effect_hide_victory_next.add_event_on_end(events.clone(), IngameUiEvent::EffectVictoryNext);

    return VictoryUi {
      container: container,
      image_star1: image_star1,
      image_star2: image_star2,
      image_star3: image_star3,
      image_victory_title: image_victory_title,
      text_score_count: text_score_count,
      text_best_score: text_best_score,
      effect_star_animation: effect_star_animation,
      effect_show_best_score: effect_show_best_score,
      effect_hide_victory_restart: effect_hide_victory_restart,
      effect_hide_victory_next: effect_hide_victory_next,
      effect_twinkle_next,
      start_score_count: start_score_count,
      score: Cell::new(0.0),
      score_counted: Cell::new(0.0),
      best_score: Cell::new(false),
    };
  }
}

impl IngameUiRoot {
  pub fn new(context: &mut Context) -> Rc<IngameUiRoot> {
    let events = EventManager::new();
    let effect_manager = EffectManager::new();
    let container = UiContainer::new();

    let btn_top_class = UiElementClass {
      position_y: Some(38.0 / 480.0),
      size_y: Some(70.0 / 480.0),
      ..Default::default()
    };

    let btn_undo = UiButton::new(
      context.texture_manager.gui_btn_undo.clone(),
      context.texture_manager.gui_btn_undo_pressed.clone(),
    );
    btn_undo.set_class(&btn_top_class);
    btn_undo.set_size_x(70.0 / 480.0);
    btn_undo.set_position_x(300.0 / 480.0);
    btn_undo.set_event_on_released(events.clone(), IngameUiEvent::ButtonUndo);
    container.add_child(btn_undo);

    let btn_restart = UiButton::new(
      context.texture_manager.gui_btn_restart.clone(),
      context.texture_manager.gui_btn_restart_pressed.clone(),
    );
    btn_restart.set_class(&btn_top_class);
    btn_restart.set_size_x(70.0 / 480.0);
    btn_restart.set_position_x(375.0 / 480.0);
    btn_restart.set_event_on_released(events.clone(), IngameUiEvent::ButtonRestart);
    container.add_child(btn_restart);

    let btn_menu = UiButton::new(
      context.texture_manager.gui_btn_menu.clone(),
      context.texture_manager.gui_btn_menu_pressed.clone(),
    );
    btn_menu.set_class(&btn_top_class);
    btn_menu.set_size_x(70.0 / 480.0);
    btn_menu.set_position_x(450.0 / 480.0);
    btn_menu.set_event_on_released(events.clone(), IngameUiEvent::ButtonMenu);
    container.add_child(btn_menu);

    let game_root_entity_manager = EntityManager::new_root_manager();
    let game_mode = GameMode::new(context, events.clone());
    game_root_entity_manager.add(game_mode.clone());

    let victory_ui = VictoryUi::new(context, events.clone(), &effect_manager);
    let result = Rc::new(IngameUiRoot {
      container,
      effect_manager,
      events: events.clone(),
      game_root_entity_manager,
      game_mode,
      victory_ui,
      book_intro: BookIntro::new(context),
    });

    result
      .container
      .add_child(result.victory_ui.container.clone());
    result.container.add_child(result.book_intro.clone());
    return result;
  }

  pub fn load_game(&self, context: &mut Context, load_game_params: LoadGameParams) {
    self.game_mode.start_puzzle(
      context,
      load_game_params.book,
      load_game_params.stage_number,
    );
    if load_game_params.stage_number == 0 {
      self.book_intro.show(context, load_game_params.book);
    }
  }

  fn restart(&self, context: &mut Context) {
    self.victory_ui.container.set_visible(false);
    self.victory_ui.effect_star_animation.stop();
    self.victory_ui.effect_show_best_score.stop();
    self.game_mode.start_puzzle(
      context,
      self.game_mode.book.get(),
      self.game_mode.stage_number.get(),
    );
  }

  fn start_victory_animation(&self, context: &mut Context, victory_params: VictoryParams) {
    match victory_params.stars {
      3 => {
        self
          .victory_ui
          .image_star1
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_star2
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_star3
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_victory_title
          .set_texture(context.texture_manager.gui_awesome.clone());
      }
      2 => {
        self
          .victory_ui
          .image_star1
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_star2
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_star3
          .set_texture(context.texture_manager.star_empty.clone());
        self
          .victory_ui
          .image_victory_title
          .set_texture(context.texture_manager.gui_very_good.clone());
      }
      _ => {
        self
          .victory_ui
          .image_star1
          .set_texture(context.texture_manager.star.clone());
        self
          .victory_ui
          .image_star2
          .set_texture(context.texture_manager.star_empty.clone());
        self
          .victory_ui
          .image_star3
          .set_texture(context.texture_manager.star_empty.clone());
        self
          .victory_ui
          .image_victory_title
          .set_texture(context.texture_manager.gui_nice.clone());
      }
    };
    self.victory_ui.score.set(victory_params.score);
    self.victory_ui.score_counted.set(0.0);
    self.victory_ui.container.set_visible(true);
    self.victory_ui.container.set_active(true);
    self.victory_ui.container.set_opacity(1.0);
    self.victory_ui.image_star1.set_size(F2 { x: 0.0, y: 0.0 });
    self.victory_ui.image_star2.set_size(F2 { x: 0.0, y: 0.0 });
    self.victory_ui.image_star3.set_size(F2 { x: 0.0, y: 0.0 });
    self.victory_ui.image_victory_title.set_opacity(0.0);
    self.victory_ui.text_score_count.set_opacity(0.0);
    self
      .victory_ui
      .text_score_count
      .set_text(format!("your score . . . "));
    self.victory_ui.text_best_score.set_opacity(0.0);
    self.victory_ui.effect_star_animation.start();

    self
      .victory_ui
      .best_score
      .set(context.achievments_manager.set_score(
        self.game_mode.book.get().number(),
        self.game_mode.stage_number.get(),
        victory_params.score as i32,
        victory_params.stars,
      ));
    self.victory_ui.effect_twinkle_next.start();

    context.audio_player.play_sound(&context.audio_manager.win);
  }

  fn show_score(&self) {
    self.victory_ui.text_score_count.set_text(format!(
      "your score . . . {}",
      self.victory_ui.score.get() as i32
    ));
    self.victory_ui.start_score_count.replace(false);
    if self.victory_ui.best_score.get() {
      self.victory_ui.effect_show_best_score.start();
    }
  }

  fn undo(&self, context: &mut Context) {
    context.audio_player.play_sound(&context.audio_manager.back);
    if self.game_mode.checkpoint.get() > 0 && self.game_mode.is_game_running() {
      self
        .game_root_entity_manager
        .undo(&self.game_mode.checkpoint.get());
      self
        .game_mode
        .checkpoint
        .set(self.game_mode.checkpoint.get() - 1);
    }
  }
}

impl EffectManagerTrait<Context> for IngameUiRoot {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl UiElementTrait<Context> for IngameUiRoot {
  fn get_ui_element(&self) -> &UiElement {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    while let Some(event) = self.events.consume_event() {
      match event {
        IngameUiEvent::ButtonUndo => {
          self.undo(context);
        }
        IngameUiEvent::ButtonRestart => {
          context
            .audio_player
            .play_sound(&context.audio_manager.click);
          self.game_mode.effect_stage_fade.stop();
          if self.victory_ui.container.get_visible().get() {
            self.victory_ui.effect_star_animation.stop();
            self.victory_ui.effect_hide_victory_restart.start();
            continue;
          }
          self.restart(context);
        }
        IngameUiEvent::ButtonMenu => {
          if self.game_mode.effect_stage_fade.active.get() {
            continue;
          }
          if self.victory_ui.container.get_visible().get() {
            self.victory_ui.effect_star_animation.stop();
            self.events.add_event(IngameUiEvent::ButtonVictoryMenu);
          } else {
            context
              .audio_player
              .play_sound(&context.audio_manager.click);
            self.game_mode.paused.set(true);
            context
              .get_ui_manager_events()
              .add_event(UiManagerEvent::ShowUiOverlay(IngameMenuUiOverlay::new(
                context,
                self.events.clone(),
              )));
          }
        }
        IngameUiEvent::ButtonVictoryRestart => {
          context
            .audio_player
            .play_sound(&context.audio_manager.click);
          self.victory_ui.effect_hide_victory_restart.start();
        }
        IngameUiEvent::ButtonVictoryNext => {
          self.victory_ui.effect_star_animation.stop();
          let next_stage_number = self.game_mode.stage_number.get() + 1;
          if next_stage_number < STAGES_PER_BOOK {
            self.victory_ui.effect_hide_victory_next.start();
            self.victory_ui.container.set_active(false);
          } else {
            context
              .artificial_input_events
              .add_event(InputEvent::BackButton);
            context
              .artificial_input_events
              .add_event(InputEvent::BackButton);
            context
              .menu_choose_stages_events
              .add_event(MenuChooseStageEvent::ShowNextBook);
          }
        }
        IngameUiEvent::ButtonVictoryMenu => {
          context
            .artificial_input_events
            .add_event(InputEvent::BackButton);
        }
        IngameUiEvent::EffectVictoryRestart => {
          self.restart(context);
        }
        IngameUiEvent::EffectVictoryNext => {
          context
            .audio_player
            .play_sound(&context.audio_manager.click);
          let next_stage_number = self.game_mode.stage_number.get() + 1;
          self.victory_ui.container.set_visible(false);
          self
            .game_mode
            .start_puzzle(context, self.game_mode.book.get(), next_stage_number);
        }

        IngameUiEvent::Victory(victory_params) => {
          self.start_victory_animation(context, victory_params);
        }
        IngameUiEvent::OverlayResumeGame => {
          context
            .get_ui_manager_events()
            .add_event(UiManagerEvent::HideUiOverlay);
          self.game_mode.paused.set(false);
        }
        IngameUiEvent::OverlayNext => {
          context
            .get_ui_manager_events()
            .add_event(UiManagerEvent::HideUiOverlay);
          let next_stage_number = self.game_mode.stage_number.get() + 1;
          if next_stage_number < STAGES_PER_BOOK {
            self
              .game_mode
              .start_puzzle(context, self.game_mode.book.get(), next_stage_number);
          } else {
            context
              .get_ui_manager_events()
              .add_event(UiManagerEvent::CloseCurrentPage);
            context
              .menu_choose_stages_events
              .add_event(MenuChooseStageEvent::ShowNextBook);
          }
        }
        IngameUiEvent::OverlayFullscreen => {
          context.fullscreen();
        }
        IngameUiEvent::OverlayShowFps => {
          context.toggle_show_fps();
        }
      }
    }

    if self.victory_ui.start_score_count.get() {
      if self.victory_ui.effect_hide_victory_next.active.get()
        || self.victory_ui.effect_hide_victory_restart.active.get()
      {
        self
          .victory_ui
          .score_counted
          .set(self.victory_ui.score.get());
      }

      let score_remaining = self.victory_ui.score.get() - self.victory_ui.score_counted.get();
      if score_remaining > 0.0 {
        if !context.audio_manager.count_point.is_playing()
          || context.audio_manager.count_point.current_time() > 1.0
        {
          context
            .audio_player
            .play_sound(&context.audio_manager.count_point);
        }

        let mut added;
        if score_remaining > 100.0 {
          added = score_remaining * context.get_dt() * 0.002;
        } else {
          added = context.get_dt() * 0.2
        }
        if added > score_remaining {
          added = score_remaining;
        }

        self
          .victory_ui
          .score_counted
          .set(self.victory_ui.score_counted.get() + added);
        self.victory_ui.text_score_count.set_text(format!(
          "your score . . . {}",
          self.victory_ui.score_counted.get() as i32
        ));
      } else {
        context
          .audio_player
          .stop_sound(&context.audio_manager.count_point);
        self.show_score();
      }
    }

    self.container.update(context);
    self.game_root_entity_manager.update(context);
  }

  fn draw(&self, context: &mut Context) {
    BackgroundWood::draw(context);
    BackgroundBorders::draw(context);
    self.container.draw(context);
    self.game_root_entity_manager.draw(context);
  }

  fn get_touched_element(
    &self,
    context: &mut Context,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<Context>>> {
    return self.container.get_touched_element(context, ui_touch);
  }
}

impl UiRootTrait<Context> for IngameUiRoot {
  fn on_back_to(&self, context: &mut Context) {
    context.audio_player.play_song(&context.audio_manager.song1);
  }

  fn on_navigate_to(&self, context: &mut Context) {
    context.audio_player.play_song(&context.audio_manager.song1);
  }

  fn on_navigate_from(&self, context: &mut Context) {
    context
      .audio_player
      .stop_sound(&context.audio_manager.count_point);
  }

  fn on_press_back(&self, _context: &mut Context) -> InputState {
    if self.game_mode.is_game_running() {
      self.events.add_event(IngameUiEvent::ButtonUndo);
      return InputState::Consumed;
    }

    if self.game_mode.paused.get() {
      self.game_mode.paused.set(false);
      return InputState::Consumed;
    }

    return InputState::Available;
  }

  fn process_touch_game(&self, context: &mut Context, screen_touch: &ScreenTouch) -> bool {
    let mut game_touch = GameTouch::from_screen_touch(screen_touch, context);
    self.game_mode.process_touch(context, &mut game_touch);
    return true;
  }
}
