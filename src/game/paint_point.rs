use super::super::context::Context;
use super::paint_color::PaintColor;
use super::point_grid::PointGrid;
use crate::engine::*;

pub const POINT_RADIUS: F1 = 0.0100;
pub const DISTANCE_BETWEEN_POINTS: F1 = 0.0095;
pub const DISTANCE_SQUARED_BETWEEN_POINTS: F1 = 0.00009025;
pub const DISTANCE_BETWEEN_POINTS_AMOUNT_OF_PAINT: F1 = 4.0;
pub const DISTANCE_SQUARED_TO_REACTIVATE_PATH: F1 = 0.0020;

pub struct PaintPoint {
  pub position: F2,
  pub grid_x: usize,
  pub grid_y: usize,

  pub paint_color: PaintColor,
  pub checkpoint: u32,
  paint_depth: F1,
  rotation: F1,
  size: F2,
}

impl PaintPoint {
  pub fn new(
    context: &Context,
    position: &F2,
    paint_color: &PaintColor,
    checkpoint: &u32,
    sin_scale_iterator: &i32,
    paint_depth: &F1,
  ) -> PaintPoint {
    let width =
      0.09 * (0.4 + 0.13 * (30.0 * (*sin_scale_iterator as F1) * DISTANCE_BETWEEN_POINTS).sin());
    return PaintPoint {
      position: *position,
      grid_x: PointGrid::get_grid_x(position, context),
      grid_y: PointGrid::get_grid_y(position, context),

      paint_color: *paint_color,
      checkpoint: *checkpoint,
      paint_depth: *paint_depth,
      rotation: 0.1 * (*sin_scale_iterator as F1),
      size: context.texture_manager.mancha.get_size_from_width(width),
    };
  }

  pub fn calculate_depth(&self, context: &Context, total_point_depth: &F1) -> F1 {
    return context.draw_depths.path + ((total_point_depth - self.paint_depth) / total_point_depth);
  }

  pub fn draw(&self, context: &Context, total_point_depth: &F1) -> DrawImageArgs {
    return DrawImageArgs {
      source: DrawSource::Texture(context.texture_manager.mancha.clone()),
      position: self.position,
      size: self.size,
      depth: self.calculate_depth(context, total_point_depth),
      optional: DrawImageOptionalArgs {
        color: self.paint_color.get_draw_color(),
        rotation: self.rotation,
        ..Default::default()
      },
    };
  }
}

impl CircleShape for PaintPoint {
  fn get_center<'a>(&'a self) -> &'a F2 {
    return &self.position;
  }

  fn get_radius<'a>(&'a self) -> &'a F1 {
    return &POINT_RADIUS;
  }
}
