use crate::context::Context;
use crate::*;

#[derive(Clone, Copy, Debug)]
enum BookIntroEvent {
  Pressed,
  EffectHideFinished,
}

pub struct BookIntro {
  pub touchable: Rc<UiTouchable>,
  effect_manager: EffectManager,

  chat_char: Rc<UiSprite>,
  chat_bar: Rc<UiSprite>,

  effect_show: Rc<Effect<SetEffect>>,
  effect_hide: Rc<Effect<SetEffect>>,

  text_line1: Rc<UiText>,
  text_line2: Rc<UiText>,
  text_line3: Rc<UiText>,

  phrases: RefCell<Vec<Vec<String>>>,
  phrase_shown: Cell<usize>,

  events: Rc<EventManager<BookIntroEvent>>,
}

impl BookIntro {
  pub fn new(context: &Context) -> Rc<BookIntro> {
    let events = EventManager::new();
    let effect_manager = EffectManager::new();
    let touchable = UiTouchable::new();
    touchable.set_visible(false);
    touchable.set_active(false);
    touchable.set_size(2.0 * context.ui_viewport.screen_size);
    touchable
      .on_released_event
      .add_event(events.clone(), BookIntroEvent::Pressed);

    let ctn_bottom = UiContainer::new();
    ctn_bottom.set_position(F2 {
      x: 0.0,
      y: context.ui_viewport.screen_bottom_right_corner.y - 140.0 / 480.0,
    });
    touchable.add_child(ctn_bottom.clone());

    let chat_char = Rc::new(UiSprite::new(
      context.texture_manager.chat_char_panda.clone(),
    ));
    chat_char.set_size_from_width(300.0 / 480.0);
    chat_char.set_depth(-0.5);
    chat_char.set_position(F2 { x: 2.0, y: 0.0 });
    ctn_bottom.add_child(chat_char.clone());

    let ctn_text = UiContainer::new();
    ctn_text.set_position(F2 { x: -2.0, y: 0.0 });
    ctn_bottom.add_child(ctn_text.clone());

    let chat_bar = Rc::new(UiSprite::new(
      context.texture_manager.chat_bar_panda.clone(),
    ));
    chat_bar.set_size(F2 {
      x: 2.0, // 580.0 / 480.0,
      y: 163.0 / 480.0,
    });
    chat_bar.set_depth(-0.1);
    ctn_text.add_child(chat_bar.clone());

    let btn_next = UiButton::new(
      context.texture_manager.gui_btn_wood.clone(),
      context.texture_manager.gui_btn_wood_pressed.clone(),
    );
    btn_next.set_position(F2 {
      x: -155.0 / 480.0,
      y: 100.0 / 480.0,
    });
    btn_next.set_size(F2 {
      x: 150.0 / 480.0,
      y: 72.0 / 480.0,
    });
    btn_next.set_depth(-0.1);
    btn_next.set_event_on_released(events.clone(), BookIntroEvent::Pressed);
    let btn_next_text = UiText::new();
    btn_next_text.set_text(String::from("Next >"));
    btn_next_text.set_font_size(50.0 / 480.0);
    btn_next_text.set_alignment(TextAlignment::Center);
    btn_next_text.set_border(true);
    btn_next.container.add_child(btn_next_text.clone());
    ctn_text.add_child(btn_next.clone());

    let text_line1 = UiText::new();
    text_line1.set_font_size(0.11);
    text_line1.set_alignment(TextAlignment::Left);
    text_line1.set_position(F2 {
      x: -0.45,
      y: -80.0 / 480.0 + 0.06,
    });
    text_line1.set_border(true);
    text_line1.set_depth(-0.1);
    ctn_text.add_child(text_line1.clone());

    let text_line2 = UiText::new();
    text_line2.set_font_size(0.11);
    text_line2.set_alignment(TextAlignment::Left);
    text_line2.set_position(F2 {
      x: -0.45,
      y: -80.0 / 480.0 + 0.06 + 40.0 / 480.0,
    });
    text_line2.set_border(true);
    text_line2.set_depth(-0.1);
    ctn_text.add_child(text_line2.clone());

    let text_line3 = UiText::new();
    text_line3.set_font_size(0.11);
    text_line3.set_alignment(TextAlignment::Left);
    text_line3.set_position(F2 {
      x: -0.45,
      y: -80.0 / 480.0 + 0.06 + 2.0 * 40.0 / 480.0,
    });
    text_line3.set_border(true);
    text_line3.set_depth(-0.1);
    ctn_text.add_child(text_line3.clone());

    let effect_show = Effect::new_within_effect_manager(SetEffect::new(), &effect_manager);
    Effect::new_within_set_effect(
      VectorAffectorF2::new(ctn_text.get_position())
        .set_start_and_end(F2 { x: -2.0, y: 0.0 }, F2 { x: 0.5, y: 0.0 }, 1000.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_show,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(chat_char.get_position())
        .set_start_and_end(F2 { x: 2.0, y: 0.0 }, F2 { x: 0.85, y: 0.0 }, 1000.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_show,
    );

    let effect_hide = Effect::new_within_effect_manager(SetEffect::new(), &effect_manager);
    Effect::new_within_set_effect(
      VectorAffectorF2::new(ctn_text.get_position())
        .set_start_and_end(F2 { x: 0.5, y: 0.0 }, F2 { x: -2.0, y: 0.0 }, 1000.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_hide,
    );
    Effect::new_within_set_effect(
      VectorAffectorF2::new(chat_char.get_position())
        .set_start_and_end(F2 { x: 0.85, y: 0.0 }, F2 { x: 2.0, y: 0.0 }, 1000.0)
        .set_progression(Box::new(ExpTransProgression::new(2.0, 6.0))),
      0.0,
      &effect_hide,
    );
    effect_hide.add_event_on_end(events.clone(), BookIntroEvent::EffectHideFinished);

    return Rc::new(BookIntro {
      touchable: touchable,
      effect_manager,
      chat_char,
      chat_bar,
      effect_show,
      effect_hide,
      text_line1,
      text_line2,
      text_line3,
      phrases: RefCell::new(Vec::new()),
      phrase_shown: Cell::new(0),
      events,
    });
  }

  pub fn show(&self, context: &mut Context, book: Book) {
    let intro = match book {
      Book::Panda => Some((
        context.texture_manager.chat_char_panda.clone(),
        context.texture_manager.chat_bar_panda.clone(),
        vec![
          vec!["Hi, I'm Panda,", "and this is my", "doodle book."],
          vec!["Touch the paw", "and drag to start", "drawing."],
          vec!["Fill the red heart", "with red ink to", "solve the puzzle."],
        ],
      )),
      Book::Cat => Some((
        context.texture_manager.chat_char_cat.clone(),
        context.texture_manager.chat_bar_cat.clone(),
        vec![
          vec!["Hi, I'm Cat,", "and this is my", "doodle book."],
          vec!["These dotted lines", "are mirrors."],
          vec!["Use the reflections", "to reach more", "doodles."],
        ],
      )),
      Book::Wolf => Some((
        context.texture_manager.chat_char_wolf.clone(),
        context.texture_manager.chat_bar_wolf.clone(),
        vec![
          vec!["Hi, I'm Wolf,", "and this is my", "doodle book."],
          vec!["Moving paws can", "draw on their own."],
          vec!["Use the natural flow", "to reach your goal."],
        ],
      )),
      Book::Rabbit => Some((
        context.texture_manager.chat_char_rabbit.clone(),
        context.texture_manager.chat_bar_rabbit.clone(),
        vec![
          vec!["Hi, I'm Rabbit,", "and this is my", "doodle book."],
          vec!["The glowing portals", "can teleport you."],
          vec!["Use them to reach", "farther places."],
        ],
      )),
      Book::Panda2 => None,
    };

    if let Some(intro) = intro {
      let (chat_char, chat_bar, phrases) = intro;
      self.chat_char.set_texture(chat_char);
      self.chat_bar.set_texture(chat_bar);
      self.phrases.replace(
        phrases
          .iter()
          .map(|v| v.iter().map(|s| String::from(*s)).collect())
          .collect(),
      );
      self.phrase_shown.set(0);
      self.update_phrase_shown();

      self.touchable.set_visible(true);
      self.touchable.set_active(true);
      self.effect_show.start();
    }
  }

  fn update_phrase_shown(&self) -> bool {
    let phrases = self.phrases.borrow();
    if let Some(curr_phrase) = phrases.get(self.phrase_shown.get()) {
      self
        .text_line1
        .set_text(curr_phrase.get(0).map_or(String::from(""), |s| s.clone()));
      self
        .text_line2
        .set_text(curr_phrase.get(1).map_or(String::from(""), |s| s.clone()));
      self
        .text_line3
        .set_text(curr_phrase.get(2).map_or(String::from(""), |s| s.clone()));
      return true;
    }
    return false;
  }
}

impl EffectManagerTrait<Context> for BookIntro {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl UiElementTrait<Context> for BookIntro {
  fn get_ui_element(&self) -> &UiElement {
    return self.touchable.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    self.touchable.update(context);

    while let Some(event) = self.events.consume_event() {
      match event {
        BookIntroEvent::Pressed => {
          context.audio_player.resume_song();
          self.phrase_shown.set(self.phrase_shown.get() + 1);
          if !self.update_phrase_shown() {
            self.touchable.set_active(false);
            self.effect_hide.start();
          }
        }
        BookIntroEvent::EffectHideFinished => {
          self.touchable.set_visible(false);
        }
      }
    }
  }

  fn draw(&self, context: &mut Context) {
    self.touchable.draw(context);
  }

  fn get_touched_element(
    &self,
    context: &mut Context,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<Context>>> {
    return self.touchable.get_touched_element(context, ui_touch);
  }

  fn consume_touch(&self, _context: &mut Context, _ui_touch: &UiTouch) {}
}
