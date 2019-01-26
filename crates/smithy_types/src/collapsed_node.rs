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

impl Into<Vec<CollapsedNode>> for Node {
  fn into(self) -> Vec<CollapsedNode> {
    match self {
      Node::Dom(html_token) => vec![CollapsedNode::Dom(CollapsedHtmlToken {
        node_type: html_token.node_type,
        attributes: html_token.attributes,
        children: html_token
          .children
          .into_iter()
          .flat_map(|node| {
            let v: Vec<CollapsedNode> = node.into();
            v
          })
          .collect(),
      })],
      Node::Text(text) => vec![CollapsedNode::Text(text)],
      Node::Comment(comment_opt) => vec![CollapsedNode::Comment(comment_opt)],
      Node::Vec(vec) => vec
        .into_iter()
        .flat_map(|node| {
          let v: Vec<CollapsedNode> = node.into();
          v
        })
        .collect(),
    }
  }
}
