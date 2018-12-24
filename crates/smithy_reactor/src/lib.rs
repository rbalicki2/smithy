use js_sys::Promise;
use std::{
  cell::RefCell,
  rc::Rc,
};
use wasm_bindgen::{
  prelude::*,
  JsCast,
};

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

fn promise_timeout(duration: i32) -> Promise {
  let mut promise_closure = move |resolve: js_sys::Function, reject| {
    let timeout_closure = Closure::wrap(Box::new(move || {
      let _ = resolve.call0(&JsValue::NULL);
      web_sys::console::log_1(&"timeout elapsed!".into());
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
