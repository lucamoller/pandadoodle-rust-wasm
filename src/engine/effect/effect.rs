use crate::engine::effect::*;
use crate::engine::*;

pub trait EffectTrait<C: ContextTrait + ?Sized> {
  fn start(&self);
  fn set_looped(&self, looped: bool);
  fn update(&self, remaining_dt: F1, context: &mut C) -> (bool, F1);
  fn set_end_state(&self);
}

pub trait EffectImpl<C: ContextTrait + ?Sized>
where
  Self: 'static,
{
  fn inner_start(&self);
  fn inner_update(&self, remaining_dt: &F1, context: &mut C) -> (bool, F1);
  fn inner_set_end_state(&self);
}

pub struct Effect<C: ContextTrait + ?Sized, T: EffectImpl<C>> {
  pub effect_impl: T,

  pub active: Cell<bool>,
  looped: Cell<bool>,
  pub end_event: Event1ArgMut<C>,
}

impl<C: ContextTrait + ?Sized, T: EffectImpl<C>> Effect<C, T> {
  fn new(effect_impl: T) -> Effect<C, T> {
    Effect {
      effect_impl: effect_impl,
      active: Cell::new(false),
      looped: Cell::new(false),
      end_event: Event1ArgMut::empty(),
    }
  }

  pub fn new_within_effect_manager(
    effect_impl: T,
    effect_manager: &EffectManager<C>,
  ) -> Rc<Effect<C, T>> {
    let result = Rc::new(Effect::new(effect_impl));
    effect_manager.add_managed_effect(result.clone());
    return result;
  }

  pub fn new_within_chained_effect(
    effect_impl: T,
    chained_effect: &ChainedEffect<C>,
  ) -> Rc<Effect<C, T>> {
    let result = Rc::new(Effect::new(effect_impl));
    chained_effect.add_effect(result.clone());
    return result;
  }

  pub fn new_within_set_effect(
    effect_impl: T,
    start_time: F1,
    set_effect: &SetEffect<C>,
  ) -> Rc<Effect<C, T>> {
    let result = Rc::new(Effect::new(effect_impl));
    set_effect.add_effect(result.clone(), start_time);
    return result;
  }

  pub fn stop(&self) {
    self.active.set(false);
  }

  pub fn add_event_on_end<E: 'static + Clone>(&self, event_manager: Rc<EventManager<E>>, event: E) {
    self.end_event.add(Box::new(move |_context| {
      event_manager.add_event(event.clone());
    }));
  }
}

impl<C: ContextTrait + ?Sized, T: EffectImpl<C>> EffectTrait<C> for Effect<C, T> {
  fn start(&self) {
    self.active.set(true);
    self.effect_impl.inner_start();
  }

  fn update(&self, remaining_dt: F1, context: &mut C) -> (bool, F1) {
    if !self.active.get() {
      return (true, 0.0);
    }

    let (finished, remaining_dt) = self.effect_impl.inner_update(&remaining_dt, context);
    if finished {
      self.end_event.execute(context);
      if self.looped.get() {
        self.start();
        return self.update(remaining_dt, context);
      } else {
        self.active.set(false);
      }
    }
    return (finished, remaining_dt);
  }

  fn set_looped(&self, looped: bool) {
    self.looped.set(looped);
  }

  fn set_end_state(&self) {
    self.effect_impl.inner_set_end_state();
  }
}

impl<C: ContextTrait + ?Sized, T: EffectImpl<C> + 'static> Deref for Effect<C, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    return &self.effect_impl;
  }
}
