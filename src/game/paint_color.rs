use crate::engine::*;
use std::mem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PaintColor {
  NoColor,
  Red,
  Yellow,
  Blue,
  Orange,
  Green,
  Purple,
  Gray,
}

impl PaintColor {
  pub fn get_draw_color(&self) -> DrawColor {
    return match *self {
      PaintColor::NoColor => DrawColor::new(&150, &150, &150),
      PaintColor::Red => DrawColor::new(&224, &48, &33),
      PaintColor::Yellow => DrawColor::new(&246, &229, &27),
      PaintColor::Blue => DrawColor::new(&50, &132, &219),
      PaintColor::Orange => DrawColor::new(&248, &165, &58),
      PaintColor::Green => DrawColor::new(&101, &219, &32),
      PaintColor::Purple => DrawColor::new(&216, &50, &228),
      PaintColor::Gray => DrawColor::new(&150, &150, &150),
    };
  }

  pub fn combine_with(&mut self, other: &PaintColor) {
    *self = combine_colors(self, other);
  }
}

pub fn combine_colors(c1: &PaintColor, c2: &PaintColor) -> PaintColor {
  let (mut color1, mut color2) = (*c1, *c2);
  if color1 == PaintColor::NoColor {
    return color2;
  }
  if color2 == PaintColor::NoColor {
    return color1;
  }

  if color1 == color2 {
    return color1;
  }

  if color2 == PaintColor::Red || color2 == PaintColor::Yellow || color2 == PaintColor::Blue {
    mem::swap(&mut color1, &mut color2);
  }

  return match color1 {
    PaintColor::Red => match color2 {
      PaintColor::Yellow => PaintColor::Orange,
      PaintColor::Blue => PaintColor::Purple,
      PaintColor::Orange => PaintColor::Orange,
      PaintColor::Purple => PaintColor::Purple,
      _ => PaintColor::Gray,
    },
    PaintColor::Yellow => match color2 {
      PaintColor::Red => PaintColor::Orange,
      PaintColor::Blue => PaintColor::Green,
      PaintColor::Orange => PaintColor::Orange,
      PaintColor::Green => PaintColor::Green,
      _ => PaintColor::Gray,
    },
    PaintColor::Blue => match color2 {
      PaintColor::Red => PaintColor::Purple,
      PaintColor::Yellow => PaintColor::Green,
      PaintColor::Purple => PaintColor::Purple,
      PaintColor::Green => PaintColor::Green,
      _ => PaintColor::Gray,
    },
    _ => PaintColor::Gray,
  };
}
