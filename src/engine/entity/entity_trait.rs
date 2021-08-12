use crate::engine::entity::entity_base::*;
use crate::engine::*;

pub trait EntityTrait<C: ContextTrait + ?Sized>: EffectManagerTrait<C>
where
  Self: 'static,
{
  type State;
  fn get_base(&self) -> &EntityBase<C>;

  fn get_state_history(&self) -> &StateHistory<Self::State>;

  fn apply_state(&self, state: Self::State);

  fn get_current_state(&self) -> Self::State;

  fn undo_until_checkpoint(&self, _checkpoint: u32) {}

  fn register_current_state(&self, checkpoint: u32) {
    self
      .get_state_history()
      .register_state_internal(checkpoint, self.get_current_state());
    for child in self.get_base().children.borrow().iter() {
      child.register_current_state(checkpoint);
    }
  }

  fn to_remove(&self) -> bool;

  fn update(&self, _context: &mut C) {}

  fn draw(&self, _context: &mut C) {}
}
