use crate::context::Context;
use crate::game::barrier::*;
use crate::game::goal::Goal;
use crate::game::mirror::*;
use crate::game::moving_source::*;
use crate::game::portal::*;
use crate::game::source::Source;
use crate::game::stages_data::*;
use crate::*;

pub struct Stage {
  entity_base: EntityBase,
  pub barriers: Rc<EntityManager<Barrier>>,
  pub sources: Rc<EntityManager<Source>>,
  pub goals: Rc<EntityManager<Goal>>,
  pub mirrors: Rc<EntityManager<Mirror>>,
  pub moving_sources: Rc<EntityManager<MovingSource>>,
  pub portals: Rc<EntityManager<Portal>>,
  record: F1,
  score_2_stars: F1,
  score_3_stars: F1,
  state_history: StateHistory<()>,
}

impl Stage {
  pub fn new(context: &Context, stage_data: &StageData) -> Rc<Stage> {
    let entity_base = EntityBase::new();
    let barriers = EntityManager::new_within_parent_entity(&entity_base);
    let sources = EntityManager::new_within_parent_entity(&entity_base);
    let goals = EntityManager::new_within_parent_entity(&entity_base);
    let mirrors = EntityManager::new_within_parent_entity(&entity_base);
    let moving_sources = EntityManager::new_within_parent_entity(&entity_base);
    let portals = EntityManager::new_within_parent_entity(&entity_base);

    let new_stage = Rc::new(Stage {
      entity_base,
      barriers,
      sources,
      goals,
      mirrors,
      moving_sources,
      portals,
      record: stage_data.record as F1,
      score_2_stars: stage_data.score_2_stars as F1,
      score_3_stars: stage_data.score_3_stars as F1,
      state_history: StateHistory::new(0),
    });

    for source_data in stage_data.sources.iter() {
      new_stage.create_source(context, source_data);
    }
    for goal_data in stage_data.goals.iter() {
      new_stage.create_goal(context, goal_data);
    }
    for mirror_data in stage_data.mirrors.iter() {
      new_stage.create_mirror(context, mirror_data);
    }
    for moving_source_data in stage_data.moving_sources.iter() {
      new_stage.create_moving_source(context, moving_source_data);
    }
    for portal_data in stage_data.portals.iter() {
      new_stage.create_portal(context, portal_data);
    }
    return new_stage;
  }

  pub fn new2(record: F1, score_2_stars: F1, score_3_stars: F1) -> Rc<Stage> {
    let entity_base = EntityBase::new();
    let barriers = EntityManager::new_within_parent_entity(&entity_base);
    let sources = EntityManager::new_within_parent_entity(&entity_base);
    let goals = EntityManager::new_within_parent_entity(&entity_base);
    let mirrors = EntityManager::new_within_parent_entity(&entity_base);
    let moving_sources = EntityManager::new_within_parent_entity(&entity_base);
    let portals = EntityManager::new_within_parent_entity(&entity_base);

    return Rc::new(Stage {
      entity_base,
      barriers,
      sources,
      goals,
      mirrors,
      moving_sources,
      portals,
      record: record,
      score_2_stars: score_2_stars,
      score_3_stars: score_3_stars,
      state_history: StateHistory::new(0),
    });
  }

  pub fn create_source(&self, context: &Context, source_data: &SourceData) {
    self.sources.add(Source::new(
      context,
      source_data.paint_amount,
      source_data.paint_color,
      source_data.position,
    ));
  }

  pub fn create_goal(&self, context: &Context, goal_data: &GoalData) {
    self.goals.add(Goal::new(
      context,
      goal_data.position,
      goal_data.paint_color,
    ));
  }

  pub fn create_mirror(&self, context: &Context, mirror_data: &MirrorData) {
    let new_mirror = Mirror::new(context, mirror_data.p1, mirror_data.p2);
    for existing_mirror in self.mirrors.managed_entities.borrow().iter() {
      if let Some(intersection) = new_mirror.get_intersection_with_segment(existing_mirror.as_ref())
      {
        self
          .barriers
          .add(Barrier::new(context, intersection, 0.024));
      }
    }
    self.mirrors.add(new_mirror);
  }

