use crate::engine::*;
use std::ops::Deref;

#[derive(Clone, Default, PartialEq)]
pub struct UiElementParams {
  pub position: Shared<F2>,
  pub opacity: Shared<F1>,
  pub depth: Shared<F1>,
  pub visible: Shared<bool>,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct AbsoluteUiElementParams {
  pub position: F2,
  pub opacity: F1,
  pub depth: F1,
  pub visible: bool,
}

#[derive(Default)]
pub struct UiElementClass {
  pub position: Option<F2>,
  pub position_x: Option<F1>,
  pub position_y: Option<F1>,
  pub opacity: Option<F1>,
  pub depth: Option<F1>,
  pub visible: Option<bool>,
  pub size: Option<F2>,
  pub size_x: Option<F1>,
  pub size_y: Option<F1>,
}

pub struct UiElement<C: ContextTrait + ?Sized> {
  relative_params: UiElementParams,
  parent: RefCell<Option<Weak<dyn UiElementTrait<C>>>>,
  absolute_params: Cell<AbsoluteUiElementParams>,
  absolute_params_draw_cycle: Cell<u32>,
}

impl<C: ContextTrait + ?Sized> UiElement<C> {
  pub fn new() -> UiElement<C> {
    return UiElement {
      relative_params: UiElementParams {
        position: Shared::new(F2 { x: 0.0, y: 0.0 }),
        opacity: Shared::new(1.0),
        depth: Shared::new(0.0),
        visible: Shared::new(true),
      },
      parent: RefCell::new(None),
      absolute_params: Cell::new(AbsoluteUiElementParams::default()),
      absolute_params_draw_cycle: Cell::new(0),
    };
  }

  pub fn set_class(&self, class: &UiElementClass) {
    let relative_params = &self.relative_params;

    if let Some(position) = class.position {
      *relative_params.position.borrow_mut() = position;
    }
    if let Some(position_x) = class.position_x {
      relative_params.position.borrow_mut().x = position_x;
    }
    if let Some(position_y) = class.position_y {
      relative_params.position.borrow_mut().y = position_y;
    }
    if let Some(opacity) = class.opacity {
      *relative_params.opacity.borrow_mut() = opacity;
    }
    if let Some(depth) = class.depth {
      *relative_params.depth.borrow_mut() = depth;
    }
    if let Some(visible) = class.visible {
      *relative_params.visible.borrow_mut() = visible;
    }
  }

  pub fn set_position(&self, position: F2) {
    *self.relative_params.position.borrow_mut() = position;
  }
  pub fn set_position_x(&self, position_x: F1) {
    self.relative_params.position.borrow_mut().x = position_x;
  }
  pub fn set_position_y(&self, position_y: F1) {
    self.relative_params.position.borrow_mut().y = position_y;
  }
  pub fn set_opacity(&self, opacity: F1) {
    *self.relative_params.opacity.borrow_mut() = opacity;
  }
  pub fn set_depth(&self, depth: F1) {
    *self.relative_params.depth.borrow_mut() = depth;
  }
  pub fn set_visible(&self, visible: bool) {
    *self.relative_params.visible.borrow_mut() = visible;
  }
  pub fn get_visible(&self) -> Shared<bool> {
    return self.relative_params.visible.clone();
  }
  pub fn get_position(&self) -> Shared<F2> {
    return self.relative_params.position.clone();
  }
  pub fn get_position_x(&self) -> F1 {
    return self.relative_params.position.borrow().x;
  }
  pub fn get_position_y(&self) -> F1 {
    return self.relative_params.position.borrow().y;
  }
  pub fn get_opacity(&self) -> Shared<F1> {
    return self.relative_params.opacity.clone();
  }
  pub fn set_parent(&self, parent: Weak<dyn UiElementTrait<C>>) {
    *self.parent.borrow_mut() = Some(parent);
  }
  pub fn get_parent(&self) -> Option<Weak<dyn UiElementTrait<C>>> {
    return self.parent.borrow().clone();
  }
  pub fn get_absolute_params(&self, context: &mut C) -> AbsoluteUiElementParams {
    if self.absolute_params_draw_cycle.get() == *context.get_draw_cycle() {
      return self.absolute_params.get();
    }

    let relative_params = &self.relative_params;
    let new_absolute_params = match self.get_parent() {
      Some(weak_parent) => match weak_parent.upgrade() {
        Some(parent) => {
          let parent_absolute_params = parent.get_absolute_params(context);

          let absolute_position =
            parent_absolute_params.position + *relative_params.position.borrow();
          let absolute_opacity = parent_absolute_params.opacity * *relative_params.opacity.borrow();
          let absolute_depth =
            parent_absolute_params.depth + *relative_params.depth.borrow() / 10.0;
          let absolute_visible =
            parent_absolute_params.visible && *relative_params.visible.borrow();

          AbsoluteUiElementParams {
            position: absolute_position,
            opacity: absolute_opacity,
            depth: absolute_depth,
            visible: absolute_visible,
          }
        }
        None => AbsoluteUiElementParams {
          position: *relative_params.position.borrow(),
          opacity: *relative_params.opacity.borrow(),
          depth: *relative_params.depth.borrow(),
          visible: *relative_params.visible.borrow(),
        },
      },
      None => AbsoluteUiElementParams {
        position: *relative_params.position.borrow(),
        opacity: *relative_params.opacity.borrow(),
        depth: *relative_params.depth.borrow(),
        visible: *relative_params.visible.borrow(),
      },
    };

    self.absolute_params.set(new_absolute_params);
    self
      .absolute_params_draw_cycle
      .set(*context.get_draw_cycle());
    return self.absolute_params.get();
  }
}

pub trait UiElementTrait<C: ContextTrait + ?Sized>: EffectManagerTrait<C> {
  fn get_ui_element(&self) -> &UiElement<C>;

  fn update(&self, context: &mut C);

  // fn update_effects(&self, context: &mut C) {
  //   if let Some(effects) = self.get_effects() {
  //     for effect in effects {
  //       let dt = *context.get_dt();
  //       effect.update(&dt, context);
  //     }
  //   }
  // }

  fn draw(&self, context: &mut C);

  fn get_touched_element(
    &self,
    _context: &mut C,
    _ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<C>>> {
    return None;
  }

  fn consume_touch(&self, _context: &mut C, _ui_touch: &UiTouch) {}

  fn get_effects(&self) -> Option<Vec<&dyn EffectTrait<C>>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> Deref for dyn UiElementTrait<C> {
  type Target = UiElement<C>;

  fn deref(&self) -> &Self::Target {
    return &self.get_ui_element();
  }
}
