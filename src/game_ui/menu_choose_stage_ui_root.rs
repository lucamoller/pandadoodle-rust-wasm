use crate::context::Context;
use crate::engine::*;
use crate::*;

const STAGE_ROWS: usize = 5;
const STAGES_PER_ROW: usize = 5;

#[derive(Clone, Debug)]
pub enum MenuChooseStageEvent {
  BackButtonPressed,
  EffectWaitToSlideEnd,
  EffectHideCreditsEnd,
  ShowNextBook,
  StartStage(usize),
  SelectBook(usize),
}

pub struct MenuChooseStageUiRoot {
  container: Rc<UiContainer>,
  effect_manager: EffectManager,
  screen_size: Cell<F2>,
  pivot_books: Rc<UiPivot>,
  books_ui: Vec<BookUi>,
  stages_container: Rc<UiContainer>,
  stage_icons: Rc<RefCell<Vec<StageIcon>>>,
  credits_container: Rc<UiContainer>,
  effect_show_credits: Rc<Effect<VectorAffectorF1>>,
  effect_hide_credits: Rc<Effect<VectorAffectorF1>>,
  effect_show_stages: Rc<Effect<ChainedEffect>>,
  effect_hide_stages: Rc<Effect<ChainedEffect>>,
  effect_wait_to_slide: Rc<Effect<WaitAffector>>,
  choosing_stage: Cell<bool>,
  showing_book: Cell<Book>,
  star_count_container: Rc<UiContainer>,
  star_count_text: Rc<UiText>,
  medal_count_text: Rc<UiText>,
  back_button: Rc<UiButton>,
  events: Rc<EventManager<MenuChooseStageEvent>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Book {
  Panda = 0,
  Cat,
  Wolf,
  Rabbit,
  Panda2,
}

const BOOK_ORDER: [Book; 5] = [
  Book::Panda,
  Book::Cat,
  Book::Wolf,
  Book::Rabbit,
  Book::Panda2,
];

impl Book {
  pub fn number(&self) -> usize {
    for (number, book) in BOOK_ORDER.iter().enumerate() {
      if *self == *book {
        return number;
      }
    }
    panic!("book \"{:?}\" not found in BOOK_ORDER", *self);
  }
}

enum BookType {
  Stage(StageBook),
  Credits,
}

struct StageBook {
  book: Book,
  medal: Rc<UiSprite>,
  lock: Rc<UiSprite>,
  paper: Rc<UiSprite>,
  txt_stars_needed: Rc<UiText>,
  txt_required: Rc<UiText>,
}

struct BookUi {
  book_type: BookType,
  container: Rc<UiContainer>,
  button: Rc<UiButton>,
  effect_step_left: Rc<Effect<VectorAffectorF2>>,
  effect_step_right: Rc<Effect<VectorAffectorF2>>,
  effect_step_up: Rc<Effect<VectorAffectorF2>>,
  effect_step_return: Rc<Effect<VectorAffectorF2>>,
}

struct StageIcon {
  button: Rc<UiButton>,
  sprite_locked: Rc<UiSprite>,
  stars: Rc<UiSprite>,
  text: Rc<UiText>,
}

impl MenuChooseStageUiRoot {
  pub fn new(context: &Context) -> Rc<MenuChooseStageUiRoot> {
    let screen_size = &context.ui_viewport.screen_size;
    let screen_center = &context.ui_viewport.screen_center;
    let screen_bottom_right = &context.ui_viewport.screen_bottom_right_corner;
    let screen_top_left = &context.ui_viewport.screen_top_left_corner;

    let container = UiContainer::new();
    let effect_manager = EffectManager::new();

    let events = context.menu_choose_stages_events.clone();

    let pivot_books = UiPivot::new(330.0 / 480.0);
    pivot_books.set_position_y(screen_center.y);
    pivot_books.set_depth(5.0);
    container.add_child(pivot_books.clone());

    let book_button_class = UiElementClass {
      position: Some(F2 { x: 0.5, y: 0.0 }),
      size: Some(F2 {
        x: 280.0 / 480.0,
        y: 396.0 / 480.0,
      }),
      ..Default::default()
    };

    let mut books_ui = vec![];

    for (book_number, book) in BOOK_ORDER.iter().enumerate() {
      let button_texture = match book {
        Book::Panda => context.texture_manager.gui_book_panda.clone(),
        Book::Cat => context.texture_manager.gui_book_cat.clone(),
        Book::Wolf => context.texture_manager.gui_book_wolf.clone(),
        Book::Rabbit => context.texture_manager.gui_book_rabbit.clone(),
        Book::Panda2 => context.texture_manager.gui_book_panda2.clone(),
      };

      let container = UiContainer::new();
      pivot_books.add_child(container.clone());
      let button = UiButton::new(button_texture.clone(), button_texture.clone());
      button.sprite.set_subpixel_precision(true);
      button.set_class(&book_button_class);

      container.add_child(button.clone());

      let medal = Rc::new(UiSprite::new(context.texture_manager.medal.clone()));
      medal.set_depth(-5.0);
      medal.set_subpixel_precision(true);
      button.add_child(medal.clone());

      let effect_duration = 500.0;
      let effect_step_left = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_left.set_end_onref(
        *button.get_position().borrow() + F2 { x: -0.4, y: 0.0 },
        effect_duration,
      );
      let effect_step_right = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_right.set_end_onref(
        *button.get_position().borrow() + F2 { x: 0.4, y: 0.0 },
        effect_duration,
      );
      let effect_step_up = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_up.set_end_onref(
        F2 {
          x: 0.5,
          y: screen_top_left.y - 600.0 / 480.0,
        },
        effect_duration,
      );
      effect_step_up.set_progression_onref(Box::new(ExpTransProgression::new(2.0, 6.0)));
      let effect_step_return = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_return.set_end_onref(*button.get_position().borrow(), effect_duration);
      effect_step_return.set_progression_onref(Box::new(ExpTransProgression::new(2.0, 6.0)));

      let lock = Rc::new(UiSprite::new(context.texture_manager.gui_img_lock.clone()));
      lock.set_depth(-5.0);
      lock.set_size_from_width((125.0 / 2.0) / 480.0);
      lock.set_position(F2 {
        x: 0.0 / 480.0,
        y: 0.0 / 480.0,
      });
      lock.set_subpixel_precision(true);
      button.add_child(lock.clone());

      let paper = Rc::new(UiSprite::new(context.texture_manager.gui_paper.clone()));
      paper.set_depth(-5.0);
      paper.set_size_from_width(140.0 / 480.0);
      paper.set_position(F2 {
        x: 0.0 / 480.0,
        y: 75.0 / 480.0,
      });
      paper.set_subpixel_precision(true);
      button.add_child(paper.clone());

      let txt_stars_needed = UiText::new();
      txt_stars_needed.set_alignment(TextAlignment::Center);
      txt_stars_needed.set_font_size(40.0 / 480.0);
      txt_stars_needed.set_border(true);
      txt_stars_needed.set_position(F2 {
        x: -23.0 / 480.0,
        y: 75.0 / 480.0,
      });
      txt_stars_needed.set_depth(-5.1);
      txt_stars_needed.set_text(format!(
        "{}",
        context
          .achievments_manager
          .get_stars_required_for_book(book_number)
      ));
      txt_stars_needed.use_text_cache();
      button.add_child(txt_stars_needed.clone());

      let txt_required = UiText::new();
      txt_required.set_alignment(TextAlignment::Center);
      txt_required.set_font_size(22.0 / 480.0);
      txt_required.set_border(false);
      txt_required.set_position(F2 {
        x: -23.0 / 480.0,
        y: 95.0 / 480.0,
      });
      txt_required.set_depth(-5.1);
      txt_required.set_text("required".to_string());
      txt_required.set_color(DrawColor {
        r: 30,
        g: 30,
        b: 30,
      });
      txt_required.use_text_cache();
      button.add_child(txt_required.clone());

      button.set_event_on_released(
        events.clone(),
        MenuChooseStageEvent::SelectBook(books_ui.len()),
      );
      books_ui.push(BookUi {
        book_type: BookType::Stage(StageBook {
          book: *book,
          medal: medal,
          lock: lock,
          paper: paper,
          txt_stars_needed: txt_stars_needed,
          txt_required: txt_required,
        }),
        container: container,
        button: button,
        effect_step_left: effect_step_left,
        effect_step_right: effect_step_right,
        effect_step_up: effect_step_up,
        effect_step_return: effect_step_return,
      });
    }

    {
      // Credits
      let container = UiContainer::new();
      pivot_books.add_child(container.clone());
      let button = UiButton::new(
        context.texture_manager.gui_credits.clone(),
        context.texture_manager.gui_credits.clone(),
      );
      button.set_class(&book_button_class);
      container.add_child(button.clone());

      let credits_text = UiText::new();
      credits_text.set_text(String::from("Credits"));
      credits_text.use_text_cache();
      credits_text.set_font_size(0.15);
      credits_text.set_alignment(TextAlignment::Center);
      credits_text.set_color(DrawColor {
        r: 255,
        g: 255,
        b: 255,
      });
      credits_text.set_border(false);
      button.add_child(credits_text);

      let effect_duration = 500.0;
      let effect_step_left = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_left.set_end_onref(
        *button.get_position().borrow() + F2 { x: -0.4, y: 0.0 },
        effect_duration,
      );
      let effect_step_right = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_right.set_end_onref(
        *button.get_position().borrow() + F2 { x: 0.4, y: 0.0 },
        effect_duration,
      );
      let effect_step_up = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_up.set_end_onref(
        F2 {
          x: 0.5,
          y: screen_top_left.y - 600.0 / 480.0,
        },
        effect_duration,
      );
      effect_step_up.set_progression_onref(Box::new(ExpTransProgression::new(2.0, 6.0)));
      let effect_step_return = Effect::new_within_effect_manager(
        VectorAffectorF2::new(button.get_position()),
        &effect_manager,
      );
      effect_step_return.set_end_onref(*button.get_position().borrow(), effect_duration);

      button.set_event_on_released(
        events.clone(),
        MenuChooseStageEvent::SelectBook(books_ui.len()),
      );
      books_ui.push(BookUi {
        book_type: BookType::Credits,
        container: container,
        button: button,
        effect_step_left: effect_step_left,
        effect_step_right: effect_step_right,
        effect_step_up: effect_step_up,
        effect_step_return: effect_step_return,
      });
    }

    let credits_container = UiContainer::new();
    {
      credits_container.set_visible(false);
      credits_container.set_position(F2 { x: 0.0, y: 0.3 });
      container.add_child(credits_container.clone());

      let luca = UiText::new();
      luca.set_text(String::from("Luca Mattos Möller"));
      luca.set_position(F2 { x: 0.5, y: 0.1 });
      luca.set_font_size(0.12);
      luca.set_alignment(TextAlignment::Center);
      luca.set_border(true);
      credits_container.add_child(luca);

      let luca_role = UiText::new();
      luca_role.set_text(String::from("-Programming"));
      luca_role.set_position(F2 { x: 0.5, y: 0.2 });
      luca_role.set_font_size(0.12);
      luca_role.set_alignment(TextAlignment::Center);
      luca_role.set_color(DrawColor { r: 0, g: 0, b: 0 });
      // luca_role.set_border(true);
      credits_container.add_child(luca_role);

      let ruela = UiText::new();
      ruela.set_text(String::from("Vinícius Canaã Medeiros Ruela"));
      ruela.set_position(F2 { x: 0.5, y: 0.40 });
      ruela.set_font_size(0.12);
      ruela.set_alignment(TextAlignment::Center);
      ruela.set_border(true);
      credits_container.add_child(ruela);

      let ruela_role = UiText::new();
      ruela_role.set_text(String::from("-Programming, Music, Art"));
      ruela_role.set_position(F2 { x: 0.5, y: 0.50 });
      ruela_role.set_font_size(0.12);
      ruela_role.set_alignment(TextAlignment::Center);
      ruela_role.set_color(DrawColor { r: 0, g: 0, b: 0 });
      // ruela_role.set_border(true);
      credits_container.add_child(ruela_role);

      let natalia = UiText::new();
      natalia.set_text(String::from("Natália Roim"));
      natalia.set_position(F2 { x: 0.5, y: 0.7 });
      natalia.set_font_size(0.12);
      natalia.set_alignment(TextAlignment::Center);
      natalia.set_border(true);
      credits_container.add_child(natalia);

      let natalia_role = UiText::new();
      natalia_role.set_text(String::from("-Drawings, Art"));
      natalia_role.set_position(F2 { x: 0.5, y: 0.8 });
      natalia_role.set_font_size(0.12);
      natalia_role.set_alignment(TextAlignment::Center);
      natalia_role.set_color(DrawColor { r: 0, g: 0, b: 0 });
      credits_container.add_child(natalia_role);

      // let donation_line1 = UiText::new();
      // donation_line1.set_text(String::from("Enjoyed the game?"));
      // donation_line1.set_position(F2 { x: 0.5, y: 1.0 });
      // donation_line1.set_font_size(0.10);
      // donation_line1.set_alignment(TextAlignment::Center);
      // donation_line1.set_border(true);
      // credits_container.add_child(donation_line1);

      // let donation_line2 = UiText::new();
      // donation_line2.set_text(String::from("Consider making a donation =)"));
      // donation_line2.set_position(F2 { x: 0.5, y: 1.1 });
      // donation_line2.set_font_size(0.10);
      // donation_line2.set_alignment(TextAlignment::Center);
      // donation_line2.set_border(true);
      // credits_container.add_child(donation_line2);

      // let btn_donate = UiButton::new(
      //   context.texture_manager.gui_btn_wood.clone(),
      //   context.texture_manager.gui_btn_wood_pressed.clone(),
      // );
      // btn_donate.sprite.set_color(DrawColor {
      //   r: 255,
      //   g: 255,
      //   b: 180,
      // });
      // btn_donate.set_size_x(240.0 / 480.0);
      // btn_donate.set_size_y(82.0 / 480.0);
      // btn_donate.set_position(F2 { x: 0.5, y: 1.25 });
      // btn_donate
      //   .on_released_event
      //   .add(Box::new(|context: &mut Context, _ui_touch: &UiTouch| {
      //     context
      //       .window
      //       .open_with_url("https://www.paypal.com/donate?hosted_button_id=R38K7FJGVP6EY")
      //       .expect("window.open_with_url_and_target failed");
      //   }));
      // btn_donate.set_sound_on_released(context.audio_manager.click.clone());
      // let btn_donate_text = UiText::new();
      // btn_donate_text.set_text(String::from("Donate"));
      // btn_donate_text.set_font_size(75.0 / 480.0);
      // btn_donate_text.set_alignment(TextAlignment::Center);
      // btn_donate_text.set_color(DrawColor {
      //   r: 255,
      //   g: 255,
      //   b: 200,
      // });
      // btn_donate_text.set_border(true);
      // btn_donate.container.add_child(btn_donate_text);
      // credits_container.add_child(btn_donate.clone());
    }

    let effect_show_credits = Effect::new_within_effect_manager(
      VectorAffectorF1::new(credits_container.get_opacity()).set_start_and_end(0.0, 1.0, 1000.0),
      &effect_manager,
    );
    let effect_hide_credits = Effect::new_within_effect_manager(
      VectorAffectorF1::new(credits_container.get_opacity()).set_start_and_end(1.0, 0.0, 500.0),
      &effect_manager,
    );
    effect_hide_credits
      .add_event_on_end(events.clone(), MenuChooseStageEvent::EffectHideCreditsEnd);

    let stages_container = UiContainer::new();
    let stage_icons = Rc::new(RefCell::new(vec![]));

    let effect_show_stages =
      Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    Effect::new_within_chained_effect(WaitAffector::new(300.0), &effect_show_stages);
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(stages_container.get_opacity()).set_start_and_end(0.0, 1.0, 800.0),
      &effect_show_stages,
    );

