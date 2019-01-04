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
  HtmlElement,
  MouseEvent,
  Window,
};
mod with_inner_value;
use self::with_inner_value::*;
use futures::Future;
use std::{
  cell::RefCell,
  mem::transmute,
};
use wasm_bindgen::{
  closure::Closure,
  JsCast,
};
use wasm_bindgen_futures::JsFuture;

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

pub fn rerender_in_timeout() {
  // TODO use Promise.resolve().then to make it faster...?
  let timeout_closure = Closure::wrap(Box::new(rerender) as Box<FnMut()>);
  let _ = get_window().set_timeout_with_callback_and_timeout_and_arguments_0(
    timeout_closure.as_ref().unchecked_ref(),
    0,
  );
  // TODO how do we avoid this memory leak?
  timeout_closure.forget();
}

pub fn mount(component: Box<Component>, el: Element) {
  mount_to_element(component, &el);
  attach_listeners(&el);
  ROOT_ELEMENT.store(el);
}

pub fn unwrapped_promise_from_future<S: 'static, E: 'static>(
  future: impl Future<Item = S, Error = E> + 'static,
) -> UnwrappedPromise<S, E> {
  UnwrappedPromise::new(
    future,
    Some(rerender)
      // .map(|s| {
      //   rerender_in_timeout();
      //   s
      // })
      // .map_err(|e| {
      //   rerender_in_timeout();
      //   e
      // }),
  )
}
