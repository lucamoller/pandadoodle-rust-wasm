use crate::engine::*;

pub struct EffectManager<C: ContextTrait + ?Sized> {
  effects: RefCell<Vec<Rc<dyn EffectTrait<C>>>>,
}

impl<C: ContextTrait + ?Sized> EffectManager<C> {
  pub fn new() -> EffectManager<C> {
    return EffectManager {
      effects: RefCell::new(Vec::new()),
    };
  }

  pub fn add_managed_effect(&self, effect: Rc<dyn EffectTrait<C>>) {
    self.effects.borrow_mut().push(effect);
  }
}

pub trait EffectManagerTrait<C: ContextTrait + ?Sized> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>>;

  fn update_effects(&self, context: &mut C) {
    if let Some(effect_manager) = self.get_effect_manager() {
      for effect in effect_manager.effects.borrow().iter() {
        effect.update(*context.get_dt(), context);
      }
    }
  }
}
