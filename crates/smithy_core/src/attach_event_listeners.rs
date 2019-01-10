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
    $window_method:ident,
    $event_name:expr,
    $should_bubble:expr
  ) => {
    let cb = Closure::new(move |evt: $web_sys_event_type| {
      if let Some(path) = evt
        .target()
        .and_then(|target| target.dyn_into::<HtmlElement>().ok())
        .and_then(|el| el.get_attribute(DATA_SMITHY_PATH))
        .and_then(|attr| derive_path(attr).ok())
      {
        let event_wrapped = UiEvent::$smithy_event_type(evt);
        let handled = crate::handle_ui_event(&event_wrapped, &path);
        if handled {
          crate::rerender();
        }
      }
    });
    $html_el.$window_method($event_name, &cb, $should_bubble);
    cb.forget();
  };
}

// TODO reuse closures
pub fn attach_ui_event_listeners(html_el: &js_fns::HTMLElement) {
  // --Clipboard
  attach_ui_event_listener!(
    html_el,
    ClipboardEvent,
    OnCopy,
    add_clipboard_event_listener,
    "copy",
    true
  );
  attach_ui_event_listener!(
    html_el,
    ClipboardEvent,
    OnCut,
    add_clipboard_event_listener,
    "cut",
    true
  );
  attach_ui_event_listener!(
    html_el,
    ClipboardEvent,
    OnPaste,
    add_clipboard_event_listener,
    "paste",
    true
  );

  // --Composition

  // --Keyboard
  attach_ui_event_listener!(
    html_el,
    KeyboardEvent,
    OnKeyDown,
    add_keyboard_event_listener,
    "keydown",
    false
  );
  attach_ui_event_listener!(
    html_el,
    KeyboardEvent,
    OnKeyPress,
    add_keyboard_event_listener,
    "keypress",
    false
  );
  attach_ui_event_listener!(
    html_el,
    KeyboardEvent,
    OnKeyUp,
    add_keyboard_event_listener,
    "keyup",
    false
  );

  // --Focus
  attach_ui_event_listener!(
    html_el,
    FocusEvent,
    OnFocus,
    add_focus_event_listener,
    "focus",
    false
  );
  attach_ui_event_listener!(
    html_el,
    FocusEvent,
    OnBlur,
    add_focus_event_listener,
    "blur",
    false
  );

  // --Form
  attach_ui_event_listener!(
    html_el,
    InputEvent,
    OnChange,
    add_input_event_listener,
    "change",
    false
  );
  attach_ui_event_listener!(
    html_el,
    InputEvent,
    OnInput,
    add_input_event_listener,
    "input",
    false
  );
  attach_ui_event_listener!(
    html_el,
    InputEvent,
    OnInvalid,
    add_input_event_listener,
    "invalid",
    false
  );
  attach_ui_event_listener!(
    html_el,
    InputEvent,
    OnSubmit,
    add_input_event_listener,
    "submit",
    false
  );

  // --Mouse
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnClick,
    add_mouse_event_listener,
    "click",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnContextMenu,
    add_mouse_event_listener,
    "contextmenu",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnClick,
    add_mouse_event_listener,
    "dblclick",
    false
  );

  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDrag,
    add_mouse_event_listener,
    "drag",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragEnd,
    add_mouse_event_listener,
    "dragend",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragEnter,
    add_mouse_event_listener,
    "dragenter",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragExit,
    add_mouse_event_listener,
    "dragexit",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragLeave,
    add_mouse_event_listener,
    "dragleave",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragOver,
    add_mouse_event_listener,
    "dragover",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDragStart,
    add_mouse_event_listener,
    "dragstart",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnDrop,
    add_mouse_event_listener,
    "drop",
    false
  );

  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseDown,
    add_mouse_event_listener,
    "mousedown",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseEnter,
    add_mouse_event_listener,
    "mouseenter",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseLeave,
    add_mouse_event_listener,
    "mouseleave",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseMove,
    add_mouse_event_listener,
    "mousemove",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseOver,
    add_mouse_event_listener,
    "mouseover",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseOut,
    add_mouse_event_listener,
    "mouseout",
    false
  );
  attach_ui_event_listener!(
    html_el,
    MouseEvent,
    OnMouseUp,
    add_mouse_event_listener,
    "mouseup",
    false
  );

  // --Pointer
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerDown,
    add_pointer_event_listener,
    "pointerdown",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerMove,
    add_pointer_event_listener,
    "pointermove",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerUp,
    add_pointer_event_listener,
    "pointerup",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerCancel,
    add_pointer_event_listener,
    "pointercancel",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnGotPointerCapture,
    add_pointer_event_listener,
    "gotpointercapture",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnLostPointerCapture,
    add_pointer_event_listener,
    "lostpointercapture",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerEnter,
    add_pointer_event_listener,
    "pointerenter",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerLeave,
    add_pointer_event_listener,
    "pointerleave",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerOver,
    add_pointer_event_listener,
    "pointerover",
    false
  );
  attach_ui_event_listener!(
    html_el,
    PointerEvent,
    OnPointerOut,
    add_pointer_event_listener,
    "pointerout",
    false
  );

  // --Selection
  attach_ui_event_listener!(
    html_el,
    WebSysUiEvent,
    OnSelect,
    add_ui_event_listener,
    "onselect",
    false
  );

  // --Touch
  attach_ui_event_listener!(
    html_el,
    TouchEvent,
    OnTouchCancel,
    add_touch_event_listener,
    "touchcancel",
    false
  );
  attach_ui_event_listener!(
    html_el,
    TouchEvent,
    OnTouchEnd,
    add_touch_event_listener,
    "touchend",
    false
  );
  attach_ui_event_listener!(
    html_el,
    TouchEvent,
    OnTouchMove,
    add_touch_event_listener,
    "touchmove",
    false
  );
  attach_ui_event_listener!(
    html_el,
    TouchEvent,
    OnTouchStart,
    add_touch_event_listener,
    "touchstart",
    false
  );

  // --Scroll
  attach_ui_event_listener!(
    html_el,
    ScrollAreaEvent,
    OnScroll,
    add_scroll_area_event_listener,
    "scroll",
    false
  );

  // --Image
  attach_ui_event_listener!(
    html_el,
    WebSysUiEvent,
    OnLoad,
    add_ui_event_listener,
    "load",
    false
  );
  attach_ui_event_listener!(
    html_el,
    WebSysUiEvent,
    OnError,
    add_ui_event_listener,
    "error",
    false
  );

  // --Animation
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationStart,
    add_animation_event_listener,
    "animationstart",
    false
  );
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationEnd,
    add_animation_event_listener,
    "animationend",
    false
  );
  attach_ui_event_listener!(
    html_el,
    AnimationEvent,
    OnAnimationIteration,
    add_animation_event_listener,
    "animationiteration",
    false
  );

  // --Transition
  attach_ui_event_listener!(
    html_el,
    TransitionEvent,
    OnTransitionEnd,
    add_transition_event_listener,
    "transitionend",
    false
  );

  // --Other
  attach_ui_event_listener!(
    html_el,
    WebSysUiEvent,
    OnToggle,
    add_ui_event_listener,
    "toggle",
    false
  );
}

