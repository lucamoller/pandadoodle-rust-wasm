use crate::engine::*;

pub struct Event {
  callbacks: RefCell<Vec<Box<dyn FnMut() -> ()>>>,
}

impl Event {
  pub fn empty() -> Event {
    Event {
      callbacks: RefCell::new(Vec::new()),
    }
  }

  pub fn add(&self, callback: Box<dyn FnMut() -> ()>) {
    self.callbacks.borrow_mut().push(callback);
  }

  pub fn execute(&self) {
    for callback in self.callbacks.borrow_mut().iter_mut() {
      callback();
    }
  }
}

pub struct Event1Arg<T> {
  callbacks: RefCell<Vec<Box<dyn FnMut(&T) -> ()>>>,
}

impl<T> Event1Arg<T> {
  pub fn empty() -> Event1Arg<T> {
    Event1Arg {
      callbacks: RefCell::new(Vec::new()),
    }
  }

  pub fn add(&self, callback: Box<dyn FnMut(&T) -> ()>) {
    self.callbacks.borrow_mut().push(callback);
  }

  pub fn execute(&self, arg1: &T) {
    for callback in self.callbacks.borrow_mut().iter_mut() {
      callback(arg1);
    }
  }
}

pub struct Event1ArgMut<T: ?Sized> {
  callbacks: RefCell<Vec<Box<dyn FnMut(&mut T) -> ()>>>,
}

impl<T: ?Sized> Event1ArgMut<T> {
  pub fn empty() -> Event1ArgMut<T> {
    Event1ArgMut {
      callbacks: RefCell::new(Vec::new()),
    }
  }

  pub fn add(&self, callback: Box<dyn FnMut(&mut T) -> ()>) {
    self.callbacks.borrow_mut().push(callback);
  }

  pub fn execute(&self, arg1: &mut T) {
    for callback in self.callbacks.borrow_mut().iter_mut() {
      callback(arg1);
    }
  }
}

pub struct Event2ArgMutRef<T: ?Sized, U> {
  callbacks: RefCell<Vec<Box<dyn FnMut(&mut T, &U) -> ()>>>,
}

impl<T: ?Sized, U> Event2ArgMutRef<T, U> {
  pub fn empty() -> Event2ArgMutRef<T, U> {
    Event2ArgMutRef {
      callbacks: RefCell::new(Vec::new()),
    }
  }

  pub fn add(&self, callback: Box<dyn FnMut(&mut T, &U) -> ()>) {
    self.callbacks.borrow_mut().push(callback);
  }

  pub fn add_event<E: 'static + Clone>(&self, event_manager: Rc<EventManager<E>>, event: E) {
    self.add(Box::new(move |_, _| event_manager.add_event(event.clone())));
  }

  pub fn execute(&self, arg1: &mut T, arg2: &U) {
    for callback in self.callbacks.borrow_mut().iter_mut() {
      callback(arg1, arg2);
    }
  }
}

pub struct Event3ArgMutRefRef<T: ?Sized, U, V> {
  callbacks: RefCell<Vec<Box<dyn FnMut(&mut T, &U, &V) -> ()>>>,
}

impl<T: ?Sized, U, V> Event3ArgMutRefRef<T, U, V> {
  pub fn empty() -> Event3ArgMutRefRef<T, U, V> {
    Event3ArgMutRefRef {
      callbacks: RefCell::new(Vec::new()),
    }
  }

  pub fn add(&self, callback: Box<dyn FnMut(&mut T, &U, &V) -> ()>) {
    self.callbacks.borrow_mut().push(callback);
  }

  pub fn execute(&self, arg1: &mut T, arg2: &U, arg3: &V) {
    for callback in self.callbacks.borrow_mut().iter_mut() {
      callback(arg1, arg2, arg3);
    }
  }
}
