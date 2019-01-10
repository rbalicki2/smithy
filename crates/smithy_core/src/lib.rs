use smithy_types::{
  AsInnerHtml,
  Component,
  Node,
  Path,
  UiEvent,
  UnwrappedPromise,
  WindowEvent,
};
use web_sys::{
  Element,
  HashChangeEvent,
  Window,
};
mod with_inner_value;
use self::with_inner_value::*;
use futures::Future;
use std::{
  cell::RefCell,
  mem::transmute,
};
use wasm_bindgen::closure::Closure;

mod attach_event_listeners;
mod js_fns;

// TODO this should not be thread-local, but should be instantiated inside of
// mount()
thread_local! {
  static ROOT_ELEMENT: RefCell<Option<Element>> = RefCell::new(None);
  static LAST_RENDERED_NODE: RefCell<Option<Node>> = RefCell::new(None);
  static ROOT_COMPONENT: RefCell<Option<Box<Component>>> = RefCell::new(None);
}

fn get_window() -> Window {
  web_sys::window().unwrap()
}

fn mount_to_element(mut component: Box<Component>, el: &Element) {
  {
    let node = component.render();
    el.set_inner_html(&node.as_inner_html(&[]));
    LAST_RENDERED_NODE.store(node);
  }
  ROOT_COMPONENT.store(component);
}

fn handle_window_event(w: &WindowEvent) -> bool {
  ROOT_COMPONENT.with_inner_value(|root_component| root_component.handle_window_event(w))
}

fn handle_ui_event(ui_event: &UiEvent, path: &Path) -> bool {
  ROOT_COMPONENT.with_inner_value(|root_component| root_component.handle_ui_event(ui_event, &path))
}

fn attach_listeners(el: &Element) {
  let html_el = unsafe { transmute::<&Element, &js_fns::HTMLElement>(el) };
  attach_event_listeners::attach_ui_event_listeners(&html_el);

  let window = get_window();
  let window = unsafe { transmute::<Window, js_fns::WINDOW>(window) };
  attach_event_listeners::attach_window_event_listeners(&window);
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

pub fn unwrapped_promise_from_future<S: 'static, E: 'static>(
  future: impl Future<Item = S, Error = E> + 'static,
) -> UnwrappedPromise<S, E> {
  UnwrappedPromise::new(future, Some(rerender))
}
