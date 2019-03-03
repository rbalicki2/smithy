extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use web_sys::{
  BeforeUnloadEvent,
  HashChangeEvent,
  PopStateEvent,
  PromiseRejectionEvent,
  UiEvent,
};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name=log)]
  pub fn log(msg: &str);

  pub type HTMLElement;

  // Selection
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_ui_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(UiEvent)>,
    should_bubble: bool,
  );

  // Window events
  pub type WINDOW;

  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_hash_change_event_listener(
    this: &WINDOW,
    event_name: &str,
    cb: &Closure<FnMut(HashChangeEvent)>,
  );
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_before_unload_event_listener(
    this: &WINDOW,
    event_name: &str,
    cb: &Closure<FnMut(BeforeUnloadEvent)>,
  );
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_pop_state_event_listener(
    this: &WINDOW,
    event_name: &str,
    cb: &Closure<FnMut(PopStateEvent)>,
  );
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_promise_rejection_event_listener(
    this: &WINDOW,
    event_name: &str,
    cb: &Closure<FnMut(PromiseRejectionEvent)>,
  );
}
