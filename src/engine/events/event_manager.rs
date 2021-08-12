use crate::engine::*;

pub struct EventManager<T> {
  pending_events: RefCell<VecDeque<T>>,
}

impl<T> EventManager<T> {
  pub fn new() -> Rc<EventManager<T>> {
    return Rc::new(EventManager {
      pending_events: RefCell::default(),
    });
  }

  pub fn add_event(&self, event: T) {
    self.pending_events.borrow_mut().push_back(event);
  }

  pub fn consume_event(&self) -> Option<T> {
    return self.pending_events.borrow_mut().pop_front();
  }
}