    let effect_hide_stages =
      Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(stages_container.get_opacity()).set_start_and_end(1.0, 0.0, 300.0),
      &effect_hide_stages,
    );

    let effect_wait_to_slide =
      Effect::new_within_effect_manager(WaitAffector::new(500.0), &effect_manager);
    effect_wait_to_slide
      .add_event_on_end(events.clone(), MenuChooseStageEvent::EffectWaitToSlideEnd);

    let star_count_container = UiContainer::new();
    star_count_container.set_position(F2 {
      x: 0.0,
      y: screen_center.y * 0.1,
    });
    star_count_container.set_depth(5.0);
    container.add_child(star_count_container.clone());

    let star_count_text = UiText::new();
    star_count_text.use_text_cache();
    star_count_text.set_alignment(TextAlignment::Right);
    star_count_text.set_font_size(0.14);
    star_count_text.set_border(true);
    star_count_text.set_position(F2 {
      x: 180.0 / 480.0,
      y: 5.0 / 480.0,
    });
    star_count_text.set_depth(0.5);
    star_count_container.add_child(star_count_text.clone());

    let star_count_sprite = Rc::new(UiSprite::new(context.texture_manager.mini_star.clone()));
    star_count_sprite.set_size_from_width(55.0 / 480.0);
    star_count_sprite.set_position(F2 {
      x: 215.0 / 480.0,
      y: -5.0 / 480.0,
    });
    star_count_sprite.set_depth(0.5);
    star_count_container.add_child(star_count_sprite.clone());

