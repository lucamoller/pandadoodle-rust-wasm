use crate::engine::entity::entity_base::*;
use crate::engine::*;

pub trait EntityManagerTrait<C: ContextTrait + ?Sized> {
  fn undo(&self, current_checkpoint: &u32);
  fn register_current_state(&self, checkpoint: u32);
  fn update(&self, context: &mut C);
  fn draw(&self, context: &mut C);
}

pub struct EntityManager<C: ContextTrait + ?Sized, T: EntityTrait<C>> {
  pub managed_entities: RefCell<Vec<Rc<T>>>,
  pub context_type: PhantomData<C>,
}

impl<C: ContextTrait + ?Sized, T: EntityTrait<C>> Clone for EntityManager<C, T> {
  fn clone(&self) -> EntityManager<C, T> {
    return EntityManager {
      managed_entities: RefCell::new(self.managed_entities.borrow().clone()),
      context_type: PhantomData,
    };
  }
}

impl<C: ContextTrait + ?Sized, T: EntityTrait<C>> EntityManager<C, T> {
  fn new() -> EntityManager<C, T> {
    return EntityManager {
      managed_entities: RefCell::default(),
      context_type: PhantomData,
    };
  }

  pub fn new_within_parent_entity(parent_entity_base: &EntityBase<C>) -> Rc<EntityManager<C, T>> {
    let result = Rc::new(EntityManager::new());
    parent_entity_base
      .children
      .borrow_mut()
      .push(result.clone());
    return result;
  }

  pub fn new_root_manager() -> EntityManager<C, T> {
    return EntityManager::new();
  }

  pub fn add(&self, entity: Rc<T>) {
    self.managed_entities.borrow_mut().push(entity);
  }

  pub fn clear(&self) {
    self.managed_entities.borrow_mut().clear();
  }

  pub fn replace(&self, other: EntityManager<C, T>) {
    self
      .managed_entities
      .replace(other.managed_entities.borrow().clone());
  }
}

impl<C: ContextTrait + ?Sized, T: EntityTrait<C>> EntityManagerTrait<C> for EntityManager<C, T> {
  fn undo(&self, current_checkpoint: &u32) {
    self.managed_entities.borrow_mut().retain(|entity| {
      entity
        .get_state_history()
        .exists_before(&current_checkpoint)
    });

    for entity in self.managed_entities.borrow().iter() {
      if let Some(target_state) = entity
        .get_state_history()
        .rollback_changes(current_checkpoint)
      {
        entity.apply_state(target_state);
      }
      entity.undo_until_checkpoint(*current_checkpoint);

      for child in entity.get_base().children.borrow().iter() {
        child.undo(current_checkpoint);
      }
    }
  }

  fn register_current_state(&self, checkpoint: u32) {
    for entity in self.managed_entities.borrow().iter() {
      entity.register_current_state(checkpoint);
    }
  }

  fn update(&self, context: &mut C) {
    for entity in self.managed_entities.borrow_mut().iter_mut() {
      entity.update(context);
      entity.update_effects(context);

      for child in entity.get_base().children.borrow().iter() {
        child.update(context);
      }
    }

    self
      .managed_entities
      .borrow_mut()
      .retain(|entity| !entity.to_remove());
  }

  fn draw(&self, context: &mut C) {
    for entity in self.managed_entities.borrow_mut().iter() {
      entity.draw(context);
      for child in entity.get_base().children.borrow().iter() {
        child.draw(context);
      }
    }
  }
}
