use std::{
  cell::RefCell,
  thread::LocalKey,
};

pub trait WithInnerValue<T> {
  fn with_inner_value<R>(&'static self, callback: impl FnMut(&mut T) -> R) -> R;
  fn store(&'static self, val: T);
  // TODO implement
  // fn replace_inner_value(&'static self, callback: impl Fn(T));
}

impl<T> WithInnerValue<T> for LocalKey<RefCell<Option<T>>> {
  fn with_inner_value<R>(&'static self, callback: impl FnOnce(&mut T) -> R) -> R {
    self.with(|rc| {
      let val_opt = rc.replace(None);
      // TODO don't unwrap here, but what to do instead?
      let mut val = val_opt.unwrap();
      let new_val = callback(&mut val);
      rc.replace(Some(val));
      new_val
    })
  }

  fn store(&'static self, val: T) {
    self.with(|rc| {
      rc.replace(Some(val));
    });
  }
}