    let medal_count_text = UiText::new();
    medal_count_text.use_text_cache();
    medal_count_text.set_alignment(TextAlignment::Right);
    medal_count_text.set_font_size(0.14);
    medal_count_text.set_border(true);
    medal_count_text.set_position(F2 {
      x: 410.0 / 480.0,
      y: 5.0 / 480.0,
    });
    medal_count_text.set_depth(0.5);
    star_count_container.add_child(medal_count_text.clone());

    let medal_count_sprite = Rc::new(UiSprite::new(context.texture_manager.medal.clone()));
    medal_count_sprite.set_size_from_width(45.0 / 480.0);
    medal_count_sprite.set_position(F2 {
      x: 440.0 / 480.0,
      y: 0.0 / 480.0,
    });
    medal_count_sprite.set_depth(0.5);
    star_count_container.add_child(medal_count_sprite.clone());

    let back_button = UiButton::new(
      context.texture_manager.gui_btn_back.clone(),
      context.texture_manager.gui_btn_back_pressed.clone(),
    );
    back_button.set_size_from_x(60.0 / 480.0);
    back_button.set_position(F2 {
      x: 40.0 / 480.0,
      y: screen_bottom_right.y - 60.0 / 480.0,
    });
    back_button.set_depth(0.9);
    back_button.set_event_on_released(events.clone(), MenuChooseStageEvent::BackButtonPressed);
    back_button.set_sound_on_released(context.audio_manager.click.clone());
    container.add_child(back_button.clone());

