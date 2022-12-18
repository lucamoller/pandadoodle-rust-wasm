use crate::context::*;
use crate::engine::*;
use crate::game::background_canvas::BackgroundCanvas;
use crate::game::brush::Brush;
use crate::game::paint_color;
use crate::game::paint_path::*;
use crate::game::paint_point;
use crate::game::paint_point::PaintPoint;
use crate::game::paint_source::*;
use crate::game::point_grid::PointGrid;
use crate::game::source::Source;
use crate::game::stage::Stage;
use crate::game::stages_data::StageData;
use crate::game::star_bar::*;
use crate::game_ui::Book;
use crate::game_ui::IngameUiEvent;
use crate::*;

pub struct GameMode {
  events: Rc<EventManager<IngameUiEvent>>,
  effect_manager: EffectManager,
  entity_base: EntityBase,
  pub checkpoint: Cell<u32>,
  pub book: Cell<Book>,
  pub stage_number: Cell<usize>,

  stage_manager: Rc<EntityManager<Stage>>,
  pub stage: RefCell<Rc<Stage>>,

  point_grid_manager: Rc<EntityManager<PointGrid>>,
  pub point_grid: RefCell<Rc<PointGrid>>,
  pub active_path: RefCell<Weak<SourcedPaintPath>>,
  pub brush: RefCell<Brush>,

  pub paint_paths: Rc<EntityManager<SourcedPaintPath>>,
  pub particles: Shared<Vec<Particle>>,

  star_bar: Rc<StarBar>,

  pub effect_stage_fade: Rc<Effect<ChainedEffect>>,

  pub paused: Cell<bool>,
  pub finished: Cell<bool>,
  pub state_history: StateHistory<GameModeState>,
}

pub struct GameModeState {
  pub active_path: Weak<SourcedPaintPath>,
}

pub fn calculate_new_point_position(touch_position: &F2, last_point_position: &F2) -> F2 {
  let mut step = touch_position - last_point_position;
  step *= &(paint_point::DISTANCE_BETWEEN_POINTS / step.length());
  return last_point_position + step;
}

impl GameMode {
  pub fn new(context: &mut Context, events: Rc<EventManager<IngameUiEvent>>) -> Rc<GameMode> {
    let effect_manager = EffectManager::new();
    let entity_base = EntityBase::new();
    let particles = Shared::new(Vec::new());
    let paint_paths = EntityManager::new_within_parent_entity(&entity_base);

    let effect_stage_fade =
      Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    Effect::new_within_chained_effect(WaitAffector::new(1000.0), &effect_stage_fade);
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(context.stage_opacity.clone()).set_start_and_end(1.0, 0.1, 700.0),
      &effect_stage_fade,
    );

    let stage_manager = EntityManager::new_within_parent_entity(&entity_base);
    let stage = Stage::new(context, &StageData::default());
    stage_manager.add(stage.clone());

    let point_grid_manager = EntityManager::new_within_parent_entity(&entity_base);
    let point_grid = PointGrid::new(context);
    point_grid_manager.add(point_grid.clone());

    let star_bar_manager = EntityManager::new_within_parent_entity(&entity_base);
    let star_bar = StarBar::new(context);
    star_bar_manager.add(star_bar.clone());

    let result = Rc::new(GameMode {
      events,
      effect_manager,
      entity_base,
      checkpoint: Cell::new(0),
      book: Cell::new(Book::Panda),
      stage_number: Cell::new(0),
      stage_manager,
      stage: RefCell::new(stage),
      point_grid_manager,
      point_grid: RefCell::new(point_grid),
      active_path: RefCell::new(Weak::new()),
      brush: RefCell::new(Brush::new(context, particles.clone())),
      particles: particles,
      paint_paths,
      star_bar,
      effect_stage_fade: effect_stage_fade.clone(),
      paused: Cell::new(false),
      finished: Cell::new(false),
      state_history: StateHistory::new(0),
    });

    {
      let game_mode = result.clone();
      effect_stage_fade.end_event.add(Box::new(move |context| {
        game_mode.on_effect_stage_fade_end(context);
      }));
    }

