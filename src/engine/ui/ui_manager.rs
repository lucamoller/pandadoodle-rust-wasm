use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;

const MAX_TOUCH_IDS: usize = 10;

pub enum UiManagerEvent<C: ContextTrait + ?Sized> {
  CloseCurrentPage,
  ShowUiOverlay(Rc<dyn UiOverlayTrait<C>>),
  HideUiOverlay,
  AnimationFadeOutEnd,
  AnimationSwitchRootEnd,
}

impl<C: ContextTrait + ?Sized> Clone for UiManagerEvent<C> {
  fn clone(&self) -> UiManagerEvent<C> {
    return match self {
      UiManagerEvent::CloseCurrentPage => UiManagerEvent::CloseCurrentPage,
      UiManagerEvent::ShowUiOverlay(overlay) => UiManagerEvent::ShowUiOverlay(overlay.clone()),
      UiManagerEvent::HideUiOverlay => UiManagerEvent::HideUiOverlay,
      UiManagerEvent::AnimationFadeOutEnd => UiManagerEvent::AnimationFadeOutEnd,
      UiManagerEvent::AnimationSwitchRootEnd => UiManagerEvent::AnimationSwitchRootEnd,
    };
  }
}

pub struct UiManager<C: ContextTrait + ?Sized> {
  effect_manager: EffectManager<C>,
  navigation_stack: RefCell<Vec<Rc<dyn UiRootTrait<C>>>>,
  pub current_page: RefCell<Option<Rc<dyn UiRootTrait<C>>>>,
  next_page: RefCell<Option<Rc<dyn UiRootTrait<C>>>>,
  block_input: Cell<bool>,

  overlay: RefCell<Option<Rc<dyn UiOverlayTrait<C>>>>,
  sprite_overlay_background: Rc<UiSprite<C>>,

  sprite_fade: Rc<UiSprite<C>>,

  last_press_pos: RefCell<Vec<F2>>,
  dragged_element: RefCell<Vec<Option<Rc<dyn UiElementTrait<C>>>>>,

  animation_switch_root: Rc<Effect<C, ChainedEffect<C>>>,
}

impl<C: ContextTrait + ?Sized> UiManager<C> {
  pub fn new(context: &C) -> Rc<UiManager<C>> {
    let effect_manager = EffectManager::new();
    let sprite_fade = Rc::new(UiSprite::new(context.get_pixel_texture()));
    sprite_fade.set_opacity(0.0);
    sprite_fade.set_color(DrawColor { r: 0, g: 0, b: 0 });
    sprite_fade.set_depth(context.get_front_board_depth());

    let sprite_overlay_background = Rc::new(UiSprite::new(context.get_pixel_texture()));
    sprite_overlay_background.set_opacity(0.5);
    sprite_overlay_background.set_color(DrawColor { r: 0, g: 0, b: 0 });
    sprite_overlay_background.set_depth(context.get_front_board_depth());

    let animation_switch_root =
      Effect::new_within_effect_manager(ChainedEffect::new(), &effect_manager);
    animation_switch_root.add_event_on_end(
      context.get_ui_manager_events().clone(),
      UiManagerEventGeneric::AnimationSwitchRootEnd,
    );
    let fade_out_effect = Effect::new_within_chained_effect(
      VectorAffectorF1::new(sprite_fade.get_opacity()).set_start_and_end(0.0, 1.0, 100.0),
      &animation_switch_root,
    );
    fade_out_effect.add_event_on_end(
      context.get_ui_manager_events().clone(),
      UiManagerEventGeneric::AnimationFadeOutEnd,
    );
    Effect::new_within_chained_effect(
      VectorAffectorF1::new(sprite_fade.get_opacity()).set_start_and_end(1.0, 0.0, 200.0),
      &animation_switch_root,
    );

    return Rc::new(UiManager {
      effect_manager,
      navigation_stack: RefCell::new(Vec::new()),
      current_page: RefCell::new(None),
      next_page: RefCell::new(None),
      block_input: Cell::new(false),
      overlay: RefCell::new(None),
      sprite_overlay_background: sprite_overlay_background,
      sprite_fade: sprite_fade,
      last_press_pos: RefCell::new(vec![F2 { x: 0.0, y: 0.0 }; MAX_TOUCH_IDS]),
      dragged_element: RefCell::new(vec![None; MAX_TOUCH_IDS]),
      animation_switch_root: animation_switch_root.clone(),
    });
  }

  pub fn animation_fade_out_end(&self, context: &mut C) {
    self
      .current_page
      .replace(self.next_page.borrow_mut().take());
    if let Some(current_page) = &*self.current_page.borrow() {
      current_page.on_navigate_to(context);
    }
  }

  pub fn animation_switch_root_end(&self) {
    self.block_input.set(false);
  }

  pub fn push_page_on_stack(&self, next_page: Rc<dyn UiRootTrait<C>>, context: &mut C) {
    let current_page = self.current_page.borrow().clone();
    if let Some(current_page) = &current_page {
      self
        .navigation_stack
        .borrow_mut()
        .push(current_page.clone());
      current_page.on_navigate_from(context);
      self.block_input.set(true);
      self.next_page.replace(Some(next_page));
      self.animation_switch_root.start();
    } else {
      self.current_page.replace(Some(next_page.clone()));
      next_page.on_navigate_to(context);
    }
  }

