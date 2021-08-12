use crate::engine::types::f1::*;
use std::ops;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct F2 {
  pub x: F1,
  pub y: F1,
}

impl ops::Add<&F2> for &F2 {
  type Output = F2;
  fn add(self, rhs: &F2) -> F2 {
    return F2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }
}

impl ops::Add<F2> for &F2 {
  type Output = F2;
  fn add(self, rhs: F2) -> F2 {
    return self + &rhs;
  }
}

impl ops::Add<&F2> for F2 {
  type Output = F2;
  fn add(self, rhs: &F2) -> F2 {
    return &self + rhs;
  }
}

impl ops::Add<F2> for F2 {
  type Output = F2;
  fn add(self, rhs: F2) -> F2 {
    return &self + &rhs;
  }
}

impl ops::AddAssign<&F2> for F2 {
  fn add_assign(&mut self, other: &F2) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl ops::Sub<&F2> for &F2 {
  type Output = F2;
  fn sub(self, rhs: &F2) -> F2 {
    return F2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}

impl ops::Sub<&F2> for F2 {
  type Output = F2;
  fn sub(self, rhs: &F2) -> F2 {
    return &self - rhs;
  }
}

impl ops::Sub<F2> for &F2 {
  type Output = F2;
  fn sub(self, rhs: F2) -> F2 {
    return self - &rhs;
  }
}

impl ops::Sub<F2> for F2 {
  type Output = F2;
  fn sub(self, rhs: F2) -> F2 {
    return &self - &rhs;
  }
}

impl ops::SubAssign<&F2> for F2 {
  fn sub_assign(&mut self, other: &F2) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl ops::Mul<&F1> for &F2 {
  type Output = F2;

  fn mul(self, rhs: &F1) -> F2 {
    return F2 {
      x: self.x * rhs,
      y: self.y * rhs,
    };
  }
}

impl ops::Mul<F1> for &F2 {
  type Output = F2;

  fn mul(self, rhs: F1) -> F2 {
    return self * &rhs;
  }
}

impl ops::Mul<&F1> for F2 {
  type Output = F2;

  fn mul(self, rhs: &F1) -> F2 {
    return &self * rhs;
  }
}

impl ops::Mul<F1> for F2 {
  type Output = F2;

  fn mul(self, rhs: F1) -> F2 {
    return &self * &rhs;
  }
}

impl ops::Mul<F2> for F1 {
  type Output = F2;

  fn mul(self, rhs: F2) -> F2 {
    return &rhs * &self;
  }
}

impl ops::MulAssign<&F1> for F2 {
  fn mul_assign(&mut self, other: &F1) {
    self.x *= other;
    self.y *= other;
  }
}

impl F2 {
  pub fn dotp(a: &F2, b: &F2) -> F1 {
    return a.x * b.x + a.y * b.y;
  }
  pub fn crossp(a: &F2, b: &F2) -> F1 {
    return a.x * b.y - b.x * a.y;
  }
  pub fn distance(a: &F2, b: &F2) -> F1 {
    let diff = b - a;
    return diff.length();
  }
  pub fn distance2(a: &F2, b: &F2) -> F1 {
    let diff = b - a;
    return diff.length2();
  }
  pub fn rotate_new(v: &F2, angle: &F1) -> F2 {
    let mut result = v.clone();
    result.rotate(angle);
    return result;
  }

  pub fn length2(&self) -> F1 {
    return F2::dotp(self, self);
  }

  pub fn length(&self) -> F1 {
    return self.length2().sqrt();
  }

  pub fn normalize(&mut self) {
    let length = self.length();
    self.x /= length;
    self.y /= length;
  }

  pub fn rotate(&mut self, angle: &F1) {
    let cos = angle.cos();
    let sin = angle.sin();
    let tmp = self.x * cos - self.y * sin;
    self.y = self.x * sin + self.y * cos;
    self.x = tmp;
  }

  pub fn round(&mut self) {
    self.x = self.x.round();
    self.y = self.y.round();
  }

  pub fn eq_near(&self, other: &F2) -> bool {
    let diff_x = self.x - other.x;
    let diff_y = self.y - other.y;
    return (diff_x.abs() < F1Util::near_eps()) && (diff_y.abs() < F1Util::near_eps());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
    let v1 = F2 { x: 1.0, y: 3.0 };
    let v2 = F2 { x: 2.0, y: 4.0 };
    let sum = &v1 + &v2;
    assert_eq!(sum.eq_near(&F2 { x: 3.0, y: 7.0 }), true);
  }

  #[test]
  fn test_sub() {
    let v1 = F2 { x: 4.0, y: 5.0 };
    let v2 = F2 { x: 2.0, y: 1.0 };
    let sum = &v1 - &v2;
    assert_eq!(sum.eq_near(&F2 { x: 2.0, y: 4.0 }), true);
  }
}
