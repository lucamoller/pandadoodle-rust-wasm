use crate::engine::effect::*;
use crate::engine::*;

pub struct ChainedEffect<C: ContextTrait + ?Sized> {
  effects: RefCell<Vec<Rc<dyn EffectTrait<C>>>>,
  current: Cell<usize>,
}

impl<C: ContextTrait + ?Sized> ChainedEffect<C> {
  pub fn new() -> ChainedEffect<C> {
    return ChainedEffect {
      effects: RefCell::new(Vec::new()),
      current: Cell::new(0),
    };
  }

  pub fn add_effect(&self, effect: Rc<dyn EffectTrait<C>>) {
    self.effects.borrow_mut().push(effect);
  }

  fn has_finished(&self) -> bool {
    return self.current.get() >= self.effects.borrow().len();
  }

  fn start_current(&self) {
    if !self.has_finished() {
      self.effects.borrow()[self.current.get()].start();
    }
  }
}

impl<C: ContextTrait + ?Sized> EffectImpl<C> for ChainedEffect<C> {
  fn inner_start(&self) {
    self.current.set(0);
    self.start_current();
  }

  fn inner_update(&self, remaining_dt: &F1, context: &mut C) -> (bool, F1) {
    let mut remaining_dt = *remaining_dt;
    while !self.has_finished() && remaining_dt > 0.0 {
      let (current_finished, new_remaining_dt) =
        self.effects.borrow()[self.current.get()].update(remaining_dt, context);
      remaining_dt = new_remaining_dt;
      if current_finished {
        self.current.set(self.current.get() + 1);
        self.start_current();
      }
    }
    return (self.has_finished(), remaining_dt);
  }

  fn inner_set_end_state(&self) {
    for effect in self.effects.borrow().iter() {
      effect.set_end_state();
    }
    self.current.set(self.effects.borrow().len());
  }
}
