use crate::{
  HtmlToken,
  Node,
};

type Path = Vec<usize>;

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
  pub path: Vec<usize>,
}

fn clone_and_extend(path: &Path, next_item: usize) -> Path {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

impl Node {
  pub fn into_collapsed_node(self, path: Path) -> Vec<CollapsedNode> {
    match self {
      Node::Dom(html_token) => vec![CollapsedNode::Dom(CollapsedHtmlToken {
        path: path.clone(),
        node_type: html_token.node_type,
        attributes: html_token.attributes,
        children: html_token
          .children
          .into_iter()
          .enumerate()
          .flat_map(|(i, node)| node.into_collapsed_node(clone_and_extend(&path, i)))
          .collect(),
      })],
      Node::Text(text) => vec![CollapsedNode::Text(text)],
      Node::Comment(comment_opt) => vec![CollapsedNode::Comment(comment_opt)],
      Node::Vec(vec) => vec
        .into_iter()
        .enumerate()
        .flat_map(|(i, node)| node.into_collapsed_node(clone_and_extend(&path, i)))
        .collect(),
    }
  }
}

impl Into<Vec<CollapsedNode>> for Node {
  // TODO collapse text nodes
  fn into(self) -> Vec<CollapsedNode> {
    self.into_collapsed_node(vec![])
  }
}
