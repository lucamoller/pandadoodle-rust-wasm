use crate::context::Context;
use crate::engine::*;
use crate::game::goal_wrong_reach_animation::GoalWrongReachAnimation;
use crate::*;

pub struct Barrier {
  entity_base: EntityBase,
  radius: F1,
  position: F2,
  size: F2,
  wrong_reach_animations: Rc<EntityManager<GoalWrongReachAnimation>>,
  state_history: StateHistory<()>,
}

impl Barrier {
  pub fn new(_context: &Context, position: F2, radius: F1) -> Rc<Barrier> {
    let entity_base = EntityBase::new();
    let wrong_reach_animations = EntityManager::new_within_parent_entity(&entity_base);
    return Rc::new(Barrier {
      entity_base,
      radius: radius,
      position: position,
      size: F2 {
        x: 1.8 * radius,
        y: 1.8 * radius,
      },
      wrong_reach_animations,
      state_history: StateHistory::new(0),
    });
  }

  pub fn wrong_reach(&self, context: &mut Context, position: &F2) {
    context
      .audio_player
      .play_sound(&context.audio_manager.wrong);
    self
      .wrong_reach_animations
      .add(GoalWrongReachAnimation::new(context, position));
  }
}

impl EffectManagerTrait<Context> for Barrier {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Barrier {
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

  fn draw(&self, context: &mut Context) {
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(context.texture_manager.circle40.clone()),
        position: self.position,
        size: self.size,
        depth: context.draw_depths.barrier,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          color: DrawColor { r: 0, g: 0, b: 0 },
          ..Default::default()
        },
      },
    );
  }
}

impl CircleShape for Barrier {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &self.radius;
  }
}
