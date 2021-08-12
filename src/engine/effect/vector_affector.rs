use crate::engine::*;
use std::ops;

pub type VectorAffectorF1 = VectorAffector<F1>;
pub type VectorAffectorF2 = VectorAffector<F2>;

pub trait Vector = where
  Self: 'static,
  Self: Default + Copy + std::fmt::Debug,
  Self: ops::Add<Self, Output = Self>,
  Self: ops::Sub<Self, Output = Self>,
  Self: ops::Mul<F1, Output = Self>,
  for<'a> &'a Self: ops::Add<Self, Output = Self>,
  for<'a> &'a Self: ops::Sub<Self, Output = Self>,
  for<'a> &'a Self: ops::Mul<F1, Output = Self>,
  for<'a> Self: ops::Add<&'a Self, Output = Self>,
  for<'a> Self: ops::Sub<&'a Self, Output = Self>,
  for<'a> Self: ops::Mul<&'a F1, Output = Self>,
  for<'a, 'b> &'a Self: ops::Add<&'b Self, Output = Self>,
  for<'a, 'b> &'a Self: ops::Sub<&'b Self, Output = Self>,
  for<'a, 'b> &'a Self: ops::Mul<&'b F1, Output = Self>;

pub struct VectorAffector<T: Vector> {
  target: Shared<T>,
  current_time: Cell<F1>,
  total_time: Cell<F1>,

  start: Cell<T>,
  end: Cell<T>,
  start_from_current: Cell<bool>,
  progression: RefCell<Box<dyn RatioProgressionTrait>>,
}

impl<T: Vector> VectorAffector<T> {
  pub fn new(target: Shared<T>) -> VectorAffector<T> {
    return VectorAffector {
      target: target,
      current_time: Cell::new(0.0),
      total_time: Cell::new(0.0),

      start: Cell::new(T::default()),
      end: Cell::new(T::default()),
      start_from_current: Cell::new(false),
      progression: RefCell::new(Box::new(LinearProgression::new())),
    };
  }

  pub fn set_progression_onref(&self, progression: Box<dyn RatioProgressionTrait>) {
    self.progression.replace(progression);
  }

  pub fn set_progression(self, progression: Box<dyn RatioProgressionTrait>) -> Self {
    self.set_progression_onref(progression);
    return self;
  }

  pub fn get_fraction(&self) -> F1 {
    let ratio = self.current_time.get() / self.total_time.get();
    return self.progression.borrow().get_value(ratio);
  }

  pub fn set_start_and_end_onref(&self, start: T, end: T, total_time: F1) {
    self.start_from_current.set(false);
    self.start.set(start);
    self.end.set(end);
    self.total_time.set(total_time);
  }

  pub fn set_start_and_end(self, start: T, end: T, total_time: F1) -> Self {
    self.set_start_and_end_onref(start, end, total_time);
    return self;
  }

  pub fn set_end_onref(&self, end: T, total_time: F1) {
    self.start_from_current.set(true);
    self.end.set(end);
    self.total_time.set(total_time);
  }

  pub fn set_end(self, end: T, total_time: F1) -> Self {
    self.set_end_onref(end, total_time);
    return self;
  }
}

impl<C: ContextTrait + ?Sized, T: Vector> EffectImpl<C> for VectorAffector<T> {
  fn inner_start(&self) {
    self.current_time.set(0.0);
    if self.start_from_current.get() {
      self.start.set(*self.target.borrow());
    } else {
      *self.target.borrow_mut() = self.start.get();
    }
  }

  fn inner_update(&self, dt: &F1, _context: &mut C) -> (bool, F1) {
    self.current_time.set(self.current_time.get() + dt);
    let mut remaining_dt = 0.0;

    if self.current_time.get() >= self.total_time.get() {
      remaining_dt = self.total_time.get() - self.current_time.get();
      self.current_time.set(self.total_time.get());
    }

    *self.target.borrow_mut() =
      &self.start.get() + ((self.end.get() - self.start.get()) * self.get_fraction());
    return (
      self.current_time.get() == self.total_time.get(),
      remaining_dt,
    );
  }

  fn inner_set_end_state(&self) {
    *self.target.borrow_mut() = self.end.get();
  }
}
