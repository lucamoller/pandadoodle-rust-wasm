use std::convert::From;
use std::hash::{Hash, Hasher};
use std::marker::Unsize;
use std::ops::CoerceUnsized;

#[derive(Debug)]
pub struct HashablePointer<T: ?Sized> {
  pub pointer: *const T,
}

impl<T: ?Sized> Clone for HashablePointer<T> {
  fn clone(&self) -> HashablePointer<T> {
    return HashablePointer {
      pointer: self.pointer,
    };
  }
}

impl<T> HashablePointer<T> {
  pub fn new(reference: &T) -> HashablePointer<T> {
    return HashablePointer {
      pointer: reference as *const T,
    };
  }
}

impl<T> PartialEq for HashablePointer<T> {
  fn eq(&self, other: &HashablePointer<T>) -> bool {
    return self.pointer == other.pointer;
  }
}

impl<T> Eq for HashablePointer<T> {}

impl<T, U> CoerceUnsized<HashablePointer<U>> for HashablePointer<T>
where
  T: Unsize<U> + ?Sized,
  U: ?Sized,
{
}

impl<T> Hash for HashablePointer<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pointer.hash(state);
  }
}

impl<T> From<&T> for HashablePointer<T> {
  fn from(reference: &T) -> HashablePointer<T> {
    return HashablePointer::new(reference);
  }
}
