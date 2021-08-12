pub type F1 = f32;

pub const NEAR_EPS: F1 = 1e-4;

pub struct F1Util {}

impl F1Util {
  #[inline]
  pub fn near_eps() -> F1 {
    NEAR_EPS
  }

  pub fn move_within_range(input: &F1, min: &F1, max: &F1) -> F1 {
    if input < min {
      return *min;
    }
    if input > max {
      return *max;
    }
    return *input;
  }
}
