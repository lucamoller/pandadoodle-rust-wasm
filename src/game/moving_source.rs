use crate::game::paint_color::PaintColor;
use crate::game::paint_path::*;
use crate::game::paint_point::*;
use crate::game::paint_source::*;
use crate::game::stages_data::*;
use crate::*;

pub struct MovingSource {
  entity_base: EntityBase,
  position: F2,
  radius: F1,
  speed: F1,
  paint_color: Cell<PaintColor>,
  time_moved: Cell<F1>,
  paw: RefCell<MovingSourcePaw>,
  active_path: RefCell<Option<Rc<PaintPath>>>,
  paint_paths: Rc<EntityManager<PaintPath>>,
  current_paint_amount: Cell<F1>,

  cached_canvas: CachedCanvasBackend,

  state_history: StateHistory<MovingSourceState>,
}

pub struct MovingSourcePaw {
  paw_position: F2,
  paw_radius: F1,
}

pub struct MovingSourceState {
  paint_color: PaintColor,
  time_moved: F1,
  active_path: Option<Rc<PaintPath>>,
  paint_paths: EntityManager<PaintPath>,
}

impl MovingSource {
  pub fn new(context: &Context, moving_source_data: &MovingSourceData) -> Rc<MovingSource> {
    let entity_base = EntityBase::new();
    let paint_paths = EntityManager::new_within_parent_entity(&entity_base);

    return Rc::new(MovingSource {
      entity_base,
      position: moving_source_data.position,
      radius: moving_source_data.radius,
      speed: moving_source_data.speed,
      paint_color: Cell::new(PaintColor::NoColor),
      time_moved: Cell::new(0.0),
      paw: RefCell::new(MovingSourcePaw {
        paw_position: F2::default(),
        paw_radius: 0.0160,
      }),
      active_path: RefCell::new(None),
      paint_paths,
      current_paint_amount: Cell::new(50000.0),
      cached_canvas: CachedCanvasBackend::new(&context.get_canvas_size()),
      state_history: StateHistory::new(0),
    });
  }

  pub fn restart(&self) {
    self.time_moved.set(0.0);
    self.paint_color.set(PaintColor::NoColor);
  }

  fn collided_with_empty_wrong_goal(
    &self,
    context: &mut Context,
    candidate_paint_color: PaintColor,
  ) -> bool {
    let game_mode = context.game_mode.borrow().clone().unwrap();
    for goal in game_mode
      .stage
      .borrow()
      .goals
      .managed_entities
      .borrow()
      .iter()
    {
      if self.paw.borrow().collide_with_circle(goal.as_ref()) {
        if !goal.filled.get() && candidate_paint_color != goal.paint_color {
          return true;
        }
      }
    }
    return false;
  }
}

impl EffectManagerTrait<Context> for MovingSource {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for MovingSource {
  type State = MovingSourceState;

  fn get_base(&self) -> &EntityBase {
    return &self.entity_base;
  }

  fn get_state_history(&self) -> &StateHistory<Self::State> {
    return &self.state_history;
  }

  fn apply_state(&self, state: Self::State) {
    self.paint_color.set(state.paint_color);
    self.time_moved.set(state.time_moved);
    self.active_path.replace(state.active_path);
    self.paint_paths.replace(state.paint_paths);
  }

