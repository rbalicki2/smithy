use smithy_types::{
  AsInnerHtml,
  Component,
  SmithyComponent,
};
use wasm_bindgen::JsValue;
use web_sys::{
  console,
  Element,
};

pub fn mount(mut app: Vec<SmithyComponent>, el: Element) {
  let html: String = app.render().as_inner_html();
  console::log_1(&JsValue::from_str(&html));

  el.set_inner_html(&html);
}
