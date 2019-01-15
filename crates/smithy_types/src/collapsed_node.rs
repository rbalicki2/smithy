use crate::{
  HtmlToken,
  Node,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CollapsedNode {
  Dom(CollapsedHtmlToken),
  Text(String),
  Comment(Option<String>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CollapsedHtmlToken {
  pub node_type: String,
  pub children: Vec<CollapsedNode>,
  pub attributes: crate::Attributes,
}

impl Into<CollapsedNode> for Node {
  fn into(self) -> CollapsedNode {
    match self {
      Node::Dom(html_token) => unimplemented!(),
      Node::Text(text) => CollapsedNode::Text(text),
      Node::Comment(comment_opt) => CollapsedNode::Comment(comment_opt),
      Node::Vec(vec) => panic!("Can't call .into() on a Node::Vec, this shouldn't happen."),
    }
  }
}

impl Into<CollapsedHtmlToken> for HtmlToken {
  fn into(self) -> CollapsedHtmlToken {
    unimplemented!()
  }
}
