use crate::engine::*;

pub struct SetEffect<C: ContextTrait + ?Sized> {
  effects: RefCell<Vec<(Rc<dyn EffectTrait<C>>, F1)>>,
  current_time: Cell<F1>,
}

impl<C: ContextTrait + ?Sized> SetEffect<C> {
  pub fn new() -> SetEffect<C> {
    return SetEffect {
      effects: RefCell::new(Vec::new()),
      current_time: Cell::new(0.0),
    };
  }

  pub fn add_effect(&self, effect: Rc<dyn EffectTrait<C>>, start_time: F1) {
    self.effects.borrow_mut().push((effect, start_time));
  }
}

impl<C: ContextTrait + ?Sized> EffectImpl<C> for SetEffect<C> {
  fn inner_start(&self) {
    self.current_time.set(0.0);
  }

  fn inner_update(&self, dt: &F1, context: &mut C) -> (bool, F1) {
    let last_time = self.current_time.get();
    self.current_time.set(self.current_time.get() + dt);

    let mut all_finished = true;
    let mut remaining_dt = *dt;

    for (effect, start_time) in self.effects.borrow().iter() {
      if *start_time >= self.current_time.get() {
        all_finished = false;
        continue;
      }
      if *start_time >= last_time && *start_time < self.current_time.get() {
        effect.start();
      }

      let (current_finished, new_remaining_dt) = effect.update(*dt, context);
      all_finished = all_finished && current_finished;
      remaining_dt = if new_remaining_dt < remaining_dt {
        new_remaining_dt
      } else {
        remaining_dt
      };
    }
    return (all_finished, remaining_dt);
  }

  fn inner_set_end_state(&self) {
    for (effect, _) in self.effects.borrow().iter() {
      effect.set_end_state();
    }
  }
}
