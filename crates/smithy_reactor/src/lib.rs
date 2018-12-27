use futures::Future;
use js_sys::Promise;
use std::{
  cell::RefCell,
  rc::Rc,
};
use wasm_bindgen::{
  prelude::*,
  JsCast,
};
use wasm_bindgen_futures::{
  future_to_promise,
  JsFuture,
};
use web_sys::console::log_1;

thread_local! {
  pub static RERENDER: RefCell<Option<Box<Fn() + 'static>>> = RefCell::new(None);
}

pub enum UnwrappedPromise<S, E> {
  Pending,
  Success(S),
  Error(E),
}

impl UnwrappedPromise<JsValue, JsValue> {
  pub fn from_js_promise(p: &Promise) -> Rc<RefCell<Self>> {
    let unwrapped = Rc::new(RefCell::new(UnwrappedPromise::Pending));
    let unwrapped_ref_success = unwrapped.clone();
    let unwrapped_ref_err = unwrapped.clone();
    let c1 = Closure::new(move |v| {
      let mut unwrapped = unwrapped_ref_success.borrow_mut();
      *unwrapped = UnwrappedPromise::Success(v);
    });
    let c2 = Closure::new(move |e| {
      let mut unwrapped = unwrapped_ref_err.borrow_mut();
      *unwrapped = UnwrappedPromise::Error(e);
    });
    p.then2(&c1, &c2);
    c1.forget();
    c2.forget();
    unwrapped
  }
}

// N.B. 'static here smells, but is required by future_to_promise
impl<S: 'static, E: 'static> UnwrappedPromise<S, E> {
  pub fn from_future(
    future: impl Future<Item = S, Error = E> + 'static,
  ) -> Rc<RefCell<UnwrappedPromise<S, E>>> {
    let data = Rc::new(RefCell::new(UnwrappedPromise::Pending));
    let data_1 = data.clone();

    let future = Box::new(
      future
        .map(move |s| {
          log_1(&JsValue::from_str("future cb"));
          *data_1.borrow_mut() = UnwrappedPromise::Success(s);
          RERENDER.with(|rerender| {
            let rerender = rerender.borrow();
            if let Some(ref rerender) = *rerender {
              log_1(&JsValue::from_str("rerendering"));
              rerender();
            } else {
              log_1(&JsValue::from_str("rerender not found"));
            }
          });
          JsValue::NULL
        })
        .map_err(|_| JsValue::NULL),
    );
    let future = future_to_promise(future);
    std::mem::forget(future);
    data
  }
}

pub fn promise_timeout(duration: i32) -> Promise {
  let mut promise_closure = move |resolve: js_sys::Function, reject| {
    web_sys::console::log_1(&"outer promise closure".into());
    let timeout_closure = Closure::wrap(Box::new(move || {
      let _ = resolve.call0(&JsValue::NULL);
      web_sys::console::log_1(&"inner timeout closure".into());
    }) as Box<FnMut()>);
    let window = web_sys::window().unwrap();

    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
      timeout_closure.as_ref().unchecked_ref(),
      duration,
    );
    timeout_closure.forget();
  };
  let promise = Promise::new(&mut promise_closure);
  promise
}

pub fn promise_from_timeout(
  cb: Box<Fn()>,
  duration: i32,
) -> Rc<RefCell<UnwrappedPromise<JsValue, JsValue>>> {
  web_sys::console::log_1(&JsValue::from_str("1"));
  let promise = promise_timeout(duration);

  let unwrapped = UnwrappedPromise::from_js_promise(&promise);
  let closure = Closure::new(move |_| cb());
  promise.then2(&closure, &closure);
  closure.forget();

  web_sys::console::log_1(&JsValue::from_str("2"));
  unwrapped
}
