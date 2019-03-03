use web_sys::HtmlElement;

// TODO impl Deref<Option<HtmlElement>>

#[derive(Debug)]
pub struct DomRef {
  pub element_opt: Option<HtmlElement>,
}

impl DomRef {
  pub fn new() -> DomRef {
    DomRef { element_opt: None }
  }

  pub fn set(&mut self, element_opt: Option<HtmlElement>) {
    self.element_opt = element_opt;
  }

  pub fn get(&self) -> &Option<HtmlElement> {
    &self.element_opt
  }
}

pub type DomRefWithPath<'a> = (Vec<usize>, &'a mut DomRef);
