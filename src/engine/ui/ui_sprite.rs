use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;
use std::ops::Deref;

pub struct UiSprite<C: ContextTrait + ?Sized> {
  pub element: UiElement<C>,

  pub texture: RefCell<Rc<Texture>>,
  pub size: Shared<F2>,
  pub rotation: Shared<F1>,
  pub color: Shared<DrawColor>,
  pub subpixel_precision: Cell<bool>,
}

impl<C: ContextTrait + ?Sized> UiSprite<C> {
  pub fn new(texture: Rc<Texture>) -> UiSprite<C> {
    return UiSprite {
      element: UiElement::new(),

      texture: RefCell::new(texture),
      size: Shared::default(),
      rotation: Shared::default(),
      color: Shared::default(),
      subpixel_precision: Cell::new(false),
    };
  }

  pub fn set_class(&self, class: &UiElementClass) {
    if let Some(size) = class.size {
      self.set_size(size);
    }
    if let Some(size_x) = class.size_x {
      self.set_size_x(size_x);
    }
    if let Some(size_y) = class.size_y {
      self.set_size_y(size_y);
    }

    self.element.set_class(class);
  }

  pub fn set_size(&self, size: F2) {
    *self.size.borrow_mut() = size;
  }

  pub fn set_size_from_width(&self, size_x: F1) {
    *self.size.borrow_mut() = self.texture.borrow().get_size_from_width(size_x);
  }

  pub fn set_size_x(&self, size_x: F1) {
    self.size.borrow_mut().x = size_x;
  }

  pub fn set_size_y(&self, size_y: F1) {
    self.size.borrow_mut().y = size_y;
  }

  pub fn set_color(&self, color: DrawColor) {
    *self.color.borrow_mut() = color;
  }

  pub fn set_subpixel_precision(&self, subpixel_precision: bool) {
    self.subpixel_precision.set(subpixel_precision);
  }

  pub fn set_texture(&self, texture: Rc<Texture>) {
    self.texture.replace(texture);
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiSprite<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiSprite<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return &self.element;
  }

  fn update(&self, _context: &mut C) {}

  fn draw(&self, context: &mut C) {
    let absolute_params = self.get_absolute_params(context);

    if !absolute_params.visible {
      return;
    }

    context.draw_ui_viewport(DrawImageArgs {
      source: DrawSource::Texture(self.texture.borrow().clone()),
      position: absolute_params.position,
      size: *self.size.borrow(),
      depth: absolute_params.depth,
      optional: DrawImageOptionalArgs {
        color: *self.color.borrow(),
        rotation: *self.rotation.borrow(),
        opacity: absolute_params.opacity,
        subpixel_precision: self.subpixel_precision.get(),
        ..Default::default()
      },
    });
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiSprite<C> {
  type Target = UiElement<C>;

  fn deref(&self) -> &Self::Target {
    return &self.element;
  }
}
