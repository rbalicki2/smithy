use smithy_types::{
  AsInnerHtml,
  Component,
  Node,
  Path,
  UiEvent,
  WindowEvent,
};
use web_sys::{
  Element,
  HashChangeEvent,
  HtmlElement,
  MouseEvent,
  Window,
};
mod with_inner_value;
use self::with_inner_value::*;
use js_sys::{
  global,
  Object,
};
use std::{
  cell::RefCell,
  mem::transmute,
  rc::Rc,
};
use wasm_bindgen::{
  closure::Closure,
  JsCast,
};

pub use smithy_reactor::UnwrappedPromise;

mod js_fns;

// TODO this should not be thread-local, but should be instantiated inside of
// mount()
thread_local! {
  static ROOT_ELEMENT: RefCell<Option<Element>> = RefCell::new(None);
  static LAST_RENDERED_NODE: RefCell<Option<Node>> = RefCell::new(None);
  static ROOT_COMPONENT: RefCell<Option<Box<Component>>> = RefCell::new(None);
}

fn get_window() -> Window {
  unsafe { transmute::<Object, Window>(global()) }
}

fn mount_to_element(mut component: Box<Component>, el: &Element) {
  {
    let node = component.render();
    el.set_inner_html(&node.as_inner_html(&[]));
    LAST_RENDERED_NODE.store(node);
  }
  ROOT_COMPONENT.store(component);
}

fn derive_path(s: String) -> Result<Vec<usize>, std::num::ParseIntError> {
  s.split(",").map(|s| s.parse::<usize>()).collect()
}

fn handle_window_event(w: &WindowEvent) -> bool {
  ROOT_COMPONENT.with_inner_value(|root_component| root_component.handle_window_event(w))
}

fn handle_ui_event(ui_event: &UiEvent, path: &Path) -> bool {
  ROOT_COMPONENT.with_inner_value(|root_component| root_component.handle_ui_event(ui_event, &path))
}

fn attach_listeners(el: &Element) {
  let html_el = unsafe { transmute::<&Element, &js_fns::HTMLElement>(el) };

  // click
  let cb = Closure::new(move |evt: MouseEvent| {
    if let Some(path) = evt
      .target()
      .and_then(|target| target.dyn_into::<HtmlElement>().ok())
      .and_then(|el| el.get_attribute("data-smithy-path"))
      .and_then(|attr| derive_path(attr).ok())
    {
      let event_wrapped = UiEvent::OnClick(evt);
      let handled = handle_ui_event(&event_wrapped, &path);
      if handled {
        rerender();
      }
    }
  });
  html_el.add_mouse_event_listener("click", &cb, false);
  cb.forget();

  let window = get_window();
  let window = unsafe { transmute::<Window, js_fns::WINDOW>(window) };
  // hashchange
  let cb = Closure::new(move |evt: HashChangeEvent| {
    let event_wrapped = WindowEvent::OnHashChange(evt);
    let handled = handle_window_event(&event_wrapped);
    if handled {
      rerender();
    }
  });
  window.add_hash_change_event_listener("hashchange", &cb);
  cb.forget();
}

pub fn rerender() {
  ROOT_COMPONENT.with_inner_value(|root_component| {
    let node = root_component.render();

    ROOT_ELEMENT.with_inner_value(|el| {
      el.set_inner_html(&node.as_inner_html(&[]));
    });
    LAST_RENDERED_NODE.store(node);
  });
}

pub fn mount(component: Box<Component>, el: Element) {
  mount_to_element(component, &el);
  attach_listeners(&el);
  ROOT_ELEMENT.store(el);
}

pub fn promise_from_timeout(
  duration: i32,
) -> Rc<RefCell<UnwrappedPromise<wasm_bindgen::JsValue, wasm_bindgen::JsValue>>> {
  smithy_reactor::promise_from_timeout(Box::new(rerender), duration)
}
