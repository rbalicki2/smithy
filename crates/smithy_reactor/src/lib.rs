use js_sys::Promise;
use wasm_bindgen::{
  prelude::*,
  JsCast,
};

pub fn promise_timeout(duration: i32) -> Promise {
  let mut promise_closure = move |resolve: js_sys::Function, _reject| {
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
