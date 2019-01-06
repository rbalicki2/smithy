use crate::{
  Component,
  EventHandled,
  Node,
  Path,
  UiEvent,
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

impl<T> Component for Vec<T>
where
  T: Component,
{
  fn render(&mut self) -> Node {
    let nodes = self.iter_mut().map(|i| i.render()).collect::<Vec<Node>>();
    Node::Vec(nodes)
  }

  fn handle_ui_event(&mut self, event: &UiEvent, path: &Path) -> EventHandled {
    // TODO maybe make this more functional
    if let Some((first, rest)) = path.split_first() {
      if let Some(target_node) = self.get_mut(*first) {
        target_node.handle_ui_event(event, rest)
      } else {
        false
      }
    } else {
      false
    }
  }
}
