extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use web_sys::Event;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name=log)]
  pub fn log(msg: &str);

  pub type HTMLElement;

  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn attach_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<dyn FnMut(Event)>,
    should_bubble: bool,
  );

  pub type WINDOW;

  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn attach_event_listener(this: &WINDOW, event_name: &str, cb: &Closure<dyn FnMut(Event)>);
}
