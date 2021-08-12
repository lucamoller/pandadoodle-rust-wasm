use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::hash::{Hash, Hasher};
use std::marker::Unsize;
use std::ops::CoerceUnsized;
use std::rc::Rc;
use std::rc::Weak;

pub struct Shared<T: ?Sized> {
  pub pointer: Rc<RefCell<T>>,
}

// Manually implemented so that the Clone trait is not required for T.
impl<T: ?Sized> Clone for Shared<T> {
  fn clone(&self) -> Shared<T> {
    return Shared {
      pointer: self.pointer.clone(),
    };
  }
}

impl<T> Shared<T> {
  pub fn new(value: T) -> Shared<T> {
    return Shared {
      pointer: Rc::new(RefCell::new(value)),
    };
  }
}

impl<T: ?Sized> Shared<T> {
  pub fn borrow(&self) -> Ref<'_, T> {
    return self.pointer.borrow();
  }

  pub fn borrow_mut(&self) -> RefMut<'_, T> {
    return self.pointer.borrow_mut();
  }

  pub fn downgrade(&self) -> WeakShared<T> {
    return WeakShared {
      weak: Rc::downgrade(&self.pointer),
    };
  }

  pub fn as_pointer(&self) -> *const T {
    return &*self.pointer.borrow() as *const T;
  }
}

impl<T> Shared<T> {
  pub fn replace(&self, t: T) {
    self.pointer.replace(t);
  }
}

impl<T: ?Sized + Copy> Shared<T> {
  pub fn get(&self) -> T {
    return *self.pointer.borrow();
  }
}

impl<T> PartialEq for Shared<T> {
  fn eq(&self, other: &Shared<T>) -> bool {
    return self.as_pointer() == other.as_pointer();
  }
}

impl<T> Eq for Shared<T> {}

impl<T, U> CoerceUnsized<Shared<U>> for Shared<T>
where
  T: Unsize<U> + ?Sized,
  U: ?Sized,
{
}

impl<T> Hash for Shared<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_pointer().hash(state);
  }
}

impl<T> Default for Shared<T>
where
  T: Default,
{
  fn default() -> Shared<T> {
    return Shared::new(T::default());
  }
}

pub struct WeakShared<T: ?Sized> {
  pub weak: Weak<RefCell<T>>,
}

// Manually implemented so that the Clone trait is not required for T.
impl<T: ?Sized> Clone for WeakShared<T> {
  fn clone(&self) -> WeakShared<T> {
    return WeakShared {
      weak: self.weak.clone(),
    };
  }
}

impl<T> WeakShared<T> {
  pub fn new() -> WeakShared<T> {
    return WeakShared { weak: Weak::new() };
  }
}

impl<T: ?Sized> WeakShared<T> {
  pub fn upgrade(&self) -> Option<Shared<T>> {
    return match self.weak.upgrade() {
      Some(pointer) => Some(Shared { pointer: pointer }),
      None => None,
    };
  }
}

impl<T, U> CoerceUnsized<WeakShared<U>> for WeakShared<T>
where
  T: Unsize<U> + ?Sized,
  U: ?Sized,
{
}
