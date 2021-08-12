use crate::engine::*;

pub struct WaitAffector {
  current_time: Cell<F1>,
  total_time: F1,
}

impl WaitAffector {
  pub fn new(total_time: F1) -> WaitAffector {
    return WaitAffector {
      current_time: Cell::new(0.0),
      total_time: total_time,
    };
  }
}

impl<C: ContextTrait> EffectImpl<C> for WaitAffector {
  fn inner_start(&self) {
    self.current_time.set(0.0);
  }

  fn inner_update(&self, dt: &F1, _context: &mut C) -> (bool, F1) {
    self.current_time.set(self.current_time.get() + dt);
    let mut remaining_dt = 0.0;

    if self.current_time.get() >= self.total_time {
      remaining_dt = self.total_time - self.current_time.get();
      self.current_time.set(self.total_time);
    }

    return (self.current_time.get() == self.total_time, remaining_dt);
  }

  fn inner_set_end_state(&self) {}
}
