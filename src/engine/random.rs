use crate::engine::*;
use rand::prelude::*;

pub fn get_random_from_0_to_1() -> F1 {
  let max_u32 = std::u32::MAX as f64;
  let rand_value = rand::thread_rng().next_u32() as f64;
  return (rand_value / max_u32) as F1;
}

pub fn get_random_in_interval(start: &F1, end: &F1) -> F1 {
  let range = (end - start) as F1;
  return start + get_random_from_0_to_1() * range;
}

pub fn get_random_from_minus1_to_1() -> F1 {
  return get_random_in_interval(&-1.0, &1.0);
}

pub fn get_random_in_rectangular_region(center: &F2, size: &F2) -> F2 {
  return F2 {
    x: center.x + get_random_in_interval(&(-0.5 * size.x), &(0.5 * size.x)),
    y: center.y + get_random_in_interval(&(-0.5 * size.y), &(0.5 * size.y)),
  };
}
