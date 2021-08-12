use crate::context::Context;
use crate::engine::*;
use crate::game::paint_color::PaintColor;
use crate::*;

pub struct GoalCollectEffect {
  effect_manager: EffectManager,
  entity_base: EntityBase,
  texture_circle: Rc<Texture>,
  paint_color: PaintColor,
  position: F2,
  xs: [F1; 10],
  ys: [F1; 10],
  show: Cell<bool>,
  opacity: Shared<F1>,
  opacity_effect: Rc<Effect<VectorAffectorF1>>,
  state_history: StateHistory<GoalCollectEffectState>,
}

pub struct GoalCollectEffectState {}

impl GoalCollectEffect {
  pub fn new(context: &Context, paint_color: PaintColor, position: F2) -> Rc<GoalCollectEffect> {
    let effect_manager = EffectManager::new();
    let opacity = Shared::default();
    let opacity_effect = Effect::new_within_effect_manager(
      VectorAffectorF1::new(opacity.clone()).set_start_and_end(1.0, 0.0, 500.0),
      &effect_manager,
    );

    let mut xs = [0.0; 10];
    let mut ys = [0.0; 10];
    let d_rad = std::f32::consts::PI / 5.0;
    let mut rad: F1 = 0.0;
    for i in 0..10 {
      xs[i] = rad.cos();
      ys[i] = rad.sin();
      rad += d_rad;
    }

    let goal_collect_effect = Rc::new(GoalCollectEffect {
      effect_manager,
      entity_base: EntityBase::new(),
      texture_circle: context.texture_manager.circle40.clone(),
      paint_color: paint_color,
      position: position,
      xs: xs,
      ys: ys,
      show: Cell::new(false),
      opacity: opacity.clone(),
      opacity_effect: opacity_effect.clone(),
      state_history: StateHistory::new(0),
    });

    {
      let goal_collect_effect_clone = goal_collect_effect.clone();
      goal_collect_effect
        .opacity_effect
        .end_event
        .add(Box::new(move |_context| {
          goal_collect_effect_clone.on_animation_end();
        }));
    }
    return goal_collect_effect;
  }

  fn on_animation_end(&self) {
    self.show.set(false);
  }

  pub fn start(&self) {
    self.show.set(true);
    self.opacity_effect.start();
  }
}

impl EffectManagerTrait<Context> for GoalCollectEffect {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl EntityTrait<Context> for GoalCollectEffect {
  type State = GoalCollectEffectState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, _state: Self::State) {}

  fn get_current_state(&self) -> Self::State {
    return GoalCollectEffectState {};
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    if !self.show.get() {
      return;
    }

    let radius: F1 = (1.0 - *self.opacity.borrow()) * 0.12;
    let size: F1 = 30.0 * self.opacity.borrow().sqrt() / 480.0;
    let size = F2 { x: size, y: size };
    for i in 0..10 {
      let position = F2 {
        x: self.xs[i] * radius,
        y: self.ys[i] * radius,
      } + self.position;

      context.draw_manager.draw_viewport(
        &context.game_viewport,
        DrawImageArgs {
          source: DrawSource::Texture(self.texture_circle.clone()),
          position: position,
          size: size,
          depth: context.draw_depths.goal - 0.01,
          optional: DrawImageOptionalArgs {
            opacity: *self.opacity.borrow(),
            color: self.paint_color.get_draw_color(),
            ..Default::default()
          },
        },
      );
    }
  }
}
