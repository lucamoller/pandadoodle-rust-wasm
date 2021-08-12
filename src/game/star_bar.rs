use crate::engine::*;
use crate::*;

pub struct StarBar {
  entity_base: EntityBase,
  star1: Rc<Star>,
  star2: Rc<Star>,
  star3: Rc<Star>,
  state_history: StateHistory<()>,
}

impl StarBar {
  pub fn new(context: &Context) -> Rc<StarBar> {
    let entity_base = EntityBase::new();
    let stars_manager = EntityManager::new_within_parent_entity(&entity_base);
    let star1 = Star::new(F2 { x: 0.27, y: 0.07 }, context);
    let star2 = Star::new(F2 { x: 0.17, y: 0.07 }, context);
    let star3 = Star::new(F2 { x: 0.07, y: 0.07 }, context);
    stars_manager.add(star1.clone());
    stars_manager.add(star2.clone());
    stars_manager.add(star3.clone());

    return Rc::new(StarBar {
      entity_base,
      star1: star1,
      star2: star2,
      star3: star3,
      state_history: StateHistory::new(0),
    });
  }

  pub fn set_star_fills(&self, star_fills: (F1, F1, F1)) {
    let (star1, star2, star3) = star_fills;
    self.star1.set_fill(star1);
    self.star2.set_fill(star2);
    self.star3.set_fill(star3);
  }
}

impl EffectManagerTrait<Context> for StarBar {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for StarBar {
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

pub struct Star {
  effect_manager: EffectManager,
  entity_base: EntityBase,
  position: F2,
  size: F2,
  fill: Cell<F1>,
  bright_opacity: Shared<F1>,
  effect_fade: Rc<Effect<VectorAffectorF1>>,
  glow_size: Shared<F2>,
  state_history: StateHistory<()>,
}

impl Star {
  pub fn new(position: F2, context: &Context) -> Rc<Star> {
    let effect_manager = EffectManager::new();
    let size = context.texture_manager.star.get_size_from_width(0.1);

    let bright_opacity = Shared::new(1.0);
    let effect_fade = Effect::new_within_effect_manager(
      VectorAffectorF1::new(bright_opacity.clone()),
      &effect_manager,
    );
    effect_fade.set_end_onref(0.0, 500.0);

    let glow_size = Shared::new(size);
    let effect_glow_size = Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    Effect::new_within_chained_effect(
      VectorAffectorF2::new(glow_size.clone()).set_end(size * 1.03, 800.0),
      &effect_glow_size,
    );
    Effect::new_within_chained_effect(
      VectorAffectorF2::new(glow_size.clone()).set_end(size, 800.0),
      &effect_glow_size,
    );
    effect_glow_size.set_looped(true);
    effect_glow_size.start();

    return Rc::new(Star {
      effect_manager,
      entity_base: EntityBase::new(),
      position: position,
      size: size,
      fill: Cell::new(1.0),
      bright_opacity: bright_opacity,
      effect_fade: effect_fade,
      glow_size: glow_size,
      state_history: StateHistory::new(0),
    });
  }

  pub fn set_fill(&self, new_fill: F1) {
    let mut new_fill = new_fill;
    if new_fill < 0.0 {
      new_fill = 0.0;
    }

    if self.fill.get() == 0.0 {
      if new_fill > 0.0 {
        self.effect_fade.stop();
        *self.bright_opacity.borrow_mut() = 1.0;
      }
    } else if new_fill == 0.0 {
      self.effect_fade.start();
    }

    self.fill.set(new_fill);
  }
}

impl EffectManagerTrait<Context> for Star {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return Some(&self.effect_manager);
  }
}

impl EntityTrait<Context> for Star {
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
    context.draw_ui_viewport(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.star_l.clone()),
      position: self.position,
      size: self.size,
      depth: context.draw_depths.ui,
      optional: DrawImageOptionalArgs {
        ..Default::default()
      },
    });

    context.draw_ui_viewport(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.star.clone()),
      position: F2 {
        x: self.position.x - self.size.x * 0.5,
        y: self.position.y,
      },
      size: F2 {
        x: self.size.x * self.fill.get(),
        y: self.size.y,
      },
      depth: context.draw_depths.ui - 0.1,
      optional: DrawImageOptionalArgs {
        anchor_point: F2 { x: 0.0, y: 0.5 },
        partial_region_offset: F2 { x: 0.0, y: 0.0 },
        partial_region_size: F2 {
          x: self.fill.get(),
          y: 1.0,
        },
        subpixel_precision: true,
        ..Default::default()
      },
    });

    context.draw_ui_viewport(DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.star_active_bright.clone()),
      position: self.position,
      size: self.glow_size.get(),
      depth: context.draw_depths.ui - 0.2,
      optional: DrawImageOptionalArgs {
        opacity: self.bright_opacity.get(),
        subpixel_precision: true,
        ..Default::default()
      },
    });
  }
}