    stages_container.set_position(*screen_center);
    stages_container.set_visible(false);
    stages_container.set_opacity(0.0);
    stages_container.set_depth(10.0);
    container.add_child(stages_container.clone());
    for i in 0..STAGE_ROWS {
      for j in 0..STAGES_PER_ROW {
        let stage_number = i * STAGE_ROWS + j;
        let container = UiContainer::new();
        container.set_position(F2 {
          x: 0.2 * (-(STAGES_PER_ROW as F1 - 1.0) * 0.5 + (j as F1)),
          y: 0.208 / 0.198 * 0.2 * (-(STAGE_ROWS as F1 - 1.0) * 0.5 + (i as F1)),
        });
        stages_container.add_child(container.clone());

        let button = UiButton::new(
          context.texture_manager.gui_stage_icon.clone(),
          context.texture_manager.gui_stage_icon_pressed.clone(),
        );
        button.set_size(F2 {
          x: 100.0 / 480.0,
          y: 120.0 / 480.0,
        });
        button.set_event_on_released(
          events.clone(),
          MenuChooseStageEvent::StartStage(stage_number),
        );
        button.set_sound_on_released(context.audio_manager.click.clone());
        container.add_child(button.clone());

        let stars = Rc::new(UiSprite::new(context.texture_manager.collect0star.clone()));
        stars.set_size(F2 {
          x: 38.0 * 97.0 / 47.0 / 480.0,
          y: 38.0 / 480.0,
        });
        stars.set_position(F2 {
          x: 0.0,
          y: 30.0 / 480.0,
        });
        stars.set_depth(-0.9);
        container.add_child(stars.clone());

        let text = UiText::new();
        text.use_text_cache();
        text.set_font_size(50.0 / 480.0);
        text.set_alignment(TextAlignment::Center);
        text.set_border(false);
        text.set_color(DrawColor {
          r: 102,
          g: 43,
          b: 0,
        });
        text.set_position(F2 {
          x: 0.0,
          y: -10.0 / 480.0,
        });
        text.set_depth(-0.9);
        text.set_text(format!("{}", i * STAGE_ROWS + j + 1));
        container.add_child(text.clone());

        let sprite_locked = Rc::new(UiSprite::new(
          context.texture_manager.gui_stage_icon_lock.clone(),
        ));
        sprite_locked.set_size(F2 {
          x: 100.0 / 480.0,
          y: 120.0 / 480.0,
        });
        container.add_child(sprite_locked.clone());

        stage_icons.borrow_mut().push(StageIcon {
          button: button,
          sprite_locked: sprite_locked,
          stars: stars,
          text: text,
        });
      }
    }

