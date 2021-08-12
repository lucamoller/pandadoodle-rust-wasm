use crate::engine::*;

pub trait CircleShape {
  fn get_center<'a>(&'a self) -> &'a F2;
  fn get_radius<'a>(&'a self) -> &'a F1;

  fn is_point_inside(&self, point: &F2) -> bool {
    return GeometryUtils::point_inside_circle(point, self.get_center(), self.get_radius());
  }

  fn collide_with_circle(&self, other: &impl CircleShape) -> bool {
    return GeometryUtils::circle_intersect(
      self.get_center(),
      self.get_radius(),
      other.get_center(),
      other.get_radius(),
    );
  }

  fn collide_with_segment(&self, other: &impl SegmentShape) -> bool {
    return GeometryUtils::circle_segment_intersect(
      self.get_center(),
      self.get_radius(),
      other.get_p1(),
      other.get_p2(),
    );
  }
}

pub trait SegmentShape {
  fn get_p1<'a>(&'a self) -> &'a F2;
  fn get_p2<'a>(&'a self) -> &'a F2;

  fn collide_with_circle(&self, other: &impl CircleShape) -> bool
  where
    Self: Sized,
  {
    return other.collide_with_segment(self);
  }

  fn collide_with_segment(&self, other: &impl SegmentShape) -> bool
  where
    Self: Sized,
  {
    return GeometryUtils::segment_intersect(
      self.get_p1(),
      self.get_p2(),
      other.get_p1(),
      other.get_p2(),
    )
    .is_some();
  }

  fn get_intersection_with_segment(&self, other: &impl SegmentShape) -> Option<F2>
  where
    Self: Sized,
  {
    return GeometryUtils::segment_intersect(
      self.get_p1(),
      self.get_p2(),
      other.get_p1(),
      other.get_p2(),
    );
  }
}
