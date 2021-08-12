use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;
use std::ops::Deref;

pub struct UiContainer<C: ContextTrait + ?Sized> {
  pub active: Cell<bool>,
  child_elements: RefCell<Vec<Rc<dyn UiElementTrait<C>>>>,
  element: UiElement<C>,
  weak_self: RefCell<Weak<Self>>,
}

impl<C: ContextTrait + ?Sized> UiContainer<C> {
  pub fn new() -> Rc<UiContainer<C>> {
    let result = Rc::new(UiContainer {
      active: Cell::new(true),
      child_elements: RefCell::new(Vec::new()),
      element: UiElement::new(),
      weak_self: RefCell::new(Weak::new()),
    });
    *result.weak_self.borrow_mut() = Rc::downgrade(&result);
    return result;
  }

  pub fn add_child(&self, child_element: Rc<dyn UiElementTrait<C>>) {
    child_element.set_parent(self.weak_self.borrow().clone());
    self.child_elements.borrow_mut().push(child_element);
  }

  pub fn set_active(&self, active: bool) {
    self.active.set(active);
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiContainer<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiContainer<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return &self.element;
  }

  fn update(&self, context: &mut C) {
    for child in self.child_elements.borrow().iter() {
      child.update(context);
      child.update_effects(context);
    }
  }

  fn draw(&self, context: &mut C) {
    for child in self.child_elements.borrow().iter() {
      child.draw(context);
    }
  }

  fn get_touched_element(
    &self,
    context: &mut C,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<C>>> {
    let absolute_params = self.get_absolute_params(context);
    if !absolute_params.visible || !self.active.get() {
      return None;
    }

    for child in self.child_elements.borrow().iter() {
      let touched_child_element = child.get_touched_element(context, ui_touch);
      if touched_child_element.is_some() {
        return touched_child_element;
      }
    }
    return None;
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiContainer<C> {
  type Target = UiElement<C>;

  fn deref(&self) -> &Self::Target {
    return &self.element;
  }
}
