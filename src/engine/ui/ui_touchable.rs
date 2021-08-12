use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;
use std::ops::Deref;

pub struct UiTouchable<C: ContextTrait + ?Sized> {
  pub container: Rc<UiContainer<C>>,
  pub size: Shared<F2>,
  pub extension_size: Shared<F2>,
  pub pressed: Cell<bool>,

  pub on_pressed_event: Event2ArgMutRef<C, UiTouch>,
  pub on_released_event: Event2ArgMutRef<C, UiTouch>,
  pub on_moved_event: Event2ArgMutRef<C, UiTouch>,

  pub weak_self: RefCell<Weak<UiTouchable<C>>>,
  pub weak_impl: RefCell<Option<Weak<dyn UiElementTrait<C>>>>,
}

impl<C: ContextTrait + ?Sized> UiTouchable<C> {
  pub fn new() -> Rc<UiTouchable<C>> {
    let result = Rc::new(UiTouchable {
      container: UiContainer::new(),
      size: Shared::default(),
      extension_size: Shared::default(),
      pressed: Cell::new(false),
      on_pressed_event: Event2ArgMutRef::empty(),
      on_released_event: Event2ArgMutRef::empty(),
      on_moved_event: Event2ArgMutRef::empty(),
      weak_self: RefCell::new(Weak::new()),
      weak_impl: RefCell::new(None),
    });
    *result.weak_self.borrow_mut() = Rc::downgrade(&result);
    return result;
  }

  pub fn check_within_bounds(
    &self,
    absolute_params: &AbsoluteUiElementParams,
    ui_touch: &UiTouch,
  ) -> bool {
    let total_size = (*self.size.borrow() + *self.extension_size.borrow()) * 0.5;

    return ui_touch.position.x >= absolute_params.position.x - total_size.x
      && ui_touch.position.x <= absolute_params.position.x + total_size.x
      && ui_touch.position.y >= absolute_params.position.y - total_size.y
      && ui_touch.position.y <= absolute_params.position.y + total_size.y;
  }

  pub fn set_size(&self, size: F2) {
    *self.size.borrow_mut() = size;
  }
  pub fn set_size_x(&self, size_x: F1) {
    self.size.borrow_mut().x = size_x;
  }
  pub fn set_size_y(&self, size_y: F1) {
    self.size.borrow_mut().y = size_y;
  }

  pub fn set_extension_size(&self, extension_size: F2) {
    *self.extension_size.borrow_mut() = extension_size;
  }
  pub fn set_extension_size_x(&self, extension_size_x: F1) {
    self.extension_size.borrow_mut().x = extension_size_x;
  }
  pub fn set_extension_size_y(&self, extension_size_y: F1) {
    self.extension_size.borrow_mut().y = extension_size_y;
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiTouchable<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiTouchable<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return self.container.get_ui_element();
  }

  fn update(&self, context: &mut C) {
    self.container.update(context);
  }

  fn draw(&self, context: &mut C) {
    self.container.draw(context);
  }

  fn get_touched_element(
    &self,
    context: &mut C,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<C>>> {
    let absolute_params = self.get_absolute_params(context);
    if !absolute_params.visible || !self.container.active.get() {
      return None;
    }

    let touched_child_element = self.container.get_touched_element(context, ui_touch);
    if touched_child_element.is_some() {
      return touched_child_element;
    }

    let touch_within_bounds = self.check_within_bounds(&absolute_params, ui_touch);
    self.pressed.set(match ui_touch.touch_type {
      TouchType::Pressed => touch_within_bounds,
      _ => self.pressed.get() && touch_within_bounds,
    });

    if touch_within_bounds {
      if let Some(weak_impl) = self.weak_impl.borrow().as_ref() {
        return Some(weak_impl.upgrade().unwrap());
      }
      return Some(self.weak_self.borrow().upgrade().unwrap());
    }
    return None;
  }

  fn consume_touch(&self, context: &mut C, ui_touch: &UiTouch) {
    if self.pressed.get() {
      match ui_touch.touch_type {
        TouchType::Pressed => {
          self.on_pressed_event.execute(context, ui_touch);
        }
        TouchType::Moved => {
          self.on_moved_event.execute(context, ui_touch);
        }
        TouchType::Released => {
          self.on_released_event.execute(context, ui_touch);
          self.pressed.set(false);
        }
      }
    }
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiTouchable<C> {
  type Target = UiContainer<C>;

  fn deref(&self) -> &Self::Target {
    return self.container.as_ref();
  }
}