macro_rules! attach_window_event_listener {
  (
    $window:expr,
    $web_sys_event_type:ident,
    $smithy_event_type:ident,
    $window_method:ident,
    $event_name:expr
  ) => {
    let cb = Closure::new(move |evt: $web_sys_event_type| {
      let event_wrapped = WindowEvent::$smithy_event_type(evt);
      let handled = crate::handle_window_event(&event_wrapped);
      if handled {
        crate::rerender();
      }
    });
    $window.$window_method($event_name, &cb);
    cb.forget();
  };
}

pub fn attach_window_event_listeners(window: &js_fns::WINDOW) {
  attach_window_event_listener!(
    window,
    BeforeUnloadEvent,
    OnBeforeUnload,
    add_before_unload_event_listener,
    "beforeunload"
  );
  attach_window_event_listener!(
    window,
    HashChangeEvent,
    OnHashChange,
    add_hash_change_event_listener,
    "hashchange"
  );
  attach_window_event_listener!(
    window,
    PopStateEvent,
    OnPopState,
    add_pop_state_event_listener,
    "popstate"
  );
  attach_window_event_listener!(
    window,
    PromiseRejectionEvent,
    OnUnhandledRejection,
    add_promise_rejection_event_listener,
    "unhandledrejection"
  );
}
