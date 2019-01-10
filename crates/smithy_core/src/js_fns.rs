extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use web_sys::{
  AnimationEvent,
  BeforeUnloadEvent,
  ClipboardEvent,
  FocusEvent,
  HashChangeEvent,
  InputEvent,
  KeyboardEvent,
  MouseEvent,
  PointerEvent,
  PopStateEvent,
  PromiseRejectionEvent,
  ScrollAreaEvent,
  TouchEvent,
  TransitionEvent,
  UiEvent,
};

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console, js_name=log)]
  pub fn log(msg: &str);

  // N.B. we need to create methods for all of the events that we are doing
  // since wasm_bindgen only creates set_onclick etc handlers that take closures
  // with no params.
  // TODO ensure this is still the case
  pub type HTMLElement;

  // Selection
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_ui_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(UiEvent)>,
    should_bubble: bool,
  );

  // --Clipboard
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_clipboard_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(ClipboardEvent)>,
    should_bubble: bool,
  );
  // --Composition
  // onCompositionEnd
  // onCompositionStart
  // onCompositionUpdate
  // --Keyboard
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_keyboard_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(KeyboardEvent)>,
    should_bubble: bool,
  );
  // // --Focus
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_focus_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(FocusEvent)>,
    should_bubble: bool,
  );
  // // --Form
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_input_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(InputEvent)>,
    should_bubble: bool,
  );
  // --Mouse
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_mouse_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(MouseEvent)>,
    should_bubble: bool,
  );
  // // --Pointer
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_pointer_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(PointerEvent)>,
    should_bubble: bool,
  );
  // // --Selection
  // // uses add_ui_event_listener
  // // --Touch
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_touch_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(TouchEvent)>,
    should_bubble: bool,
  );
  // // --Scroll
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_scroll_area_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(ScrollAreaEvent)>,
    should_bubble: bool,
  );
  // // --Wheel
  // // onWheel
  // // --Media
  // // onAbort
  // // onCanPlay
  // // onCanPlayThrough
  // // onDurationChange
  // // onEmptied
  // // onEncrypted
  // // onEnded
  // // onError
  // // onLoadedData
  // // onLoadedMetadata
  // // onLoadStart
  // // onPause
  // // onPlay
  // // onPlaying
  // // onProgress
  // // onRateChange
  // // onSeeked
  // // onSeeking
  // // onStalled
  // // onSuspend
  // // onTimeUpdate
  // // onVolumeChange
  // // onWaiting
  // // --Image
  // // onLoad, onError both use add_ui_event_listener
  // // --Animation
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_animation_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(AnimationEvent)>,
    should_bubble: bool,
  );
  // // --Transition
  #[wasm_bindgen(method, js_name=addEventListener)]
  pub fn add_transition_event_listener(
    this: &HTMLElement,
    event_name: &str,
    cb: &Closure<FnMut(TransitionEvent)>,
    should_bubble: bool,
  );
  // // --Other
  // // onToggle uses add_ui_event_listener

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
