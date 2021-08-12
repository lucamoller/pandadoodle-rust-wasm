use super::paint_color::*;
use crate::context::*;
use crate::game::paint_point::*;
use crate::game::paint_source::*;
use crate::*;

const SOURCE_RADIUS: F1 = 0.055;
const SOURCE_DRAW_RADIUS: F1 = 0.065;

pub struct Source {
  entity_base: EntityBase,
  pub paint_color: PaintColor,
  pub position: F2,
  pub radius: F1,
  pub draw_radius: F1,

  pub current_paint_amount: Shared<F1>,
  pub initial_paint_amount: F1,

  source_texture: Rc<Texture>,
  source_empty_texture: Rc<Texture>,
  state_history: StateHistory<SourceState>,
  text_cache: Rc<TextCache>,
}

pub struct SourceState {
  current_paint_amount: F1,
}

impl Source {
  pub fn new(
    context: &Context,
    initial_paint_amount: F1,
    paint_color: PaintColor,
    position: F2,
  ) -> Rc<Source> {
    let adjusted_initial_paint_amount = initial_paint_amount + 0.01;
    let current_paint_amount = Shared::new(adjusted_initial_paint_amount);
    let state_history = StateHistory::new(0);

    return Rc::new(Source {
      entity_base: EntityBase::new(),
      paint_color: paint_color,

      position: position,
      radius: SOURCE_RADIUS,
      draw_radius: SOURCE_DRAW_RADIUS,

      current_paint_amount: current_paint_amount.clone(),
      initial_paint_amount: adjusted_initial_paint_amount,

      source_texture: match paint_color {
        PaintColor::Red => context.texture_manager.source_red.clone(),
        PaintColor::Blue => context.texture_manager.source_blue.clone(),
        PaintColor::Yellow => context.texture_manager.source_yellow.clone(),
        _ => context.texture_manager.source_red.clone(),
      },
      source_empty_texture: match paint_color {
        PaintColor::Red => context.texture_manager.source_red_empty.clone(),
        PaintColor::Blue => context.texture_manager.source_blue_empty.clone(),
        PaintColor::Yellow => context.texture_manager.source_yellow_empty.clone(),
        _ => context.texture_manager.source_red_empty.clone(),
      },

      state_history: state_history,
      text_cache: TextCache::new(),
    });
  }

  pub fn restart(&mut self) {
    *self.current_paint_amount.borrow_mut() = self.initial_paint_amount;
  }
}

impl EffectManagerTrait<Context> for Source {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for Source {
  type State = SourceState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    *self.current_paint_amount.borrow_mut() = state.current_paint_amount;
  }

  fn get_current_state(&self) -> Self::State {
    return SourceState {
      current_paint_amount: *self.current_paint_amount.borrow(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {
    if *self.current_paint_amount.borrow() > self.initial_paint_amount {
      *self.current_paint_amount.borrow_mut() = self.initial_paint_amount;
    }
  }

  fn draw(&self, context: &mut Context) {
    let mut size = F2 {
      x: SOURCE_DRAW_RADIUS * 2.0,
      y: SOURCE_DRAW_RADIUS * 2.0,
    };

    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(self.source_empty_texture.clone()),
        position: self.position,
        size: size,
        depth: context.draw_depths.source + 0.1,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          ..Default::default()
        },
      },
    );

    let fill_portion = *self.current_paint_amount.borrow() / self.initial_paint_amount;
    let total_height = size.y;
    size.y *= fill_portion;

    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(self.source_texture.clone()),
        position: F2 {
          x: self.position.x,
          y: self.position.y + ((total_height - size.y) / 2.0),
        },
        size: size,
        depth: context.draw_depths.source,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          partial_region_offset: F2 {
            x: 0.0,
            y: 1.0 - fill_portion,
          },
          partial_region_size: F2 {
            x: 1.0,
            y: fill_portion,
          },
          subpixel_precision: true,
          ..Default::default()
        },
      },
    );

    context.draw_manager.draw_string_viewport(
      &context.game_viewport,
      DrawStringArgs {
        text: String::from(format!(
          "{:.1}",
          (*self.current_paint_amount.borrow() / 100.0)
        )),
        position: self.position + F2 { x: 0.0, y: 0.040 },
        font_size: 1.1 * SOURCE_DRAW_RADIUS,
        depth: context.draw_depths.source - 0.1,
        optional: DrawStringOptionalArgs {
          alignment: TextAlignment::Center,
          opacity: context.stage_opacity.get(),
          border: true,
          // border_scale: 4.0,
          text_cache: Some(self.text_cache.clone()),
          ..Default::default()
        },
      },
    );
  }
}

impl CircleShape for Source {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &self.radius;
  }
}

impl PaintSource for Source {
  fn has_paint_left(&self) -> bool {
    return *self.current_paint_amount.borrow() >= DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT;
  }

  fn consume_ink(&self, amount: &F1, current_checkpoint: &u32) {
    self.register_current_state(*current_checkpoint);
    *self.current_paint_amount.borrow_mut() -= amount;
    // ASSERT(paintAmount.value >= 0, ("paint amount < 0!"));
  }
}
