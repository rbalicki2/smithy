use crate::{
  Component,
  EventHandled,
  Node,
  Path,
  UiEvent,
};

/**
 * N.B. this is subject to change! We want to be smart about how we
 * impl Component for common types, especially related to container types
 * (Vec, Option, Box, etc.)
 */

macro_rules! basic_impl_component {
  ($type:ty) => {
    impl Component for $type {
      fn render(&mut self) -> Node {
        Node::Text(self.to_string())
      }
    }
  };
}

basic_impl_component!(&str);

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

impl<T> Component for Option<T>
where
  T: Component,
{
  fn render(&mut self) -> Node {
    match self {
      Some(t) => t.render(),
      None => Node::Comment(None),
    }
  }

  fn handle_ui_event(&mut self, event: &UiEvent, path: &Path) -> EventHandled {
    match self {
      Some(t) => t.handle_ui_event(event, path),
      None => false,
    }
  }
}

basic_impl_component!(bool);
basic_impl_component!(char);
basic_impl_component!(i8);
basic_impl_component!(i16);
basic_impl_component!(i32);
basic_impl_component!(i64);
basic_impl_component!(isize);
basic_impl_component!(u8);
basic_impl_component!(u16);
basic_impl_component!(u32);
basic_impl_component!(u64);
basic_impl_component!(usize);
basic_impl_component!(f32);
basic_impl_component!(f64);
