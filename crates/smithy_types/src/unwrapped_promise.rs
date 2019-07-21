use futures::Future;
use std::{
  cell::RefCell,
  ops::Deref,
  rc::Rc,
};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{
  future_to_promise,
  JsFuture,
};

/// A wrapper around a future that can easily be rendered with
/// a match statement.
///
/// It is used by Smithy to create promises that also cause smithy
/// to re-render when they are completed.
pub struct UnwrappedPromise<S, E> {
  promise_state: Rc<RefCell<PromiseState<S, E>>>,
  #[allow(dead_code)] // future must not be dropped before promise_state
  future: Box<dyn Future<Item = JsValue, Error = JsValue>>,
}

impl<S: 'static, E: 'static> UnwrappedPromise<S, E> {
  pub fn new(
    future: impl Future<Item = S, Error = E> + 'static,
    callback: Option<impl Fn() + 'static>,
  ) -> Self {
    let data = Rc::new(RefCell::new(PromiseState::Pending));
    let data_1 = data.clone();
    let data_2 = data.clone();

    let callback = Rc::new(RefCell::new(callback));
    let callback_1 = callback.clone();

    let future = future
      .map(move |s| {
        *data_1.borrow_mut() = PromiseState::Success(s);
        if let Some(ref cb) = *callback.borrow() {
          cb();
        };
        JsValue::NULL
      })
      .map_err(move |e| {
        *data_2.borrow_mut() = PromiseState::Error(e);
        if let Some(ref cb) = *callback_1.borrow() {
          cb();
        };
        JsValue::NULL
      });
    // execute the future
    let future = Box::new(JsFuture::from(future_to_promise(future)));
    let unwrapped_promise = UnwrappedPromise {
      promise_state: data,
      future,
    };
    unwrapped_promise
  }
}

impl<S, E> Deref for UnwrappedPromise<S, E> {
  type Target = Rc<RefCell<PromiseState<S, E>>>;
  fn deref(&self) -> &Rc<RefCell<PromiseState<S, E>>> {
    &self.promise_state
  }
}

/// An enum representing the three states of a Javascript promise.
#[derive(Clone, Debug)]
pub enum PromiseState<S, E> {
  Pending,
  Success(S),
  Error(E),
}
