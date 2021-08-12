use crate::engine::entity::entity_manager::*;
use crate::engine::*;

pub struct EntityBase<C: ContextTrait + ?Sized> {
  pub children: RefCell<Vec<Rc<dyn EntityManagerTrait<C>>>>,
}

impl<C: ContextTrait + ?Sized> EntityBase<C> {
  pub fn new() -> EntityBase<C> {
    return EntityBase {
      children: RefCell::default(),
    };
  }
}
