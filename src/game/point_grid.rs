use super::paint_color::PaintColor;
use crate::context::Context;
use crate::engine::*;
use crate::game::paint_point::PaintPoint;
use crate::*;
use std::rc::Rc;

const GRID_WIDTH: usize = 40;
const GRID_HEIGHT: usize = 50;

pub struct PointGrid {
  entity_base: EntityBase,
  grid: RefCell<Vec<Vec<Vec<Rc<PaintPoint>>>>>,
  point_count: Cell<u32>,
  cached_canvas: CachedCanvasBackend,
  state_history: StateHistory<()>,
}

impl PointGrid {
  pub fn new(context: &Context) -> Rc<PointGrid> {
    let result = Rc::new(PointGrid {
      entity_base: EntityBase::new(),
      grid: RefCell::new(Vec::new()),
      point_count: Cell::new(0),
      cached_canvas: CachedCanvasBackend::new(&context.get_canvas_size()),
      state_history: StateHistory::new(0),
    });

    {
      let mut grid = result.grid.borrow_mut();
      for _ in 0..GRID_WIDTH {
        let mut v = Vec::new();
        for _ in 0..GRID_HEIGHT {
          v.push(Vec::new());
        }
        grid.push(v);
      }
    }
    return result;
  }

  pub fn get_grid_x(position: &F2, _: &Context) -> usize {
    let result = (position.x * (GRID_WIDTH as F1)).floor() as usize;
    if result == GRID_WIDTH {
      return GRID_WIDTH - 1;
    }
    return result;
  }

  pub fn get_grid_y(position: &F2, context: &Context) -> usize {
    let result =
      (position.y * (GRID_HEIGHT as F1) / context.game_viewport.viewport_yx_ratio).floor() as usize;
    if result == GRID_HEIGHT {
      return GRID_HEIGHT - 1;
    }
    return result;
  }

  pub fn create_point(
    &self,
    context: &Context,
    position: &F2,
    checkpoint: &u32,
    paint_color: &PaintColor,
    path_point_count: &i32,
  ) -> Rc<PaintPoint> {
    self.point_count.set(self.point_count.get() + 1);
    let paint_point = Rc::new(PaintPoint::new(
      context,
      position,
      paint_color,
      checkpoint,
      path_point_count,
      &(self.point_count.get() as F1),
    ));

    let draw_args = paint_point.draw(context, &(self.point_count.get() as F1));
    self.cached_canvas.draw_backend.execute_image_draw(
      &context
        .draw_manager
        .convert_viewport_into_canvas_draw_args(&context.game_viewport, draw_args),
      &self.cached_canvas.canvas_size.get(),
    );

    let grid_x = paint_point.grid_x;
    let grid_y = paint_point.grid_y;
    self.grid.borrow_mut()[grid_x][grid_y].push(paint_point.clone());

    // console_log_with_div!("created point!");

    return paint_point;

    // context.grid.get_paint_depth()
  }

  pub fn get_collisions(&self, being_checked: &Rc<PaintPoint>) -> Vec<Rc<PaintPoint>> {
    let mut result = Vec::new();
    let grid = self.grid.borrow();
    let grid_x = being_checked.grid_x as i32;
    let grid_y = being_checked.grid_y as i32;
    for other_grid_x in grid_x - 1..grid_x + 2 {
      for other_grid_y in grid_y - 1..grid_y + 2 {
        if other_grid_x < 0 || other_grid_x >= GRID_WIDTH as i32 {
          continue;
        }
        if other_grid_y < 0 || other_grid_y >= GRID_HEIGHT as i32 {
          continue;
        }

        for other_point in grid[other_grid_x as usize][other_grid_y as usize].iter() {
          if RcUtil::eq_ptr(other_point, being_checked) {
            continue;
          }

          if being_checked.collide_with_circle(other_point.as_ref()) {
            result.push(other_point.clone());
          }
        }
      }
    }
    return result;
  }

  pub fn get_collisions_circle_shape(
    &self,
    context: &Context,
    being_checked: &impl CircleShape,
  ) -> Vec<Rc<PaintPoint>> {
    let mut result = Vec::new();
    let grid = self.grid.borrow();
    let grid_x = PointGrid::get_grid_x(being_checked.get_center(), context) as i32;
    let grid_y = PointGrid::get_grid_y(being_checked.get_center(), context) as i32;
    for other_grid_x in grid_x - 1..grid_x + 2 {
      for other_grid_y in grid_y - 1..grid_y + 2 {
        if other_grid_x < 0 || other_grid_x >= GRID_WIDTH as i32 {
          continue;
        }
        if other_grid_y < 0 || other_grid_y >= GRID_HEIGHT as i32 {
          continue;
        }

        for other_point in grid[other_grid_x as usize][other_grid_y as usize].iter() {
          if being_checked.collide_with_circle(other_point.as_ref()) {
            result.push(other_point.clone());
          }
        }
      }
    }
    return result;
  }

  // pub fn undo(&self, current_checkpoint: &u32) {
  //   for grid_line in self.grid.borrow_mut().iter_mut() {
  //     for grid_cell in grid_line.iter_mut() {
  //       grid_cell.retain(|paint_point| paint_point.checkpoint < *current_checkpoint);
  //     }
  //   }
  //   self.cached_canvas.clear_cache_required.set(true);
  // }
}

impl EffectManagerTrait<Context> for PointGrid {
  fn get_effect_manager(&self) -> Option<&EffectManager> {
    return None;
  }
}

impl EntityTrait<Context> for PointGrid {
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

  fn undo_until_checkpoint(&self, current_checkpoint: u32) {
    for grid_line in self.grid.borrow_mut().iter_mut() {
      for grid_cell in grid_line.iter_mut() {
        grid_cell.retain(|paint_point| paint_point.checkpoint < current_checkpoint);
      }
    }
    self.cached_canvas.clear_cache_required.set(true);
  }

  fn to_remove(&self) -> bool {
    return false;
  }

  fn update(&self, _context: &mut Context) {}

  fn draw(&self, context: &mut Context) {
    self
      .cached_canvas
      .check_canvas_size_changed(&context.get_canvas_size());
    if self.cached_canvas.check_clear_cache() {
      let mut all_draw_args: Vec<DrawImageArgs> = Vec::new();
      for grid_line in self.grid.borrow().iter() {
        for grid_cell in grid_line.iter() {
          for paint_point in grid_cell.iter() {
            all_draw_args.push(paint_point.draw(context, &(self.point_count.get() as F1)))
          }
        }
      }
      all_draw_args.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap());
      for draw_args in all_draw_args {
        self.cached_canvas.draw_backend.execute_image_draw(
          &context
            .draw_manager
            .convert_viewport_into_canvas_draw_args(&context.game_viewport, draw_args),
          &self.cached_canvas.canvas_size.get(),
        );
      }
    }

    context.draw_manager.draw_screen(DrawImageArgs {
      source: DrawSource::Canvas(self.cached_canvas.canvas.clone()),
      position: F2 { x: 0.0, y: 0.0 },
      size: context.screen_size,
      depth: context.draw_depths.path,
      optional: DrawImageOptionalArgs {
        opacity: context.stage_opacity.get(),
        anchor_point: F2 { x: 0.0, y: 0.0 },
        ..Default::default()
      },
    });
  }
}
