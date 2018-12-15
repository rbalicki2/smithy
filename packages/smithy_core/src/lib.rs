use smithy_types::{
  AsInnerHtml,
  Component,
  Node,
  UiEvent,
  WindowEvent,
};
use std::cell::RefCell;
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
use std::mem::transmute;
use wasm_bindgen::{
  closure::Closure,
  JsCast,
};

mod js_fns;

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
      ROOT_COMPONENT.with_inner_value(|root_component| {
        let handled = root_component.handle_ui_event(&event_wrapped, &path);

        if handled {
          let node = root_component.render();

          ROOT_ELEMENT.with_inner_value(|el| {
            el.set_inner_html(&node.as_inner_html(&[]));
          });
          LAST_RENDERED_NODE.store(node);
        }
      });
    }
  });
  html_el.add_mouse_event_listener("click", &cb, false);
  cb.forget();

  let window = get_window();
  let window = unsafe { transmute::<Window, js_fns::WINDOW>(window) };
  // hashchange
  let cb = Closure::new(move |evt: HashChangeEvent| {
    let event_wrapped = WindowEvent::OnHashChange(evt);
    ROOT_COMPONENT.with_inner_value(|root_component| {
      let handled = root_component.handle_window_event(&event_wrapped);
      if handled {
        let node = root_component.render();

        ROOT_ELEMENT.with_inner_value(|el| {
          el.set_inner_html(&node.as_inner_html(&[]));
        });
        LAST_RENDERED_NODE.store(node);
      }
    });
  });
  window.add_hash_change_event_listener("hashchange", &cb);
  cb.forget();
}

pub fn mount(component: Box<Component>, el: Element) {
  mount_to_element(component, &el);
  attach_listeners(&el);
  ROOT_ELEMENT.store(el);
}