    context.game_mode.replace(Some(result.clone()));
    return result;
  }

  pub fn is_game_running(&self) -> bool {
    return !self.paused.get() && !self.finished.get();
  }

  pub fn start_puzzle(&self, context: &mut Context, book: Book, stage_number: usize) {
    self.paint_paths.clear();
    self.active_path.replace(Weak::new());
    self
      .brush
      .replace(Brush::new(context, self.particles.clone()));
    self.point_grid_manager.clear();
    let new_point_grid = PointGrid::new(context);
    self.point_grid_manager.add(new_point_grid.clone());
    self.point_grid.replace(new_point_grid);
    self.paused.set(false);
    self.finished.set(false);
    context.stage_opacity.replace(1.0);

    self.book.set(book);
    self.stage_number.set(stage_number);

    let stage_data = context.stages_data.get_stage(book.number(), stage_number);

    let stage_index = context
      .achievments_manager
      .get_stage_index(book.number(), stage_number);

    self.stage_manager.clear();
    let new_stage = Stage::new(context, stage_data);
    self.stage_manager.add(new_stage.clone());
    self.stage.replace(new_stage);
  }

  pub fn on_effect_stage_fade_end(&self, context: &mut Context) {
    let stage_index = context
      .achievments_manager
      .get_stage_index(self.book.get().number(), self.stage_number.get());

    self.events.add_event(IngameUiEvent::Victory(VictoryParams {
      score: self.stage.borrow().get_current_score(),
      stars: self.stage.borrow().get_stars(),
    }));
  }

  pub fn update_active_path_with_touch(&self, context: &mut Context, game_touch: &mut GameTouch) {
    game_touch.position = context
      .game_viewport
      .move_inside_viewport(&game_touch.position);

    let active_path_pointer = self.active_path.borrow().upgrade().unwrap();
    let active_path = active_path_pointer.as_ref();

    let source = active_path.source.upgrade().unwrap();
    let source_option = active_path.source.upgrade();
    if game_touch.touch_type == TouchType::Released {
      if !source.has_paint_left() {
        self.active_path.replace(Weak::new());
      } else {
        active_path.put_on_hold(&self.checkpoint.get());
      }
      return;
    }

    self.update_path_position(
      context,
      active_path,
      &game_touch.position,
      source.as_ref(),
      source_option,
    );
  }

  pub fn update_path_position(
    &self,
    context: &mut Context,
    paint_path: &PaintPath,
    position: &F2,
    paint_source: &impl PaintSource,
    source_option: Option<Rc<Source>>,
  ) {
    if paint_path.last_point.borrow().upgrade().is_none() {
      if paint_source.has_paint_left() {
        self.create_paint_point(position, paint_path, context);
      }
      return;
    }

    loop {
      if paint_path.disabled.get() || paint_path.path_on_hold.get() {
        break;
      }

      let last_point_pointer = paint_path.last_point.borrow().upgrade().unwrap();

      // console_log_with_div!(
      //   "update_path_with_cursor_position! \n\
      //   last_point: {:?},\n\
      //   game_touch: {:?}\n\
      //   F2::distance2: {:?}",
      //   last_point_pointer.borrow().position,
      //   game_touch,
      //   F2::distance2(&last_point_pointer.borrow().position, &game_touch.position),
      // );

      if F2::distance2(&last_point_pointer.position, &position)
        < paint_point::DISTANCE_SQUARED_BETWEEN_POINTS
      {
        break;
      }
      if !paint_source.has_paint_left()
      //paint_point::DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT
      //   != source.request_ink(&paint_point::DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT)
      {
        break;
      }

      let new_point_position =
        calculate_new_point_position(&position, &last_point_pointer.position);

      if match source_option.as_ref() {
        Some(source) => self.should_consume_ink(
          &new_point_position,
          last_point_pointer.as_ref(),
          source.as_ref(),
        ),
        None => true,
      } {
        paint_source.consume_ink(
          &paint_point::DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT,
          &self.checkpoint.get(),
        );
      }
      self.create_paint_point(&new_point_position, paint_path, context);
    }
  }

  pub fn should_consume_ink(
    &self,
    new_point_position: &F2,
    last_point: &PaintPoint,
    source: &Source,
  ) -> bool {
    if source.is_point_inside(new_point_position) {
      return false;
    }

    for goal in self.stage.borrow().goals.managed_entities.borrow().iter() {
      let goal = goal.as_ref();
      if last_point.paint_color != goal.paint_color {
        continue;
      }
      if goal.is_point_inside(new_point_position) {
        return false;
      }
    }

    for portal in self.stage.borrow().portals.managed_entities.borrow().iter() {
      if portal.endpoint1.is_point_inside(new_point_position) {
        return false;
      }
      if portal.endpoint2.is_point_inside(new_point_position) {
        return false;
      }
    }

    return true;
  }

  pub fn update_brush(&self, game_touch: &mut GameTouch) {
    if let Some(active_path) = self.active_path.borrow().upgrade() {
      if !active_path.disabled.get() {
        let active_path = active_path.as_ref();
        self.brush.borrow_mut().position = game_touch.position;
        self.brush.borrow_mut().active = !active_path.path_on_hold.get();
        self
          .brush
          .borrow_mut()
          .set_color(&active_path.paint_color.get());
        return;
      }
    }

    self.brush.borrow_mut().active = false;
  }

  pub fn create_paint_point(&self, position: &F2, paint_path: &PaintPath, context: &mut Context) {
    let new_point = self.point_grid.borrow().create_point(
      context,
      position,
      &self.checkpoint.get(),
      &paint_path.paint_color.get(),
      &paint_path.point_count.get(),
      // self.active_path.borrow().clone(),
    );

    paint_path.last_point.replace(Rc::downgrade(&new_point));
    paint_path.point_count.set(paint_path.point_count.get() + 1);

    for point_collided in self.point_grid.borrow().get_collisions(&new_point) {
      if new_point.paint_color != point_collided.paint_color {
        paint_path.paint_color.set(paint_color::combine_colors(
          &paint_path.paint_color.get(),
          &point_collided.paint_color,
        ));
      }
    }

    for source in self.stage.borrow().sources.managed_entities.borrow().iter() {
      if new_point.collide_with_circle(source.as_ref()) {
        let mut paint_color = paint_path.paint_color.get();
        paint_color.combine_with(&source.paint_color);
        paint_path.paint_color.set(paint_color);
      }
    }

    let mut new_goal_filled = false;
    for goal in self.stage.borrow().goals.managed_entities.borrow().iter() {
      let goal = goal.as_ref();
      if new_point.collide_with_circle(goal) {
        if new_point.paint_color == goal.paint_color {
          if !goal.filled.get() {
            goal.set_filled(&self.checkpoint.get());
            new_goal_filled = true;

            let sound = match self.goals_remaining() {
              0 => &context.audio_manager.star3,
              1 => &context.audio_manager.star2,
              2 => &context.audio_manager.star1,
              _ => &context.audio_manager.collect,
            };
            context.audio_player.play_sound(sound);
            context.vibration_manager.vibrate();
          }
        } else {
          if !goal.filled.get() {
            paint_path.disabled.set(true);
            goal.wrong_reach(context, &new_point.position);
          } else {
            let mut paint_color = paint_path.paint_color.get();
            paint_color.combine_with(&goal.paint_color);
            paint_path.paint_color.set(paint_color);
          }
        }
      }
    }

    for symmetric_path in paint_path.symmetric_paths.managed_entities.borrow().iter() {
      let symmetric_position = symmetric_path.get_symmetric_point(position);
      if !context
        .game_viewport
        .is_inside_viewport(&symmetric_position)
      {
        symmetric_path.paint_path.disabled.set(true);
      }
      if symmetric_path.paint_path.disabled.get() {
        continue;
      }

      self.create_paint_point(&symmetric_position, symmetric_path, context);
    }

    for mirror in self.stage.borrow().mirrors.managed_entities.borrow().iter() {
      if new_point.collide_with_segment(mirror.as_ref()) {
        mirror.touch(context, self.checkpoint.get(), paint_path)
      }
    }

    for barrier in self
      .stage
      .borrow()
      .barriers
      .managed_entities
      .borrow()
      .iter()
    {
      if new_point.collide_with_circle(barrier.as_ref()) {
        paint_path.disabled.set(true);
        barrier.wrong_reach(context, &new_point.position);
      }
    }

    let activated_portal = paint_path.activated_portal.borrow().clone();
    if let Some(activated_portal) = activated_portal {
      if !(new_point.collide_with_circle(&activated_portal.endpoint1)
        || new_point.collide_with_circle(&activated_portal.endpoint2))
      {
        paint_path.activated_portal.replace(None);
      }
    } else {
      for portal in self.stage.borrow().portals.managed_entities.borrow().iter() {
        if new_point.collide_with_circle(&portal.endpoint1) {
          paint_path.activated_portal.replace(Some(portal.clone()));
          paint_path.put_on_hold(&self.checkpoint.get());
          self.create_paint_point(&portal.endpoint2.position, paint_path, context);
        }
        if new_point.collide_with_circle(&portal.endpoint2) {
          paint_path.activated_portal.replace(Some(portal.clone()));
          paint_path.put_on_hold(&self.checkpoint.get());
          self.create_paint_point(&portal.endpoint1.position, paint_path, context);
        }
      }
    }

    if new_goal_filled {
      self.check_stage_complete(context);
    }
  }

  pub fn goals_remaining(&self) -> usize {
    let mut result = 0;
    for goal in self.stage.borrow().goals.managed_entities.borrow().iter() {
      let goal = goal.as_ref();
      if !goal.filled.get() {
        result += 1;
      }
    }
    return result;
  }

  pub fn check_stage_complete(&self, _context: &mut Context) {
    let all_goals_filled = self.goals_remaining() == 0;
    if all_goals_filled {
      for goal in self.stage.borrow().goals.managed_entities.borrow().iter() {
        goal.play_collect_animation();
      }

      self.finished.set(true);
      self.effect_stage_fade.start();
      self.active_path.replace(Weak::new());
    }
  }

  pub fn try_start_path(
    &self,
    context: &Context,
    game_touch: &GameTouch,
  ) -> Option<Weak<SourcedPaintPath>> {
    if !(game_touch.touch_type == TouchType::Pressed || game_touch.touch_type == TouchType::Moved) {
      return None;
    }

    let stage = self.stage.borrow();
    let source_option = stage.get_clicked_source(&game_touch.position);
    match source_option {
      Some(source) => {
        if source.has_paint_left() {
          self.checkpoint.set(self.checkpoint.get() + 1);
          self.register_current_state(self.checkpoint.get());
        }
        let new_paint_path = SourcedPaintPath::new(
          context,
          &source,
          self.particles.clone(),
          self.checkpoint.get(),
        );
        let result = Rc::downgrade(&new_paint_path);

        self.paint_paths.add(new_paint_path);

        // console_log_with_div!(
        //   "source touch! started path {:?}, {:?}",
        //   source.position,
        //   source.paint_color
        // );
        return Some(result);
      }
      _ => {}
    }
    return None;
  }

  pub fn process_touch(&self, context: &mut Context, game_touch: &mut GameTouch) {
    if !self.is_game_running() {
      return;
    }
    if let Some(active_path) = self.active_path.borrow().upgrade() {
      let active_path = active_path.as_ref();
      if active_path.path_on_hold.get() && game_touch.touch_type == TouchType::Pressed {
        if let Some(last_point) = active_path.last_point.borrow().upgrade() {
          if F2::distance2(&last_point.position, &game_touch.position)
            < paint_point::DISTANCE_SQUARED_TO_REACTIVATE_PATH
          {
            self.checkpoint.set(self.checkpoint.get() + 1);
            active_path.release_from_hold(&self.checkpoint.get());
            self.register_current_state(self.checkpoint.get());
          }
        }
      }
    }

    let use_active: bool = match self.active_path.borrow().upgrade() {
      Some(active_path) => !active_path.path_on_hold.get() && !active_path.disabled.get(),
      None => false,
    };

    if use_active {
      self.update_active_path_with_touch(context, game_touch);
    } else if let Some(new_path) = self.try_start_path(context, game_touch) {
      if let Some(active_path) = self.active_path.borrow().upgrade() {
        active_path.active.set(false);
      }
      self.active_path.replace(new_path);
      self.update_active_path_with_touch(context, game_touch);
    }

    self.update_brush(game_touch);
  }
}

impl EffectManagerTrait<Context> for GameMode {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl EntityTrait<Context> for GameMode {
  type State = GameModeState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    self.active_path.replace(state.active_path);
  }

  fn get_current_state(&self) -> Self::State {
    return GameModeState {
      active_path: self.active_path.borrow().clone(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, context: &mut Context) {
    self.brush.borrow_mut().update(context);
    self
      .star_bar
      .set_star_fills(self.stage.borrow().get_star_bar_fills());

    for particle in self.particles.borrow_mut().iter_mut() {
      particle.update(context);
    }
    self
      .particles
      .borrow_mut()
      .retain(|particle| !particle.to_remove());
  }

  fn draw(&self, context: &mut Context) {
    BackgroundCanvas::draw(context, self.book.get());
    for particle in self.particles.borrow().iter() {
      particle.draw(context, context.get_game_viewport().clone());
    }
  }
}
