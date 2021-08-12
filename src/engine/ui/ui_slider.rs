use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;

pub struct UiSlider<C: ContextTrait + ?Sized> {
  touchable: Rc<UiTouchable<C>>,
  sprite_bar: Rc<UiSprite<C>>,
  sprite_cursor: Rc<UiSprite<C>>,
  pub on_changed_event: Event3ArgMutRefRef<C, Shared<F1>, F1>,
  pub value: Shared<F1>,
}

impl<C: ContextTrait + ?Sized> UiSlider<C> {
  pub fn new(sprite_bar: Rc<UiSprite<C>>, sprite_cursor: Rc<UiSprite<C>>) -> Rc<UiSlider<C>> {
    let touchable = UiTouchable::new();
    touchable.add_child(sprite_bar.clone());
    touchable.add_child(sprite_cursor.clone());
    touchable.set_size(F2 {
      x: sprite_bar.size.borrow().x,
      y: sprite_cursor.size.borrow().y,
    });

    let result = Rc::new(UiSlider {
      touchable: touchable,
      sprite_bar: sprite_bar,
      sprite_cursor: sprite_cursor,
      on_changed_event: Event3ArgMutRefRef::empty(),
      value: Shared::new(0.0),
    });
    result.touchable.weak_impl.replace(Some(Rc::downgrade(
      &(result.clone() as Rc<dyn UiElementTrait<C>>),
    )));
    return result;
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiSlider<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiSlider<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return self.touchable.get_ui_element();
  }

  fn update(&self, context: &mut C) {
    self.touchable.update(context);
    self.sprite_cursor.set_position_x(
      self.sprite_bar.get_position_x() + (self.value.get() - 0.5) * self.sprite_bar.size.borrow().x,
    );
  }

  fn draw(&self, context: &mut C) {
    self.touchable.draw(context);
  }

  fn get_touched_element(
    &self,
    context: &mut C,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<C>>> {
    return self.touchable.get_touched_element(context, ui_touch);
  }

  fn consume_touch(&self, context: &mut C, ui_touch: &UiTouch) {
    if ui_touch.touch_type != TouchType::Moved {
      return;
    }
    let absolute_position_x = self.touchable.get_absolute_params(context).position.x;
    let mut value =
      0.5 + (ui_touch.position.x - absolute_position_x) / self.touchable.size.borrow().x;
    value = if value < 0.0 {
      0.0
    } else if value > 1.0 {
      1.0
    } else {
      value
    };
    // self.value.set(value);
    self.on_changed_event.execute(context, &self.value, &value);
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiSlider<C> {
  type Target = UiTouchable<C>;

  fn deref(&self) -> &Self::Target {
    return self.touchable.as_ref();
  }
}
