use crate::engine::*;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::marker::Unsize;
use std::ops::CoerceUnsized;
use std::ops::Deref;
use std::rc::Rc;

pub struct HashableRc<T: ?Sized> {
  pub rc: Rc<T>,
}

// Manually implemented so that the Clone trait is not required for T.
impl<T: ?Sized> Clone for HashableRc<T> {
  fn clone(&self) -> HashableRc<T> {
    return HashableRc {
      rc: self.rc.clone(),
    };
  }
}

impl<T> HashableRc<T> {
  pub fn new(value: T) -> HashableRc<T> {
    return HashableRc { rc: Rc::new(value) };
  }
}

impl<T> PartialEq for HashableRc<T> {
  fn eq(&self, other: &HashableRc<T>) -> bool {
    return RcUtil::get_ptr(self) == RcUtil::get_ptr(other);
  }
}

impl<T> PartialEq<Rc<T>> for HashableRc<T> {
  fn eq(&self, other: &Rc<T>) -> bool {
    return RcUtil::get_ptr(self) == RcUtil::get_ptr(other);
  }
}

impl<T> Eq for HashableRc<T> {}

impl<T, U> CoerceUnsized<HashableRc<U>> for HashableRc<T>
where
  T: Unsize<U> + ?Sized,
  U: ?Sized,
{
}

impl<T> Hash for HashableRc<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    RcUtil::get_ptr(self).hash(state);
  }
}

impl<T> Default for HashableRc<T>
where
  T: Default,
{
  fn default() -> HashableRc<T> {
    return HashableRc::new(T::default());
  }
}

impl<T> From<Rc<T>> for HashableRc<T> {
  fn from(rc: Rc<T>) -> HashableRc<T> {
    return HashableRc { rc: rc };
  }
}

impl<T> Deref for HashableRc<T> {
  type Target = Rc<T>;

  fn deref(&self) -> &Self::Target {
    return &self.rc;
  }
}
