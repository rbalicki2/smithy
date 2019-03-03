use crate::js_fns;
use smithy_types::{
  UiEvent,
  WindowEvent,
};
use wasm_bindgen::{
  closure::Closure,
  JsCast,
};
use web_sys::{
  AnimationEvent,
  BeforeUnloadEvent,
  ClipboardEvent,
  Event,
  FocusEvent,
  HashChangeEvent,
  HtmlElement,
  InputEvent,
  KeyboardEvent,
  MouseEvent,
  PointerEvent,
  PopStateEvent,
  PromiseRejectionEvent,
  ScrollAreaEvent,
  TouchEvent,
  TransitionEvent,
  UiEvent as WebSysUiEvent,
};

fn derive_path(s: String) -> Result<Vec<usize>, std::num::ParseIntError> {
  s.split(",").map(|s| s.parse::<usize>()).collect()
}

const DATA_SMITHY_PATH: &'static str = "data-smithy-path";

macro_rules! attach_ui_event_listener {
  (
    $html_el:expr,
    $web_sys_event_type:ident,
    $smithy_event_type:ident,
    $event_name:expr,
    $should_bubble:expr
  ) => {
    let event_handler_cb = Closure::new(move |evt: Event| {
      let evt: $web_sys_event_type = evt.unchecked_into();
      if let Some(path) = evt
        .target()
        .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        .and_then(|el| el.get_attribute(DATA_SMITHY_PATH))
        .and_then(|attr| derive_path(attr).ok())
      {
        let event_wrapped = UiEvent::$smithy_event_type(evt);
        let handle_event = move || {
          let handled = crate::handle_ui_event(&event_wrapped, &path);
          if handled {
            crate::rerender();
          }
        };

        if crate::event_handling_phase_is_ongoing() {
          let request_animation_frame_cb = Closure::wrap(Box::new(handle_event) as Box<FnMut()>);
          let window = web_sys::window().unwrap();

          let _ =
            window.request_animation_frame(request_animation_frame_cb.as_ref().unchecked_ref());
          request_animation_frame_cb.forget();
        } else {
          handle_event();
        }
      }
    });
    $html_el.attach_event_listener($event_name, &event_handler_cb, $should_bubble);
    event_handler_cb.forget();
  };
}

