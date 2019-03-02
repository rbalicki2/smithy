use web_sys::HtmlElement;

// TODO impl Deref<Option<HtmlElement>>

#[derive(Debug)]
pub struct DomRef {
  pub element_opt: Option<HtmlElement>,
  pub name: String,
}

impl DomRef {
  pub fn new(name: String) -> DomRef {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
      "- creating ref with name: {}",
      name
    )));
    DomRef {
      element_opt: None,
      name,
    }
  }

  pub fn set(&mut self, element_opt: Option<HtmlElement>) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&self.name));
    self.element_opt = element_opt;
  }

  pub fn get(&self) -> &Option<HtmlElement> {
    &self.element_opt
  }
}

pub type DomRefWithPath<'a> = (Vec<usize>, &'a mut DomRef);
