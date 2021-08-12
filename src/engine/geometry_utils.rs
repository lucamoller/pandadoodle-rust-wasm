use crate::engine::*;

pub struct GeometryUtils {
  _not_instatiable: (),
}

impl GeometryUtils {
  pub fn cmp_f1(a: &F1, b: &F1) -> i32 {
    if *a + F1Util::near_eps() < *b {
      return -1;
    }
    if *b + F1Util::near_eps() < *a {
      return 1;
    }
    return 0;
  }
  pub fn cmp_f2(a: &F2, b: &F2) -> i32 {
    let cmpx = GeometryUtils::cmp_f1(&a.x, &b.x);
    if cmpx != 0 {
      return cmpx;
    }
    return GeometryUtils::cmp_f1(&a.y, &b.y);
  }
  pub fn point_inside_circle(point: &F2, circle_center: &F2, circle_radius: &F1) -> bool {
    let diff = point - circle_center;
    return GeometryUtils::cmp_f1(&F2::dotp(&diff, &diff), &(circle_radius * circle_radius)) <= 0;
  }
  pub fn circle_intersect(
    circle_center_1: &F2,
    circle_radius_1: &F1,
    circle_center_2: &F2,
    circle_radius_2: &F1,
  ) -> bool {
    let radius_sum = circle_radius_1 + circle_radius_2;
    let diff = circle_center_1 - circle_center_2;
    return GeometryUtils::cmp_f1(&F2::dotp(&diff, &diff), &(radius_sum * radius_sum)) <= 0;
  }
  pub fn distance_squared_point_to_segment(point: &F2, seg_p1: &F2, seg_p2: &F2) -> F1 {
    let diff_p_p1 = point - seg_p1;
    let dist_squared_p_p1 = F2::dotp(&diff_p_p1, &diff_p_p1);
    let diff_p_p2 = point - seg_p2;
    let dist_squared_p_p2 = F2::dotp(&diff_p_p2, &diff_p_p2);
    let diff_seg = seg_p1 - seg_p2;
    let dist_squared_seg = F2::dotp(&diff_seg, &diff_seg);
    // In case seg_p1 is the closest point.
    if GeometryUtils::cmp_f1(&dist_squared_p_p2, &(dist_squared_p_p1 + dist_squared_seg)) >= 0 {
      return dist_squared_p_p1;
    }
    // In case seg_p2 is the closest point
    if GeometryUtils::cmp_f1(&dist_squared_p_p1, &(dist_squared_p_p2 + dist_squared_seg)) >= 0 {
      return dist_squared_p_p2;
    }
    // Closest point is somewhere in the middle of the segment
    let cross_p_p1_p_p2 = F2::crossp(&diff_p_p1, &diff_p_p2);
    return cross_p_p1_p_p2 * cross_p_p1_p_p2 / dist_squared_seg;
  }
  pub fn circle_segment_intersect(
    circle_center: &F2,
    circle_radius: &F1,
    seg_p1: &F2,
    seg_p2: &F2,
  ) -> bool {
    let distance_squared =
      GeometryUtils::distance_squared_point_to_segment(circle_center, seg_p1, seg_p2);
    return GeometryUtils::cmp_f1(&distance_squared, &(circle_radius * circle_radius)) <= 0;
  }
  pub fn line_intersect(p1: &F2, p2: &F2, q1: &F2, q2: &F2) -> Option<F2> {
    let a = p2 - p1;
    let b = q2 - q1;
    let cross_ab = F2::crossp(&a, &b);
    if GeometryUtils::cmp_f1(&cross_ab, &0.0) == 0 {
      return None;
    }
    let c = F2 {
      x: F2::crossp(p1, p2),
      y: F2::crossp(q1, q2),
    };
    return Some(F2 {
      x: F2::crossp(&F2 { x: a.x, y: b.x }, &c) / cross_ab,
      y: F2::crossp(&F2 { x: a.y, y: b.y }, &c) / cross_ab,
    });
  }
  pub fn segment_intersect(p1: &F2, p2: &F2, q1: &F2, q2: &F2) -> Option<F2> {
    let a = p2 - p1;
    let b = q2 - q1;
    let c = q1 - p1;
    let d = q2 - p2;
    let v1 = GeometryUtils::cmp_f1(&F2::crossp(&a, &c), &0.0)
      + 2 * GeometryUtils::cmp_f1(&F2::crossp(&a, &d), &0.0);
    let v2 = GeometryUtils::cmp_f1(&F2::crossp(&b, &c), &0.0)
      + 2 * GeometryUtils::cmp_f1(&F2::crossp(&b, &d), &0.0);
    // Case there are 2 points on the same side of one of the support lines, they don't intersect
    if v1 == 3 || v1 == -3 || v2 == 3 || v2 == -3 {
      return None;
    }
    // Case the cross products weren't all zero and didn't fit in the case before,
    // or any of the end points of the segments are the same, they intersect
    if p1.eq_near(q1) || p1.eq_near(q2) {
      return Some(*p1);
    }
    if p2.eq_near(q1) || p2.eq_near(q2) {
      return Some(*p2);
    }
    if v1 != 0 || v2 != 0 {
      return GeometryUtils::line_intersect(p1, p2, q1, q2);
    }
    // Now remains the case that all cross products were zero, so the points were all colinear
    let v3 = GeometryUtils::cmp_f2(p1, q1)
      + GeometryUtils::cmp_f2(p1, q2)
      + GeometryUtils::cmp_f2(p2, q1)
      + GeometryUtils::cmp_f2(p2, q2);
    // Then we sorted the points, if p1 and p2 are on the same side from q1 and q2, the result
    // will be -4 or 4
    if v3 == -4 || v3 == 4 {
      return None;
    }
    // Now we know the segments intersect, but in an infinite amount points. Let's just calculate
    // one of them. In this case, at least one of p1 or p2 is in the intersection. If p1 is not, then p2 is.
    return if GeometryUtils::cmp_f2(p1, q1) != GeometryUtils::cmp_f2(p1, q2) {
      Some(*p1)
    } else {
      Some(*p2)
    };
  }
}
