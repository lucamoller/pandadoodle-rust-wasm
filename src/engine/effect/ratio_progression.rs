use crate::engine::*;

pub trait RatioProgressionTrait {
  fn get_value(&self, ratio: F1) -> F1;
}

pub struct LinearProgression {}

impl LinearProgression {
  pub fn new() -> LinearProgression {
    return LinearProgression {};
  }
}

impl RatioProgressionTrait for LinearProgression {
  fn get_value(&self, ratio: F1) -> F1 {
    return ratio;
  }
}

pub struct QuadraticProgression {}

impl QuadraticProgression {
  pub fn new() -> QuadraticProgression {
    return QuadraticProgression {};
  }
}

impl RatioProgressionTrait for QuadraticProgression {
  fn get_value(&self, ratio: F1) -> F1 {
    return ratio * ratio;
  }
}

pub struct ExpProgression {
  base: F1,
  exponent: F1,
  denominator: F1,
}

impl ExpProgression {
  pub fn new(base: F1, exponent: F1) -> ExpProgression {
    return ExpProgression {
      base: base,
      exponent: exponent,
      denominator: base.powf(exponent) - 1.0,
    };
  }
}

impl RatioProgressionTrait for ExpProgression {
  fn get_value(&self, ratio: F1) -> F1 {
    return (self.base.powf(ratio * self.exponent) - 1.0) / self.denominator;
  }
}

pub struct ExpTransProgression {
  base: F1,
  exponent: F1,
  denominator: F1,
}

impl ExpTransProgression {
  pub fn new(base: F1, exponent: F1) -> ExpTransProgression {
    return ExpTransProgression {
      base: base,
      exponent: exponent,
      denominator: base.powf(exponent) - 1.0,
    };
  }
}

impl RatioProgressionTrait for ExpTransProgression {
  fn get_value(&self, ratio: F1) -> F1 {
    return 1.0 - ((self.base.powf((1.0 - ratio) * self.exponent) - 1.0) / self.denominator);
  }
}
