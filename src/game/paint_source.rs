use crate::engine::*;

pub trait PaintSource {
  fn has_paint_left(&self) -> bool;
  fn consume_ink(&self, amount: &F1, current_checkpoint: &u32);
}
