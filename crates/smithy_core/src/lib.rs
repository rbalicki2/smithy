use smithy_types::{
  AsInnerHtml,
  CollapsedNode,
  Component,
  Path,
  UiEvent,
  UnwrappedPromise,
  WindowEvent,
};
use web_sys::{
  Element,
  Window,
};
mod with_inner_value;
use self::with_inner_value::*;
use futures::Future;
use std::{
  cell::RefCell,
  mem::transmute,
};

mod attach_event_listeners;
mod js_fns;
mod node_diff;
mod zip_util;

use self::node_diff::{
  ApplicableTo,
  Diffable,
};

// TODO this should not be thread-local, but should be instantiated inside of
// mount()
thread_local! {
  static ROOT_ELEMENT: RefCell<Option<Element>> = RefCell::new(None);
  static LAST_RENDERED_NODE: RefCell<Option<Vec<CollapsedNode>>> = RefCell::new(None);
  static ROOT_COMPONENT: RefCell<Option<Box<Component>>> = RefCell::new(None);
  static EVENT_DEPTH: RefCell<u32> = RefCell::new(0);
}

fn get_window() -> Window {
  web_sys::window().unwrap()
}

fn mount_to_element(mut component: Box<Component>, el: &Element) {
  {
    let node: Vec<CollapsedNode> = component.render().into();
    el.set_inner_html(&node.as_inner_html());
    LAST_RENDERED_NODE.store(node);
  }
  ROOT_COMPONENT.store(component);
}

fn with_increased_event_depth<T>(f: impl Fn() -> T) -> T {
  EVENT_DEPTH.with(|depth| {
    let existing_depth = *depth.borrow();
    *depth.borrow_mut() = existing_depth + 1;
  });
  let ret = f();
  EVENT_DEPTH.with(|depth| {
    let existing_depth = *depth.borrow();
    *depth.borrow_mut() = existing_depth - 1;
  });
  ret
}

fn get_event_depth() -> u32 {
  EVENT_DEPTH.with(|depth| *depth.borrow())
}

fn event_handling_phase_is_ongoing() -> bool {
  get_event_depth() > 0
}

fn handle_window_event(w: &WindowEvent) -> bool {
  with_increased_event_depth(|| {
    ROOT_COMPONENT.with_inner_value(|root_component| root_component.handle_window_event(w))
  })
}

fn handle_ui_event(ui_event: &UiEvent, path: &Path) -> bool {
  with_increased_event_depth(|| {
    ROOT_COMPONENT
      .with_inner_value(|root_component| root_component.handle_ui_event(ui_event, &path))
  })
}

fn attach_listeners(el: &Element) {
  let html_el = unsafe { transmute::<&Element, &js_fns::HTMLElement>(el) };
  attach_event_listeners::attach_ui_event_listeners(&html_el);

  let window = get_window();
  let window = unsafe { transmute::<Window, js_fns::WINDOW>(window) };
  attach_event_listeners::attach_window_event_listeners(&window);
}

fn convert_node_list_to_vec(node_list: &web_sys::NodeList) -> Vec<web_sys::Node> {
  let len = node_list.length();
  let mut vec = Vec::with_capacity(len as usize);
  for i in 0..len {
    vec.push(node_list.get(i).unwrap());
  }
  vec
}

pub fn rerender() {
  ROOT_COMPONENT.with_inner_value(|root_component| {
    // We need to also emit information about the collapsing process for the post-render method later
    // e.g. let newly_rendered_nodes = (Vec<CollapsedNode>, Vec<usize>) = root_component.render().into();
    let newly_rendered_nodes: Vec<CollapsedNode> = root_component.render().into();

    LAST_RENDERED_NODE.with_inner_value(|last_rendered_node| {
      let diff = last_rendered_node.get_diff_with(&newly_rendered_nodes);
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "\n\n\nrerender\n------------------------\n\nfrom {:?}\n\nto {:?}\n\ndiff {:#?}\n\n",
        last_rendered_node.as_inner_html(),
        newly_rendered_nodes.as_inner_html(),
        diff
      )));
      ROOT_ELEMENT.with_inner_value(|el| {
        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
          "\n\nroot el inner {:?}",
          el.inner_html()
        )));
        for diff_item in diff.iter() {
          diff_item.apply_to(el);
        }
      });
    });

    ROOT_ELEMENT.with_inner_value(|el| {
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "bout to call post_render {:?}",
        newly_rendered_nodes
      )));
      root_component.handle_ref_assignment();
      root_component.handle_post_render(
        &newly_rendered_nodes.split_node_list(convert_node_list_to_vec(&el.child_nodes())),
      );
    });
    LAST_RENDERED_NODE.store(newly_rendered_nodes);
  });
}

pub fn mount(component: Box<Component>, el: Element) {
  console_error_panic_hook::set_once();
  mount_to_element(component, &el);
  attach_listeners(&el);
  ROOT_ELEMENT.store(el);
  // N.B. maybe (probably) we don't want this; it's here for ease of testing.
  rerender();
}

pub fn unwrapped_promise_from_future<S: 'static, E: 'static>(
  future: impl Future<Item = S, Error = E> + 'static,
) -> UnwrappedPromise<S, E> {
  UnwrappedPromise::new(future, Some(rerender))
}