    let result = Rc::new(MenuChooseStageUiRoot {
      container: container.clone(),
      effect_manager,
      screen_size: Cell::new(*screen_size),
      pivot_books: pivot_books.clone(),
      books_ui: books_ui,
      stages_container: stages_container.clone(),
      stage_icons: stage_icons.clone(),
      credits_container: credits_container.clone(),
      effect_show_credits,
      effect_hide_credits,
      choosing_stage: Cell::new(false),
      showing_book: Cell::new(Book::Panda),
      effect_show_stages: effect_show_stages,
      effect_hide_stages: effect_hide_stages,
      effect_wait_to_slide: effect_wait_to_slide.clone(),
      star_count_container: star_count_container,
      star_count_text: star_count_text,
      medal_count_text: medal_count_text,
      back_button: back_button.clone(),
      events: events.clone(),
    });

    return result;
  }

  fn show_book(&self, context: &mut Context, book: Book) {
    context.audio_player.play_sound(&context.audio_manager.book);
    self.pivot_books.set_active(false);
    self.choosing_stage.set(true);
    self.stages_container.set_visible(true);
    self.stages_container.set_active(true);
    self.effect_show_stages.start();
    self.showing_book.set(book);
    self.refresh_star_counts(context);
    self.refresh_stage_stars(context);
  }

  fn hide_book(&self, context: &mut Context) {
    self.pivot_books.set_active(true);
    self.choosing_stage.set(false);
    self.stages_container.set_active(false);
    self.effect_hide_stages.start();
    for book_ui in self.books_ui.iter() {
      book_ui.effect_step_return.start();
    }
    self.refresh_star_counts(context);
    self.refresh_stage_stars(context);
  }

  fn show_credits(&self, context: &mut Context) {
    context.audio_player.play_sound(&context.audio_manager.book);
    self.pivot_books.set_active(false);
    self.credits_container.set_visible(true);
    self.effect_show_credits.start();
    self.star_count_container.set_visible(false);
  }

  fn hide_credits(&self) {
    self.pivot_books.set_active(true);
    self.effect_hide_credits.start();
    self.star_count_container.set_visible(true);
    for book_ui in self.books_ui.iter() {
      book_ui.effect_step_return.start();
    }
  }

  fn start_stage(&self, context: &mut Context, stage_number: usize) {
    context
      .ui_events
      .add_event(UiEvent::LoadGame(LoadGameParams {
        book: self.showing_book.get(),
        stage_number: stage_number,
      }));
  }

  fn refresh_star_counts(&self, context: &mut Context) {
    if self.choosing_stage.get() {
      self.star_count_text.set_text(format!(
        "{}/{}",
        context
          .achievments_manager
          .get_stars_per_book(self.showing_book.get().number()),
        context
          .achievments_manager
          .get_total_existing_stars_per_book(),
      ));
    } else {
      self.star_count_text.set_text(format!(
        "Total: {}",
        context.achievments_manager.get_total_stars()
      ));
    }
    self.medal_count_text.set_text(format!(
      "{}/{}",
      context.achievments_manager.get_total_medals(),
      context.achievments_manager.get_total_existing_medals()
    ));

    for book_ui in self.books_ui.iter() {
      if let BookType::Stage(stage_book) = &book_ui.book_type {
        stage_book.medal.set_visible(
          context
            .achievments_manager
            .has_medal(stage_book.book.number()),
        );

        let locked = !context
          .achievments_manager
          .is_book_available(stage_book.book.number());
        stage_book.lock.set_visible(locked);
        stage_book.paper.set_visible(locked);
        stage_book.txt_stars_needed.set_visible(locked);
        stage_book.txt_required.set_visible(locked);
      }
    }
  }

  fn refresh_stage_stars(&self, context: &mut Context) {
    for (stage_number, stage_icon) in self.stage_icons.borrow().iter().enumerate() {
      if context
        .achievments_manager
        .is_stage_available(self.showing_book.get().number(), stage_number)
      {
        stage_icon.button.active.set(true);
        stage_icon.button.set_visible(true);
        stage_icon.stars.set_visible(true);
        stage_icon.text.set_visible(true);
        stage_icon.sprite_locked.set_visible(false);

        match context
          .achievments_manager
          .get_stage_stars(self.showing_book.get().number(), stage_number)
        {
          3 => {
            stage_icon
              .stars
              .set_texture(context.texture_manager.collect3star.clone());
          }
          2 => {
            stage_icon
              .stars
              .set_texture(context.texture_manager.collect2star.clone());
          }
          1 => {
            stage_icon
              .stars
              .set_texture(context.texture_manager.collect1star.clone());
          }
          _ => {
            stage_icon
              .stars
              .set_texture(context.texture_manager.collect0star.clone());
          }
        };
      } else {
        stage_icon.button.set_visible(false);
        stage_icon.button.active.set(false);
        stage_icon.stars.set_visible(false);
        stage_icon.text.set_visible(false);
        stage_icon.sprite_locked.set_visible(true);
      }
    }
  }

  fn refresh_screen_size(&self, context: &mut Context) {
    if self.screen_size.get() != context.ui_viewport.screen_size {
      self.screen_size.set(context.ui_viewport.screen_size);
      let screen_bottom_right = &context.ui_viewport.screen_bottom_right_corner;
      let screen_top_left = &context.ui_viewport.screen_top_left_corner;

      self.back_button.set_position(F2 {
        x: 40.0 / 480.0,
        y: screen_bottom_right.y - 60.0 / 480.0,
      });

      for book in self.books_ui.iter() {
        let effect_duration = 500.0;
        book.effect_step_up.set_end_onref(
          F2 {
            x: 0.5,
            y: screen_top_left.y - 600.0 / 480.0,
          },
          effect_duration,
        );
      }
    }
  }

  fn back_button_pressed(&self, context: &mut Context) {
    context
      .artificial_input_events
      .add_event(InputEvent::BackButton);
  }
}

