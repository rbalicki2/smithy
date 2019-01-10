use crate::js_fns;
use smithy_types::UiEvent;
use wasm_bindgen::{
  closure::Closure,
  JsCast,
};
use web_sys::{
  AnimationEvent,
  ClipboardEvent,
  FocusEvent,
  HashChangeEvent,
  HtmlElement,
  InputEvent,
  KeyboardEvent,
  MouseEvent,
  PointerEvent,
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
    // click
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

pub fn attach_ui_event_listeners(html_el: &js_fns::HTMLElement) {
  // Clipboard
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

  // Composition

  // Keyboard
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

  // Focus
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

  // Form
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

  // Mouse
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
}
