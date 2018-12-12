use smithy_types::{
  AsInnerHtml,
  Component,
  Event,
  Node,
};
use std::cell::RefCell;
use web_sys::{
  Element,
  MouseEvent,
};
mod with_inner_value;
use self::with_inner_value::*;
use std::mem::transmute;
use wasm_bindgen::closure::Closure;

mod js_fns;

thread_local! {
  static ROOT_ELEMENT: RefCell<Option<Element>> = RefCell::new(None);
  static LAST_RENDERED_NODE: RefCell<Option<Node>> = RefCell::new(None);
  static ROOT_COMPONENT: RefCell<Option<Box<Component>>> = RefCell::new(None);
}

fn mount_to_element(mut component: Box<Component>, el: &Element) {
  {
    let node = component.render();
    el.set_inner_html(&node.as_inner_html(&[]));
    LAST_RENDERED_NODE.store(node);
  }
  ROOT_COMPONENT.store(component);
}

fn attach_listeners(el: &Element) {
  let html_el = unsafe { transmute::<&Element, &js_fns::HTMLElement>(el) };
  let cb = Closure::new(move |evt: MouseEvent| {
    js_fns::log("mouse event");

    let event_wrapped = Event::OnClick(evt);
    ROOT_COMPONENT.with_inner_value(|root_component| {
      root_component.handle_event(&event_wrapped, &[0]);
      let node = root_component.render();

      ROOT_ELEMENT.with_inner_value(|el| {
        el.set_inner_html(&node.as_inner_html(&[]));
      });
      LAST_RENDERED_NODE.store(node);
    });
  });
  html_el.add_mouse_event_listener("click", &cb, false);
  cb.forget();
}

pub fn mount(component: Box<Component>, el: Element) {
  mount_to_element(component, &el);
  attach_listeners(&el);
  ROOT_ELEMENT.store(el);
}
