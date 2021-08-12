use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;
use std::ops::Deref;

pub struct UiText<C: ContextTrait + ?Sized> {
  pub element: UiElement<C>,
  text: Shared<String>,
  font_size: Shared<F1>,
  alignment: Shared<TextAlignment>,
  color: Cell<DrawColor>,
  border: Cell<bool>,
  text_cache: RefCell<Option<Rc<TextCache>>>,
}

impl<C: ContextTrait + ?Sized> UiText<C> {
  pub fn new() -> Rc<UiText<C>> {
    return Rc::new(UiText {
      element: UiElement::new(),
      text: Shared::default(),
      font_size: Shared::default(),
      alignment: Shared::new(TextAlignment::Left),
      color: Cell::default(),
      border: Cell::new(false),
      text_cache: RefCell::new(None),
    });
  }

  pub fn set_text(&self, text: String) {
    *self.text.borrow_mut() = text;
  }

  pub fn use_text_cache(&self) {
    self.text_cache.replace(Some(TextCache::new()));
  }

  pub fn set_font_size(&self, font_size: F1) {
    *self.font_size.borrow_mut() = font_size;
  }

  pub fn set_alignment(&self, alignment: TextAlignment) {
    *self.alignment.borrow_mut() = alignment;
  }

  pub fn set_color(&self, color: DrawColor) {
    self.color.set(color);
  }

  pub fn set_border(&self, border: bool) {
    self.border.set(border);
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiText<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait> UiElementTrait<C> for UiText<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return &self.element;
  }

  fn update(&self, _context: &mut C) {}

  fn draw(&self, context: &mut C) {
    let absolute_params = self.get_absolute_params(context);

    if !absolute_params.visible {
      return;
    }

    context.draw_string_ui_viewport(DrawStringArgs {
      text: self.text.borrow().clone(),
      position: absolute_params.position,
      font_size: *self.font_size.borrow(),
      depth: absolute_params.depth,
      optional: DrawStringOptionalArgs {
        alignment: *self.alignment.borrow(),
        color: self.color.get(),
        border: self.border.get(),
        opacity: absolute_params.opacity,
        text_cache: self.text_cache.borrow().clone(),
        ..Default::default()
      },
    });

    // context.get_draw_manager().draw_string_viewport(
    //   &context.get_ui_viewport(),
    //   DrawStringArgs {
    //     text: self.text.borrow().clone(),
    //     position: *absolute_params.position.borrow(),
    //     font_size: *self.font_size.borrow(),
    //     depth: absolute_params.depth.get(),
    //     optional: DrawStringOptionalArgs {
    //       alignment: *self.alignment.borrow(),
    //     },
    //   },
    // );
  }
}

impl<C: ContextTrait> Deref for UiText<C> {
  type Target = UiElement<C>;

  fn deref(&self) -> &Self::Target {
    return &self.element;
  }
}
