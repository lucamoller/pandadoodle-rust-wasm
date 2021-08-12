use crate::context::Context;
use crate::engine::*;
use crate::game::goal_collect_effect::GoalCollectEffect;
use crate::game::goal_wrong_reach_animation::GoalWrongReachAnimation;
use crate::game::paint_color::PaintColor;
use crate::*;

const CIRCLE_RADIUS: F1 = 0.048;
const DRAW_WIDTH: F1 = 0.131;

pub struct Goal {
  entity_base: EntityBase,
  position: F2,
  pub paint_color: PaintColor,
  texture_empty: Rc<Texture>,
  texture_filled: Rc<Texture>,
  pub filled: Cell<bool>,
  collect_goal_effect: Rc<GoalCollectEffect>,
  wrong_reach_animations: Rc<EntityManager<GoalWrongReachAnimation>>,
  state_history: StateHistory<GoalState>,
}

pub struct GoalState {
  filled: bool,
}

impl Goal {
  pub fn new(context: &Context, position: F2, paint_color: PaintColor) -> Rc<Goal> {
    let entity_base = EntityBase::new();
    let collect_goal_effect = GoalCollectEffect::new(context, paint_color, position);

    let collect_goal_effect_manager = EntityManager::new_within_parent_entity(&entity_base);
    collect_goal_effect_manager.add(collect_goal_effect.clone());
    let state_history = StateHistory::new(0);

    let wrong_reach_animations = EntityManager::new_within_parent_entity(&entity_base);

    return Rc::new(Goal {
      entity_base,
      position: position,
      paint_color: paint_color,
      texture_empty: match paint_color {
        PaintColor::Blue => context.texture_manager.goals_blue.clone(),
        PaintColor::Gray => context.texture_manager.goals_gray.clone(),
        PaintColor::Green => context.texture_manager.goals_green.clone(),
        PaintColor::Orange => context.texture_manager.goals_orange.clone(),
        PaintColor::Purple => context.texture_manager.goals_purple.clone(),
        PaintColor::Red => context.texture_manager.goals_red.clone(),
        PaintColor::Yellow => context.texture_manager.goals_yellow.clone(),
        PaintColor::NoColor => context.texture_manager.cross.clone(),
      },
      texture_filled: match paint_color {
        PaintColor::Blue => context.texture_manager.goals_blue_fill.clone(),
        PaintColor::Gray => context.texture_manager.goals_gray_fill.clone(),
        PaintColor::Green => context.texture_manager.goals_green_fill.clone(),
        PaintColor::Orange => context.texture_manager.goals_orange_fill.clone(),
        PaintColor::Purple => context.texture_manager.goals_purple_fill.clone(),
        PaintColor::Red => context.texture_manager.goals_red_fill.clone(),
        PaintColor::Yellow => context.texture_manager.goals_yellow_fill.clone(),
        PaintColor::NoColor => context.texture_manager.cross.clone(),
      },
      filled: Cell::new(false),
      collect_goal_effect: collect_goal_effect,
      wrong_reach_animations,
      state_history: state_history,
    });
  }

  pub fn restart(&self) {
    self.filled.set(false);
  }

  pub fn wrong_reach(&self, context: &mut Context, position: &F2) {
    context
      .audio_player
      .play_sound(&context.audio_manager.wrong);
    self
      .wrong_reach_animations
      .add(GoalWrongReachAnimation::new(context, position));
  }

  pub fn play_collect_animation(&self) {
    self.collect_goal_effect.start();
  }

  pub fn set_filled(&self, current_checkpoint: &u32) {
    if !self.filled.get() {
      self.register_current_state(*current_checkpoint);
      self.filled.set(true);
      self.play_collect_animation();
    }
  }
}

impl EffectManagerTrait<Context> for Goal {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Goal {
  type State = GoalState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    self.filled.set(state.filled);
  }

  fn get_current_state(&self) -> Self::State {
    return GoalState {
      filled: self.filled.get(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    let time = *context.get_latest_timestamp();
    self.wrong_reach_animations.draw(context);
    if self.filled.get() {
      let mut size = self.texture_filled.get_size_from_width(DRAW_WIDTH);
      size.x *= (9.0 + (time * 0.005 + 1.53).sin()) / 9.0;
      size.y *= (9.0 + (time * 0.005).sin()) / 9.0;

      context.draw_manager.draw_viewport(
        &context.game_viewport,
        DrawImageArgs {
          source: DrawSource::Texture(self.texture_filled.clone()),
          position: self.position,
          size: size,
          depth: context.draw_depths.goal,
          optional: DrawImageOptionalArgs {
            opacity: context.stage_opacity.get(),
            rotation: (time * 0.0015).sin() * 0.5,
            subpixel_precision: true,
            ..Default::default()
          },
        },
      );
    } else {
      context.draw_manager.draw_viewport(
        &context.game_viewport,
        DrawImageArgs {
          source: DrawSource::Texture(self.texture_empty.clone()),
          position: self.position,
          size: self.texture_empty.get_size_from_width(DRAW_WIDTH),
          depth: context.draw_depths.goal,
          optional: DrawImageOptionalArgs {
            opacity: context.stage_opacity.get(),
            ..Default::default()
          },
        },
      );
    }
  }
}

impl CircleShape for Goal {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &CIRCLE_RADIUS;
  }
}
