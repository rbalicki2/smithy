#[cfg(test)]
mod tests {
  use smithy::{
    self,
    smd,
    types::*,
  };
  use std::collections::HashMap;

  fn get_bare_div() -> Node {
    Node::Dom(HtmlToken {
      node_type: "div".into(),
      attributes: HashMap::new(),
      children: vec![],
    })
  }

  fn get_bare_div_as_vec() -> Node {
    Node::Vec(vec![get_bare_div()])
  }

  #[test]
  fn self_closing_div() {
    let mut div = smd!(<div />);
    assert_eq!(div.render(), get_bare_div_as_vec());
  }

  #[test]
  fn div() {
    let mut div = smd!(<div></div>);
    assert_eq!(div.render(), get_bare_div_as_vec());
  }

  #[test]
  fn div_with_html_children() {
    let mut div = smd!(<div><h1 /></div>);
    let result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "div".into(),
      attributes: HashMap::new(),
      children: vec![Node::Dom(HtmlToken {
        node_type: "h1".into(),
        attributes: HashMap::new(),
        children: vec![],
      })],
    })]);
    assert_eq!(div.render(), result);
  }

  #[test]
  fn div_with_text_children() {
    let mut div = smd!(<div>hello</div>);
    let result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "div".into(),
      attributes: HashMap::new(),
      children: vec![Node::Text("hello".into())],
    })]);
    assert_eq!(div.render(), result);
  }

  #[test]
  fn div_with_group_component_children() {
    let mut inner = smd!(<inner />);
    let mut outer = smd!(<outer>{ &mut inner }</outer>);
    let result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "outer".into(),
      attributes: HashMap::new(),
      children: vec![Node::Vec(vec![Node::Dom(HtmlToken {
        node_type: "inner".into(),
        attributes: HashMap::new(),
        children: vec![],
      })])],
    })]);
    assert_eq!(outer.render(), result);
  }

  #[test]
  fn div_with_group_text_children() {
    let mut outer = smd!(<outer>{ "inner" }</outer>);
    let result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "outer".into(),
      attributes: HashMap::new(),
      children: vec![Node::Text("inner".into())],
    })]);
    assert_eq!(outer.render(), result);
  }

  #[test]
  fn multiple_adjacent_divs() {
    let mut divs = smd!(<div /><div />);
    let result = Node::Vec(vec![get_bare_div(), get_bare_div()]);
    assert_eq!(divs.render(), result);
  }

  #[test]
  fn empty_macro() {
    // let mut no_dom = smd!();
    // let result = Node::Vec(vec![]);
    // assert_eq!(no_dom.render(), result);
  }
}
