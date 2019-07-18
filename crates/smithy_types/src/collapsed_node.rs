use crate::Node;

type Path = Vec<usize>;

/// An enum representing the types of nodes that can be present in the DOM.
///
/// A `Vec<CollapsedNode>` is generated from a `Node` by calling
/// `node.into_collapsed_node` on it. This will concatenate adjacent strings,
/// and flattening any `Node::Vec`'s.
///
/// That is, a `CollapsedNode` is meant to be a closer representation of the
/// DOM than a `Node`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CollapsedNode {
  Dom(CollapsedHtmlToken),
  Text(String),
  Comment(Option<String>),
}

/// A struct representing an element node in the DOM.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CollapsedHtmlToken {
  pub node_type: String,
  pub children: Vec<CollapsedNode>,
  pub attributes: crate::Attributes,
  pub path: Vec<usize>,
}

impl CollapsedHtmlToken {
  pub fn get_attributes_including_path(&self) -> crate::Attributes {
    let mut attributes = self.attributes.clone();
    attributes.insert(
      "data-smithy-path".to_string(),
      self
        .path
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>()
        .join(","),
    );
    attributes
  }
}

pub fn clone_and_extend(path: &Path, next_item: usize) -> Path {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

impl Node {
  pub fn into_collapsed_node(self, path: Path) -> Vec<CollapsedNode> {
    // Here be monsters...
    //
    // What are we doing?
    // 1. If Node is a Dom/Vec (i.e. iterable), we flat_map over each child
    // and collect that into a vec of CollapsedNode's.
    // If Node is a Text/Comment, we collect that into a vec of length 1.
    let node_vec = match self {
      Node::Dom(html_token) => vec![CollapsedNode::Dom(CollapsedHtmlToken {
        path: path.clone(),
        node_type: html_token.node_type,
        attributes: html_token.attributes,
        children: {
          // this is weird. We're wrapping children in a Node::Vec and collapsing
          // that. It would make more sense to implement Into<Vec<CollapsedNode>> on
          // Vec<Node>, presumably.
          Node::Vec(html_token.children).into()
        },
      })],
      Node::Text(text) => vec![CollapsedNode::Text(text)],
      Node::Comment(comment_opt) => vec![CollapsedNode::Comment(comment_opt)],
      Node::Vec(vec) => vec
        .into_iter()
        .enumerate()
        .flat_map(|(i, node)| node.into_collapsed_node(clone_and_extend(&path, i)))
        .collect(),
    };

    // 2. We *super jankily* combine all adjacent CollapsedNode::Text's into single
    // CollapsedNode's.
    let len = node_vec.len();
    let (mut node_vec, str_opt) = node_vec.into_iter().fold(
      (Vec::with_capacity(len), None),
      |(vec_so_far, str_opt), node| {
        let mut push = false;
        let mut ret = match (&node, &str_opt) {
          (CollapsedNode::Text(text), Some(s)) => (vec_so_far, Some(format!("{}{}", s, text))),
          (CollapsedNode::Text(text), None) => (vec_so_far, Some(text.to_string())),
          _ => {
            push = true;
            (vec_so_far, str_opt)
          },
        };
        let ret = if push {
          if let Some(s) = ret.1 {
            ret.0.push(CollapsedNode::Text(s));
          };
          ret.0.push(node);
          (ret.0, None)
        } else {
          ret
        };

        ret
      },
    );

    // 3. If there were terminal CollapsedNode::Text's, we need to push those onto the vec.
    if let Some(s) = str_opt {
      node_vec.push(CollapsedNode::Text(s));
    }

    node_vec
  }
}

impl Into<Vec<CollapsedNode>> for Node {
  // TODO collapse text nodes
  fn into(self) -> Vec<CollapsedNode> {
    self.into_collapsed_node(vec![])
  }
}