  pub fn create_moving_source(&self, context: &Context, moving_source_data: &MovingSourceData) {
    self
      .moving_sources
      .add(MovingSource::new(context, moving_source_data));
  }

  pub fn create_portal(&self, context: &Context, portal_data: &PortalData) {
    let portal_type = self.portals.managed_entities.borrow().len() % 3;
    self
      .portals
      .add(Portal::new(context, portal_data, portal_type));
  }

  pub fn get_clicked_source(&self, position: &F2) -> Option<Rc<Source>> {
    for source in self.sources.managed_entities.borrow().iter() {
      if GeometryUtils::point_inside_circle(position, &source.position, &source.draw_radius) {
        return Some(source.clone());
      }
    }
    return None;
  }

  pub fn get_star_bar_fills(&self) -> (F1, F1, F1) {
    let current_score = self.get_current_score();
    let max_score = self.get_max_score();

    let mut star1_change1 = 13000.0;
    while star1_change1 + 1000.0 > max_score {
      star1_change1 -= 500.0;
    }
    let star1_change2 = 10000.0;

    let star1;
    let mut star2 = 1.0;
    let mut star3 = 1.0;

    if current_score >= star1_change1 {
      star1 = 0.75 + 0.25 * (current_score - star1_change1) / (max_score - star1_change1);
      return (star1, star2, star3);
    }

    if current_score >= star1_change2 {
      star1 = 0.5 + 0.25 * (current_score - star1_change2) / (star1_change1 - star1_change2);
      return (star1, star2, star3);
    }

    if current_score >= self.score_3_stars {
      star1 = 0.5 * (current_score - self.score_3_stars) / (star1_change2 - self.score_3_stars);
      return (star1, star2, star3);
    }
    star1 = -1.0;

    if current_score >= self.score_2_stars {
      star2 = (current_score - self.score_2_stars) / (self.score_3_stars - self.score_2_stars);
      return (star1, star2, star3);
    }
    star2 = -1.0;

    star3 = current_score / self.score_2_stars;
    return (star1, star2, star3);
  }

  pub fn get_current_score(&self) -> F1 {
    if self.record == -1.0 {
      return 10000.0;
    }
    let record_remaining_paint_amount = self.get_total_initial_paint_amount() - self.record;
    let current_remaining_paint_amount = self.get_total_current_paint_amount();
    return 10000.0 * current_remaining_paint_amount / record_remaining_paint_amount;
  }

  pub fn get_stars(&self) -> usize {
    if self.record == -1.0 {
      return 3;
    }
    let score = self.get_current_score();
    if score >= self.score_3_stars {
      return 3;
    }
    if score >= self.score_2_stars {
      return 2;
    }
    return 1;
  }

  fn get_max_score(&self) -> F1 {
    if self.record == -1.0 {
      return 20000.0;
    }
    let initial_paint_amount = self.get_total_initial_paint_amount();
    let record_remaining_paint_amount = initial_paint_amount - self.record;
    return 10000.0 * initial_paint_amount / record_remaining_paint_amount;
  }

  fn get_total_initial_paint_amount(&self) -> F1 {
    let mut result = 0.0;
    for source in self.sources.managed_entities.borrow().iter() {
      result += source.initial_paint_amount;
    }
    return result;
  }

  fn get_total_current_paint_amount(&self) -> F1 {
    let mut result = 0.0;
    for source in self.sources.managed_entities.borrow().iter() {
      result += source.current_paint_amount.get();
    }
    return result;
  }
}

impl EffectManagerTrait<Context> for Stage {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Stage {
  type State = ();

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, _state: Self::State) {}

  fn get_current_state(&self) -> Self::State {
    return ();
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, _context: &mut Context) {}
}