// TODO reuse closures
pub fn attach_ui_event_listeners(html_el: &js_fns::HTMLElement) {
  // --Clipboard
  attach_ui_event_listener!(html_el, ClipboardEvent, OnCopy, "copy", true);
  attach_ui_event_listener!(html_el, ClipboardEvent, OnCut, "cut", true);
  attach_ui_event_listener!(html_el, ClipboardEvent, OnPaste, "paste", true);

  // --Composition

  // --Keyboard
  attach_ui_event_listener!(html_el, KeyboardEvent, OnKeyDown, "keydown", false);
  attach_ui_event_listener!(html_el, KeyboardEvent, OnKeyPress, "keypress", false);
  attach_ui_event_listener!(html_el, KeyboardEvent, OnKeyUp, "keyup", false);

  // --Focus
  attach_ui_event_listener!(html_el, FocusEvent, OnFocus, "focus", false);
  attach_ui_event_listener!(html_el, FocusEvent, OnBlur, "blur", false);

  // --Form
  attach_ui_event_listener!(html_el, InputEvent, OnChange, "change", false);
  attach_ui_event_listener!(html_el, InputEvent, OnInput, "input", false);
  attach_ui_event_listener!(html_el, InputEvent, OnInvalid, "invalid", false);
  attach_ui_event_listener!(html_el, InputEvent, OnSubmit, "submit", false);

  // --Mouse
  attach_ui_event_listener!(html_el, MouseEvent, OnClick, "click", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnContextMenu, "contextmenu", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDblClick, "dblclick", false);

  attach_ui_event_listener!(html_el, MouseEvent, OnDrag, "drag", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragEnd, "dragend", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragEnter, "dragenter", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragExit, "dragexit", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragLeave, "dragleave", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragOver, "dragover", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDragStart, "dragstart", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnDrop, "drop", false);

  attach_ui_event_listener!(html_el, MouseEvent, OnMouseDown, "mousedown", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseEnter, "mouseenter", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseLeave, "mouseleave", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseMove, "mousemove", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseOver, "mouseover", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseOut, "mouseout", false);
  attach_ui_event_listener!(html_el, MouseEvent, OnMouseUp, "mouseup", false);

  // --Pointer
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerDown, "pointerdown", false);
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerMove, "pointermove", false);
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerUp, "pointerup", false);
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerCancel,
    "pointercancel",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnGotPointerCapture,
    "gotpointercapture",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnLostPointerCapture,
    "lostpointercapture",
    false
  );
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerEnter, "pointerenter", false);
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerLeave, "pointerleave", false);
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerOver, "pointerover", false);
  attach_ui_event_listener!(html_el, PointerEvent, OnPointerOut, "pointerout", false);

  // --Selection
  attach_ui_event_listener!(html_el, WebSysUiEvent, OnSelect, "onselect", false);

  // --Touch
  attach_ui_event_listener!(html_el, TouchEvent, OnTouchCancel, "touchcancel", false);
  attach_ui_event_listener!(html_el, TouchEvent, OnTouchEnd, "touchend", false);
  attach_ui_event_listener!(html_el, TouchEvent, OnTouchMove, "touchmove", false);
  attach_ui_event_listener!(html_el, TouchEvent, OnTouchStart, "touchstart", false);

  // --Scroll
  attach_ui_event_listener!(html_el, ScrollAreaEvent, OnScroll, "scroll", false);

  // --Image
  attach_ui_event_listener!(html_el, WebSysUiEvent, OnLoad, "load", false);
  attach_ui_event_listener!(html_el, WebSysUiEvent, OnError, "error", false);

  // --Animation
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationStart,
    "animationstart",
    false
  );
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationEnd,
    "animationend",
    false
  );
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationIteration,
    "animationiteration",
    false
  );

  // --Transition
  attach_ui_event_listener!(
    html_el,
    TransitionEvent,
    OnTransitionEnd,
    "transitionend",
    false
  );

  // --Other
  attach_ui_event_listener!(html_el, WebSysUiEvent, OnToggle, "toggle", false);
}

macro_rules! attach_window_event_listener {
  (
    $window:expr,
    $web_sys_event_type:ident,
    $smithy_event_type:ident,
    $event_name:expr
  ) => {
    let handle_event_cb = Closure::new(move |evt: Event| {
      let evt: $web_sys_event_type = evt.unchecked_into();
      let event_wrapped = WindowEvent::$smithy_event_type(evt);
      let handle_event = move || {
        let handled = crate::handle_window_event(&event_wrapped);
        if handled {
          crate::rerender();
        }
      };
      if crate::event_handling_phase_is_ongoing() {
        let request_animation_frame_cb = Closure::wrap(Box::new(handle_event) as Box<FnMut()>);
        let window = web_sys::window().unwrap();
        let _ = window.request_animation_frame(request_animation_frame_cb.as_ref().unchecked_ref());
        request_animation_frame_cb.forget();
      } else {
        handle_event();
      }
    });

    $window.attach_event_listener($event_name, &handle_event_cb);
    handle_event_cb.forget();
  };
}

pub fn attach_window_event_listeners(window: &js_fns::WINDOW) {
  attach_window_event_listener!(window, BeforeUnloadEvent, OnBeforeUnload, "beforeunload");
  attach_window_event_listener!(window, HashChangeEvent, OnHashChange, "hashchange");
  attach_window_event_listener!(window, PopStateEvent, OnPopState, "popstate");
  attach_window_event_listener!(
    window,
    PromiseRejectionEvent,
    OnUnhandledRejection,
    "unhandledrejection"
  );
}
