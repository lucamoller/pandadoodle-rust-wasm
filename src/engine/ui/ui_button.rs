use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;

pub struct UiButton<C: ContextTrait + ?Sized> {
  touchable: Rc<UiTouchable<C>>,
  pub sprite: Rc<UiSprite<C>>,
  sprite_pressed: Rc<UiSprite<C>>,
}

impl<C: ContextTrait + ?Sized> UiButton<C> {
  pub fn new(sprite_texture: Rc<Texture>, sprite_pressed_texture: Rc<Texture>) -> Rc<UiButton<C>> {
    let sprite = Rc::new(UiSprite::new(sprite_texture));
    let sprite_pressed = Rc::new(UiSprite::new(sprite_pressed_texture));

    let touchable = UiTouchable::new();

    touchable.add_child(sprite.clone());
    touchable.add_child(sprite_pressed.clone());

    return Rc::new(UiButton {
      touchable: touchable,
      sprite: sprite,
      sprite_pressed: sprite_pressed,
    });
  }

  pub fn set_event_on_released<T: 'static + Clone>(
    &self,
    event_manager: Rc<EventManager<T>>,
    event: T,
  ) {
    self.on_released_event.add_event(event_manager, event);
  }

  pub fn set_sound_on_released(&self, sound: Rc<Audio>) {
    self
      .on_released_event
      .add(Box::new(move |context, _ui_touch| {
        context.play_sound(&sound);
      }));
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

    self.touchable.set_class(class);
  }

  pub fn set_size(&self, size: F2) {
    self.touchable.set_size(size);
    self.sprite.set_size(size);
    self.sprite_pressed.set_size(size);
  }

  pub fn set_size_from_x(&self, size_x: F1) {
    self.sprite.set_size_from_width(size_x);
    self.sprite_pressed.set_size_from_width(size_x);
    self.touchable.set_size(*self.sprite.size.borrow());
  }

  pub fn set_size_x(&self, size_x: F1) {
    self.touchable.set_size_x(size_x);
    self.sprite.set_size_x(size_x);
    self.sprite_pressed.set_size_x(size_x);
  }

  pub fn set_size_y(&self, size_y: F1) {
    self.touchable.set_size_y(size_y);
    self.sprite.set_size_y(size_y);
    self.sprite_pressed.set_size_y(size_y);
  }

  pub fn set_texture(&self, texture: Rc<Texture>) {
    self.sprite.set_texture(texture);
  }

  pub fn set_texture_pressed(&self, texture_pressed: Rc<Texture>) {
    self.sprite_pressed.set_texture(texture_pressed);
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiButton<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return None;
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiButton<C> {
  fn get_ui_element(&self) -> &UiElement<C> {
    return self.touchable.container.get_ui_element();
  }

  fn get_touched_element(
    &self,
    context: &mut C,
    ui_touch: &UiTouch,
  ) -> Option<Rc<dyn UiElementTrait<C>>> {
    return self.touchable.get_touched_element(context, ui_touch);
  }

  fn update(&self, context: &mut C) {
    self.touchable.container.update(context);
    let pressed = self.touchable.pressed.get();
    self.sprite.set_visible(!pressed);
    self.sprite_pressed.set_visible(pressed);
  }

  fn draw(&self, context: &mut C) {
    self.touchable.container.draw(context);
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiButton<C> {
  type Target = UiTouchable<C>;

  fn deref(&self) -> &Self::Target {
    return self.touchable.as_ref();
  }
}
