use crate::{
  Component,
  Node,
};

impl Component for &str {
  fn render(&mut self) -> Node {
    Node::Text(self.to_string())
  }
}

impl Component for String {
  fn render(&mut self) -> Node {
    Node::Text(self.clone())
  }
}

impl Component for &String {
  fn render(&mut self) -> Node {
    Node::Text((*self).to_string())
  }
}
