use crate::context::Context;
use crate::engine::*;
use crate::*;

pub struct GoalWrongReachAnimation {
  effect_manager: EffectManager,
  entity_base: EntityBase,
  texture: Rc<Texture>,
  opacity: Shared<F1>,
  size: Shared<F2>,
  position: F2,
  dead: Cell<bool>,
  state_history: StateHistory<GoalWrongReachAnimationState>,
}

pub struct GoalWrongReachAnimationState {}

impl GoalWrongReachAnimation {
  pub fn new(context: &Context, position: &F2) -> Rc<GoalWrongReachAnimation> {
    let effect_manager = EffectManager::new();
    let size = Shared::default();
    let opacity = Shared::new(1.0);
    let animation = Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    Effect::new_within_chained_effect(
      VectorAffectorF2::new(size.clone()).set_start_and_end(
        F2 { x: 0.0, y: 0.0 },
        F2 {
          x: 30.0 / 480.0,
          y: 30.0 / 480.0,
        },
        150.0,
      ),
      &animation,
    );
    Effect::new_within_chained_effect(WaitAffector::new(150.0), &animation);
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(opacity.clone()).set_end(0.0, 500.0),
      &animation,
    );

    let result = Rc::new(GoalWrongReachAnimation {
      effect_manager,
      entity_base: EntityBase::new(),
      texture: context.texture_manager.cross.clone(),
      opacity: opacity,
      size: size,
      position: *position,
      dead: Cell::new(false),
      state_history: StateHistory::new(0),
    });

    {
      let result = result.clone();
      animation.end_event.add(Box::new(move |_context| {
        result.on_animation_end();
      }));
    }
    animation.start();

    return result;
  }

  fn on_animation_end(&self) {
    self.dead.set(true);
  }
}

impl EffectManagerTrait<Context> for GoalWrongReachAnimation {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl EntityTrait<Context> for GoalWrongReachAnimation {
  type State = GoalWrongReachAnimationState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, _state: Self::State) {}

  fn get_current_state(&self) -> Self::State {
    return GoalWrongReachAnimationState {};
  }

  fn to_remove(&self) -> bool {
    return self.dead.get();
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(self.texture.clone()),
        position: self.position,
        size: *self.size.borrow(),
        depth: context.draw_depths.goal - 0.02,
        optional: DrawImageOptionalArgs {
          opacity: *self.opacity.borrow(),
          ..Default::default()
        },
      },
    );
  }
}
