use futures::Future;
use std::{
  cell::RefCell,
  ops::Deref,
  rc::Rc,
};
use wasm_bindgen::{
  closure::Closure,
  JsCast,
  JsValue,
};
use wasm_bindgen_futures::{
  future_to_promise,
  JsFuture,
};

pub struct UnwrappedPromise<S, E> {
  promise_state: Rc<RefCell<PromiseState<S, E>>>,
  // future: Box<dyn Future<Item = S, Error = E>>,
  future: Box<JsFuture>,
}

impl<S: 'static, E: 'static> UnwrappedPromise<S, E> {
  pub fn new(future: impl Future<Item = S, Error = E> + 'static) -> Self {
    let data = Rc::new(RefCell::new(PromiseState::Pending));
    let data_1 = data.clone();
    let data_2 = data.clone();

    let future = future
      .map(move |s| {
        *data_1.borrow_mut() = PromiseState::Success(s);
        // rerender();
        JsValue::NULL
      })
      .map_err(move |e| {
        *data_2.borrow_mut() = PromiseState::Error(e);
        // rerender();
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

pub enum PromiseState<S, E> {
  Pending,
  Success(S),
  Error(E),
}
