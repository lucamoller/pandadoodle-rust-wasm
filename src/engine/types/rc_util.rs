use std::rc::Rc;

pub struct RcUtil {}

impl RcUtil {
  pub fn get_ptr<T>(rc: &Rc<T>) -> *const T {
    return rc.as_ref() as *const T;
  }

  pub fn eq_ptr<T>(rc1: &Rc<T>, rc2: &Rc<T>) -> bool {
    return RcUtil::get_ptr(rc1) == RcUtil::get_ptr(rc2);
  }
}