  pub fn switch_to_page(&self, next_page: Rc<dyn UiRootTrait<C>>, context: &mut C) {
    let current_page = self.current_page.borrow().clone();
    if let Some(current_page) = &current_page {
      current_page.on_navigate_from(context);
      self.block_input.set(true);
      self.next_page.replace(Some(next_page));
      self.animation_switch_root.start();
    } else {
      self.current_page.replace(Some(next_page.clone()));
      next_page.on_navigate_to(context);
    }
  }

  pub fn process_touch(&self, context: &mut C, screen_touch: &ScreenTouch) -> bool {
    let mut ui_touch = UiTouch::from_screen_touch(&screen_touch, context);
    if self.process_touch_ui(context, &mut ui_touch) {
      return true;
    }
    if let Some(current_page) = &*self.current_page.borrow() {
      return current_page.process_touch_game(context, screen_touch);
    }
    return false;
  }

  pub fn process_touch_ui(&self, context: &mut C, ui_touch: &mut UiTouch) -> bool {
    if self.block_input.get() {
      return false;
    }
    if ui_touch.id as usize >= MAX_TOUCH_IDS {
      return false;
    }

    let mut touched_element = None;

    if let Some(overlay) = self.overlay.borrow().as_ref() {
      touched_element = overlay.get_touched_element(context, ui_touch);
    } else if let Some(current_page) = self.current_page.borrow().as_ref() {
      touched_element = current_page.get_touched_element(context, ui_touch);
    }

    match ui_touch.touch_type {
      TouchType::Released => {
        ui_touch.delta = ui_touch.position - self.last_press_pos.borrow()[ui_touch.id as usize];
        if self.dragged_element.borrow()[ui_touch.id as usize].is_some() {
          touched_element = self.dragged_element.borrow_mut()[ui_touch.id as usize].take();
        }
      }
      TouchType::Pressed => {
        if self.dragged_element.borrow()[ui_touch.id as usize].is_none() {
          self.dragged_element.borrow_mut()[ui_touch.id as usize] = touched_element.clone();
          self.last_press_pos.borrow_mut()[ui_touch.id as usize] = ui_touch.position;
        }
      }
      TouchType::Moved => {
        if self.dragged_element.borrow()[ui_touch.id as usize].is_some() {
          touched_element = self.dragged_element.borrow()[ui_touch.id as usize].clone();
        }
        ui_touch.delta = ui_touch.position - self.last_press_pos.borrow()[ui_touch.id as usize];
      }
    }

    if let Some(touched_element) = touched_element {
      touched_element.consume_touch(context, ui_touch);
      return true;
    }

    return !self.overlay.borrow().is_none();
  }

  pub fn process_back_button(&self, context: &mut C) -> InputState {
    if let Some(overlay) = self.overlay.borrow().as_ref() {
      let input_state = overlay.on_press_back(context);
      if input_state == InputState::Consumed {
        return InputState::Consumed;
      }
    }

    if let Some(current_page) = self.current_page.borrow().as_ref() {
      let input_state = current_page.on_press_back(context);
      if input_state == InputState::Consumed {
        return InputState::Consumed;
      }
    }

    let input_state = self.close_current_page(context);
    if input_state == InputState::Consumed {
      return InputState::Consumed;
    }

    return InputState::Available;
  }

  fn close_current_page(&self, context: &mut C) -> InputState {
    if let Some(prev_page) = self.navigation_stack.borrow_mut().pop() {
      if let Some(current_page) = &*self.current_page.borrow() {
        current_page.on_navigate_from(context);
      }
      self.current_page.replace(Some(prev_page.clone()));
      prev_page.on_navigate_to(context);
      return InputState::Consumed;
    }
    return InputState::Available;
  }

  pub fn update(&self, context: &mut C) {
    self.update_effects(context);
    while let Some(event) = context.get_ui_manager_events().consume_event() {
      match event {
        UiManagerEvent::CloseCurrentPage => {
          self.close_current_page(context);
        }
        UiManagerEvent::ShowUiOverlay(overlay) => {
          self.overlay.replace(Some(overlay));
        }
        UiManagerEvent::HideUiOverlay => {
          self.overlay.replace(None);
        }
        UiManagerEventGeneric::AnimationFadeOutEnd => {
          self.animation_fade_out_end(context);
        }
        UiManagerEventGeneric::AnimationSwitchRootEnd => {
          self.animation_switch_root_end();
        }
      }
    }

    if let Some(current_page) = self.current_page.borrow().as_ref() {
      current_page.update(context);
      current_page.update_effects(context);
    }

    if let Some(overlay) = self.overlay.borrow().as_ref() {
      overlay.update(context);
      overlay.update_effects(context);
    }
  }

  pub fn draw(&self, context: &mut C) {
    if let Some(current_page) = self.current_page.borrow().as_ref() {
      current_page.draw(context);
    }

    if let Some(overlay) = self.overlay.borrow().as_ref() {
      overlay.draw(context);
      self
        .sprite_overlay_background
        .set_position(context.get_ui_viewport_screen_center());
      self
        .sprite_overlay_background
        .set_size(context.get_ui_viewport_screen_size());
      self.sprite_overlay_background.draw(context);
    }

    self
      .sprite_fade
      .set_position(context.get_ui_viewport_screen_center());
    self
      .sprite_fade
      .set_size(context.get_ui_viewport_screen_size());
    self.sprite_fade.draw(context);
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiManager<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return Some(&self.effect_manager);
  }
}
