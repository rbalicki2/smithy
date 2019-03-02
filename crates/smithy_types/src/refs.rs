#[derive(Debug)]
pub struct DomRef {
  element: Option<web_sys::HtmlElement>,
}

impl DomRef {
  pub fn new() -> DomRef {
    DomRef { element: None }
  }
}
