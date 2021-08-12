use crate::engine::effect::*;
use crate::engine::ui::*;
use crate::engine::*;

pub struct UiPivot<C: ContextTrait + ?Sized> {
  effect_manager: EffectManager<C>,

  pub container: Rc<UiContainer<C>>,
  pub slot_default_width: F1,

  pub slide_container: Rc<UiContainer<C>>,
  pub aff_slide: Rc<Effect<C, VectorAffectorF2>>,
  pub index: Cell<i32>,
  pub max_index: Cell<i32>,

  pub hold_pos: Cell<F1>,
  pub start_delta_x: Cell<F1>,
  pub holding: Cell<bool>,
  pub holding_id: Cell<i32>,
  pub move_again: Cell<bool>,

  pub last_touched: RefCell<Option<Rc<dyn UiElementTrait<C>>>>,

  pub weak_self: RefCell<Weak<UiPivot<C>>>,
}

impl<C: ContextTrait + ?Sized> UiPivot<C> {
  pub fn new(slot_width: F1) -> Rc<UiPivot<C>> {
    let effect_manager = EffectManager::new();
    let container = UiContainer::new();

    let slide_container = UiContainer::new();
    let aff_slide = Effect::new_within_effect_manager(
      VectorAffectorF2::new(slide_container.get_position()),
      &effect_manager,
    );
    aff_slide
      .effect_impl
      .set_progression_onref(Box::new(ExpTransProgression::new(2.0, 6.0)));
    container.add_child(slide_container.clone());

    let result = Rc::new(UiPivot {
      effect_manager,
      container: container,
      slot_default_width: slot_width,
      slide_container: slide_container,
      aff_slide: aff_slide,
      index: Cell::new(0),
      max_index: Cell::new(0),

      hold_pos: Cell::new(0.0),
      start_delta_x: Cell::new(0.0),
      holding: Cell::new(false),
      holding_id: Cell::new(0),
      move_again: Cell::new(false),

      last_touched: RefCell::new(None),
      weak_self: RefCell::new(Weak::new()),
    });
    *result.weak_self.borrow_mut() = Rc::downgrade(&result);
    return result;
  }

  pub fn add_child(&self, child_element: Rc<dyn UiElementTrait<C>>) {
    let touchable = UiTouchable::new();
    touchable.set_size_x(self.slot_default_width);
    touchable.set_position_x(
      self.sum_position_before(self.max_index.get()) + self.slot_default_width / 2.0,
    );
    self.slide_container.add_child(touchable.clone());
    touchable.add_child(child_element);
    self.max_index.set(self.max_index.get() + 1);
    if self.max_index.get() == 1 {
      self.slide(0);
    }
  }

  pub fn slide_next(&self) {
    self.slide(self.index.get() + 1);
  }

  pub fn slide_prev(&self) {
    self.slide(self.index.get() - 1);
  }

  pub fn slide(&self, index: i32) {
    let mut index = index;
    if index < 0 {
      index = 0;
    }
    if index >= self.max_index.get() {
      index = self.max_index.get() - 1;
    }

    self.index.set(index);
    self.aff_slide.effect_impl.set_end_onref(
      F2 {
        x: self.next_pos(index),
        y: self.slide_container.get_position_y(),
      },
      400.0,
    );
    self.aff_slide.start();
  }

  fn sum_position_before(&self, index: i32) -> F1 {
    let mut sum_position_before: F1 = 0.0;
    for _ in 0..index {
      sum_position_before += self.slot_default_width;
    }
    return sum_position_before;
  }

  fn next_pos(&self, index: i32) -> F1 {
    return self.container.get_position_x()
      - self.sum_position_before(index)
      - self.slot_default_width * 0.5;
  }
}

impl<C: ContextTrait + ?Sized> EffectManagerTrait<C> for UiPivot<C> {
  fn get_effect_manager(&self) -> Option<&EffectManager<C>> {
    return Some(&self.effect_manager);
  }
}

impl<C: ContextTrait + ?Sized> UiElementTrait<C> for UiPivot<C> {
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
    if self.holding.get() && self.holding_id.get() != ui_touch.id {
      return None;
    }
    self
      .last_touched
      .replace(self.container.get_touched_element(context, ui_touch));

    if self.last_touched.borrow().is_some() || self.holding.get() {
      return Some(self.weak_self.borrow().upgrade().unwrap().clone());
    }
    return None;
  }

  fn consume_touch(&self, context: &mut C, ui_touch: &UiTouch) {
    if self.holding.get() && self.holding_id.get() != ui_touch.id {
      return;
    }

    let delta_x = ui_touch.delta.x - self.start_delta_x.get();
    if ui_touch.touch_type == TouchType::Released {
      if delta_x > 25.0 / 480.0 {
        self.slide_prev();
      } else if delta_x < -25.0 / 480.0 {
        self.slide_next();
      } else {
        if let Some(last_touched) = self.last_touched.borrow_mut().take() {
          last_touched.consume_touch(context, ui_touch);
        }
      }
      self.holding.set(false);
    } else if ui_touch.touch_type == TouchType::Moved && self.holding.get() {
      self
        .slide_container
        .set_position_x(delta_x + self.hold_pos.get());
    } else {
      self.holding.set(true);
      self.holding_id.set(ui_touch.id);
      self.start_delta_x.set(ui_touch.delta.x);
      self.hold_pos.set(self.slide_container.get_position_x());
    }
  }
}

impl<C: ContextTrait + ?Sized> Deref for UiPivot<C> {
  type Target = UiContainer<C>;

  fn deref(&self) -> &Self::Target {
    return &self.container;
  }
}