  fn get_current_state(&self) -> Self::State {
    return MovingSourceState {
      paint_color: self.paint_color.get(),
      time_moved: self.time_moved.get(),
      active_path: self.active_path.borrow().clone(),
      paint_paths: self.paint_paths.as_ref().clone(),
    };
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, context: &mut Context) {
    let game_mode = context.game_mode.borrow().clone().unwrap();
    self.register_current_state(game_mode.checkpoint.get());
    let mut total_dt = *context.get_dt();
    while total_dt > 0.0 {
      let fps60_dt = 1000.0 / 60.0;
      let curr_dt = {
        if fps60_dt < total_dt {
          fps60_dt
        } else {
          total_dt
        }
      };
      total_dt -= curr_dt;

      self.time_moved.set(self.time_moved.get() + curr_dt);
      let current_angle = 2.0 * std::f32::consts::PI * self.time_moved.get() * self.speed;
      self.paw.borrow_mut().paw_position = self.position
        + F2 {
          x: current_angle.cos() * self.radius,
          y: current_angle.sin() * self.radius,
        };

      if !context
        .game_viewport
        .is_inside_viewport(&self.paw.borrow().paw_position)
      {
        // console_log_with_div!("paw outside game_viewport!");
        if let Some(active_path) = self.active_path.borrow().as_ref() {
          active_path.disabled.set(true);
        }
        self.active_path.replace(None);
        self.paint_color.set(PaintColor::NoColor);
        break;
      }

      if self.paint_color.get() == PaintColor::NoColor {
        let collisions = game_mode
          .point_grid
          .borrow()
          .get_collisions_circle_shape(context, &*self.paw.borrow());
        for paint_point in collisions.iter() {
          // console_log_with_div!("collided paint point: {:?}", paint_point.position);
          if self.collided_with_empty_wrong_goal(context, paint_point.paint_color) {
            continue;
          }
          self.paint_color.set(paint_point.paint_color);
        }
        for goal in game_mode
          .stage
          .borrow()
          .goals
          .managed_entities
          .borrow()
          .iter()
        {
          if !goal.filled.get() {
            continue;
          }
          if self.paw.borrow().collide_with_circle(goal.as_ref()) {
            self.paint_color.set(goal.paint_color);
          }
        }
        if self.paint_color.get() != PaintColor::NoColor {
          if match self.active_path.borrow().as_ref() {
            Some(active_path) => active_path.disabled.get(),
            None => true,
          } {
            let new_paint_path = Rc::new(PaintPath::new(
              context,
              &self.paint_color.get(),
              game_mode.checkpoint.get(),
            ));
            self.paint_paths.add(new_paint_path.clone());
            self.active_path.replace(Some(new_paint_path));
          }
        }
      }

      if let Some(active_path) = self.active_path.borrow().as_ref() {
        if !active_path.disabled.get() {
          game_mode.update_path_position(
            context,
            active_path,
            &self.paw.borrow().paw_position,
            self,
            None,
          );
        }
      }

      self
        .paint_color
        .set(match self.active_path.borrow().as_ref() {
          Some(active_path) => {
            if !active_path.disabled.get() {
              active_path.paint_color.get()
            } else {
              PaintColor::NoColor
            }
          }
          None => PaintColor::NoColor,
        });
    }
  }

  fn draw(&self, context: &mut Context) {
    self
      .cached_canvas
      .check_canvas_size_changed(&context.get_canvas_size());
    if self.cached_canvas.check_clear_cache() {
      let draw_point_count = self.radius * 240.0;
      let delta_angle = 2.0 * std::f32::consts::PI / draw_point_count;

      for i in 0..(draw_point_count as i32) {
        let angle = delta_angle * (i as F1);
        let dot_position = self.position
          + F2 {
            x: angle.cos() * self.radius,
            y: angle.sin() * self.radius,
          };

        let size = context.texture_manager.dot.get_size_from_width(6.0 / 480.0);

        self.cached_canvas.draw_backend.execute_image_draw(
          &context.draw_manager.convert_viewport_into_canvas_draw_args(
            &context.game_viewport,
            DrawImageArgs {
              source: DrawSource::Texture(context.texture_manager.dot.clone()),
              position: dot_position,
              size: size,
              depth: context.draw_depths.mirror,
              optional: DrawImageOptionalArgs {
                color: DrawColor {
                  r: 78,
                  g: 46,
                  b: 16,
                },
                ..Default::default()
              },
            },
          ),
          &self.cached_canvas.canvas_size.get(),
        );
      }
    }

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Canvas(self.cached_canvas.canvas.clone()),
      position: F2 { x: 0.0, y: 0.0 },
      size: context.screen_size,
      depth: context.draw_depths.mirror,
      optional: DrawImageOptionalArgs {
        anchor_point: F2 { x: 0.0, y: 0.0 },
        opacity: context.stage_opacity.get() * 100.0 / 255.0,
        ..Default::default()
      },
    });

    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(context.texture_manager.source_moving.clone()),
        position: self.paw.borrow().paw_position,
        size: context
          .texture_manager
          .dot
          .get_size_from_width(40.0 / 480.0),
        depth: context.draw_depths.source + 0.1,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get(),
          color: self.paint_color.get().get_draw_color(),
          subpixel_precision: true,
          ..Default::default()
        },
      },
    );

    context.draw_manager.draw_viewport(
      &context.game_viewport,
      DrawImageArgs {
        source: DrawSource::Texture(context.texture_manager.source_light.clone()),
        position: self.paw.borrow().paw_position,
        size: context
          .texture_manager
          .dot
          .get_size_from_width(40.0 / 480.0),
        depth: context.draw_depths.source,
        optional: DrawImageOptionalArgs {
          opacity: context.stage_opacity.get() * 0.4,
          subpixel_precision: true,
          ..Default::default()
        },
      },
    );
  }
}

impl CircleShape for MovingSourcePaw {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.paw_position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &self.paw_radius;
  }
}

impl PaintSource for MovingSource {
  fn has_paint_left(&self) -> bool {
    return self.current_paint_amount.get() >= DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT;
  }

  fn consume_ink(&self, amount: &F1, current_checkpoint: &u32) {
    self.register_current_state(*current_checkpoint);
    self
      .current_paint_amount
      .set(self.current_paint_amount.get() - amount);
    // ASSERT(paintAmount.value >= 0, ("paint amount < 0!"));
  }
}