impl EffectManagerTrait<Context> for MenuChooseStageUiRoot {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl UiElementTrait<Context> for MenuChooseStageUiRoot {
  fn get_ui_element(&self) -> &UiElement {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut Context) {
    self.refresh_screen_size(context);
    while let Some(event) = self.events.consume_event() {
      match event {
        MenuChooseStageEvent::BackButtonPressed => {
          self.back_button_pressed(context);
        }
        MenuChooseStageEvent::EffectHideCreditsEnd => {
          self.credits_container.set_visible(false);
        }
        MenuChooseStageEvent::EffectWaitToSlideEnd => {
          self.pivot_books.slide_next();
        }
        MenuChooseStageEvent::ShowNextBook => {
          if self.choosing_stage.get() {
            self.hide_book(context);
          }
          self.effect_wait_to_slide.start();
        }
        MenuChooseStageEvent::SelectBook(book_index) => {
          let prev_step_left = if book_index > 0 {
            Some(self.books_ui[book_index - 1].effect_step_left.clone())
          } else {
            None
          };
          let next_step_right = if book_index + 1 < self.books_ui.len() {
            Some(self.books_ui[book_index + 1].effect_step_right.clone())
          } else {
            None
          };
          let self_step_up = self.books_ui[book_index].effect_step_up.clone();

          let current_index = self.pivot_books.index.get() as usize;
          if current_index != book_index {
            self.pivot_books.slide(book_index as i32);
            return;
          }

          match &self.books_ui[book_index].book_type {
            BookType::Stage(stage_book) => {
              if !context
                .achievments_manager
                .is_book_available(stage_book.book.number())
              {
                return;
              }
              self.show_book(context, stage_book.book.clone());
            }
            BookType::Credits => self.show_credits(context),
          };

          if let Some(prev_step_left) = &prev_step_left {
            prev_step_left.start();
          }
          if let Some(next_step_right) = &next_step_right {
            next_step_right.start();
          }
          self_step_up.start();
        }
        MenuChooseStageEvent::StartStage(stage_number) => {
          self.start_stage(context, stage_number);
        }
      };
    }

    let ctn_book_position_y =
      (context.get_latest_timestamp() * 0.0005 * std::f32::consts::PI).sin() * 20.0 / 480.0;

    for book_ui in self.books_ui.iter() {
      let mut scale = (book_ui.button.get_absolute_params(context).position.x - 0.5)
        .cos()
        .abs();
      if let BookType::Credits = book_ui.book_type {
        scale = scale * 1.32;
      }
      book_ui.button.set_size_from_x(scale * 280.0 / 480.0);
      book_ui.container.set_position_y(ctn_book_position_y);
      if let BookType::Stage(stage_book) = &book_ui.book_type {
        stage_book.medal.set_position(F2 {
          x: scale * 88.0 / 480.0,
          y: scale * 93.0 / 480.0,
        });
        stage_book.medal.set_size_from_width(scale * 78.0 / 480.0);
      }
    }
    self.container.update(context);
  }

  fn draw(&self, context: &mut Context) {
    BackgroundWood::draw(context);
    BackgroundBorders::draw(context);
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

impl UiRootTrait<Context> for MenuChooseStageUiRoot {
  fn on_back_to(&self, context: &mut Context) {
    self.refresh_star_counts(context);
    self.refresh_stage_stars(context);
    context.audio_player.play_song(&context.audio_manager.song2);
  }

  fn on_press_back(&self, context: &mut Context) -> InputState {
    if self.choosing_stage.get() {
      self.hide_book(context);
      return InputState::Consumed;
    }
    if self.credits_container.get_visible().get() {
      self.hide_credits();
      return InputState::Consumed;
    }
    return InputState::Available;
  }

  fn on_navigate_to(&self, context: &mut Context) {
    self.refresh_star_counts(context);
    self.refresh_stage_stars(context);
    context.audio_player.play_song(&context.audio_manager.song2);
  }
}
