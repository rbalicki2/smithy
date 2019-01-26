use smithy_types::{
  AsInnerHtml,
  Attributes,
  CollapsedHtmlToken,
  CollapsedNode,
};
use std::{
  cmp::max,
  iter::repeat_with,
};

type NewInnerHtml = String;

pub type Path = Vec<usize>;

#[derive(Debug)]
pub struct ReplaceOperation {
  pub new_inner_html: NewInnerHtml,
}

#[derive(Debug)]
pub struct InsertOperation {
  pub new_inner_html: NewInnerHtml,
}

#[derive(Debug)]
pub struct DeleteOperation {}

#[derive(Debug)]
pub struct UpdateAttributesOperation {
  pub new_attributes: Attributes,
}

#[derive(Debug)]
pub enum DiffOperation {
  Replace(ReplaceOperation),
  Insert(InsertOperation),
  Delete(DeleteOperation),
  UpdateAttributes(UpdateAttributesOperation),
}

pub type DiffItem = (Path, DiffOperation);
pub type Diff = Vec<DiffItem>;

pub trait Diffable {
  fn get_diff_with(&self, other: &Self) -> Diff;
}

/**
 * Diffing algorithm
 *
 * - If both are strings, compare strings
 * - If both are comments, compare contents
 * - If both are DOM elements...
 *   - If the node_type and attributes are the same, keep it, and:
 *     - For each existing child, if it has the same node_type,
 *       keep it, and recurse
 *   - If it has a different node_type or attributes, add it
 *   - For each additional new child, add it
 *
 *   - Thus <div><h1><h2 /><h1></div> to <div><h1><h3 /></h1></div>
 *     should see that the div, is the same and the h1 is the same,
 *     and see that h2 !== h3, and create a diff operation for that.
 *
 * - If they differ in type, replace one with the other
 */
impl Diffable for Vec<CollapsedNode> {
  fn get_diff_with(&self, other: &Self) -> Diff {
    vec![]
    // let mut diff = get_path_diff(self, other, vec![]);
    // diff.reverse();
    // diff
  }
}

fn get_path_diff(old_node: &CollapsedNode, new_node: &CollapsedNode, path: Path) -> Diff {
  match (old_node, new_node) {
    (CollapsedNode::Dom(ref old_token), CollapsedNode::Dom(ref new_token)) => {
      // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("dom"));
      get_html_token_diff(old_token, new_token, path)
    },
    (CollapsedNode::Text(ref old_text), CollapsedNode::Text(ref new_text)) => {
      get_text_diff(old_text, new_text, path)
    },
    (CollapsedNode::Comment(ref old_comment), CollapsedNode::Comment(ref new_comment)) => {
      get_comment_diff(old_comment, new_comment, path)
    },
    _ => get_replace_diff(new_node, path),
  }
}

fn optionalize_and_extend_with_none<T>(
  iter: impl Iterator<Item = T>,
) -> impl Iterator<Item = Option<T>> {
  iter.map(|item| Some(item)).chain(repeat_with(|| None))
}

fn optionalize_and_zip<T, U>(
  left_iter: impl ExactSizeIterator<Item = T>,
  right_iter: impl ExactSizeIterator<Item = U>,
) -> impl Iterator<Item = (Option<T>, Option<U>)> {
  let max_len = max(left_iter.len(), right_iter.len());
  let left_optionalized = optionalize_and_extend_with_none(left_iter);
  let right_optionalized = optionalize_and_extend_with_none(right_iter);
  left_optionalized.zip(right_optionalized).take(max_len)
}

fn clone_and_extend(path: &Path, next_item: usize) -> Path {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

fn get_html_token_diff(
  old_token: &CollapsedHtmlToken,
  new_token: &CollapsedHtmlToken,
  path: Path,
) -> Diff {
  /**
   * If the node_type's are different, we replace
   * If they're the same, we potentially change attributes
   * And call get_path_diff on each zipped child
   */
  let old_node_type = &old_token.node_type;
  let new_node_type = &new_token.node_type;
  if old_node_type != new_node_type {
    let new_inner_html = new_token.as_inner_html(&path);
    vec![(
      path,
      DiffOperation::Replace(ReplaceOperation { new_inner_html }),
    )]
  } else {
    let iter = optionalize_and_zip(old_token.children.iter(), new_token.children.iter());
    iter
      .enumerate()
      .flat_map(|(i, zipped)| match zipped {
        (Some(old_child), Some(new_child)) => {
          // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("some old some new"));
          get_path_diff(old_child, new_child, clone_and_extend(&path, i))
        },
        (Some(old_child), None) => vec![(
          clone_and_extend(&path, i),
          DiffOperation::Delete(DeleteOperation {}),
        )],
        (None, Some(new_child)) => vec![(
          clone_and_extend(&path, i),
          DiffOperation::Insert(InsertOperation {
            new_inner_html: new_child.as_inner_html(&path),
          }),
        )],
        _ => panic!("We should not encounter two None's in get_html_token_diff"),
      })
      .collect::<Vec<DiffItem>>()
    // TODO attributes
  }
}

fn get_text_diff(old_text: &String, new_text: &String, path: Path) -> Diff {
  if old_text != new_text {
    vec![(
      path,
      DiffOperation::Replace(ReplaceOperation {
        new_inner_html: new_text.to_string(),
      }),
    )]
  } else {
    vec![]
  }
}

fn get_comment_diff(
  old_comment_opt: &Option<String>,
  new_comment_opt: &Option<String>,
  path: Path,
) -> Diff {
  match (old_comment_opt, new_comment_opt) {
    (Some(old_comment), Some(new_comment)) => get_text_diff(old_comment, new_comment, path),
    (Some(_), None) => vec![(
      path,
      DiffOperation::Replace(ReplaceOperation {
        // I think?
        new_inner_html: "<!-- -->".to_string(),
      }),
    )],
    (None, Some(new_comment)) => vec![(
      path,
      DiffOperation::Replace(ReplaceOperation {
        new_inner_html: format!("<!-- {} -->", new_comment),
      }),
    )],
    (None, None) => vec![],
  }
}

fn get_replace_diff(new_node: &CollapsedNode, path: Path) -> Diff {
  let new_inner_html = new_node.as_inner_html(&path);
  vec![(
    path,
    DiffOperation::Replace(ReplaceOperation { new_inner_html }),
  )]
}

// OLD
// use smithy_types::{
//   AsInnerHtml,
//   Attributes,
//   HtmlToken,
//   Node,
// };
// use std::{
//   cmp::max,
//   iter::repeat_with,
// };

// type NewInnerHtml = String;

// pub type Path = Vec<usize>;

// #[derive(Debug)]
// pub struct ReplaceOperation {
//   pub new_inner_html: NewInnerHtml,
// }

// #[derive(Debug)]
// pub struct InsertOperation {
//   pub new_inner_html: NewInnerHtml,
// }

// #[derive(Debug)]
// pub struct DeleteOperation {}

// #[derive(Debug)]
// pub struct UpdateAttributesOperation {
//   pub new_attributes: Attributes,
// }

// #[derive(Debug)]
// pub enum DiffOperation {
//   Replace(ReplaceOperation),
//   Insert(InsertOperation),
//   Delete(DeleteOperation),
//   UpdateAttributes(UpdateAttributesOperation),
// }

// pub type DiffItem = (Path, DiffOperation);
// pub type Diff = Vec<DiffItem>;

// pub trait Diffable {
//   fn get_diff_with(&self, other: &Self) -> Diff;
// }

// /**
//  * Diffing algorithm
//  *
//  * - If both are strings, compare strings
//  * - If both are comments, compare contents
//  * - If both are DOM elements...
//  *   - If the node_type and attributes are the same, keep it, and:
//  *     - For each existing child, if it has the same node_type,
//  *       keep it, and recurse
//  *   - If it has a different node_type or attributes, add it
//  *   - For each additional new child, add it
//  *
//  *   - Thus <div><h1><h2 /><h1></div> to <div><h1><h3 /></h1></div>
//  *     should see that the div, is the same and the h1 is the same,
//  *     and see that h2 !== h3, and create a diff operation for that.
//  *
//  * - If they differ in type, replace one with the other
//  */
// impl Diffable for CollapsedNode {
//   fn get_diff_with(&self, other: &Self) -> Diff {
//     let mut diff = get_path_diff(self, other, vec![]);
//     diff.reverse();
//     diff
//   }
// }

// fn get_path_diff(old_node: &Node, new_node: &Node, path: Path) -> Diff {
//   match (old_node, new_node) {
//     (Node::Dom(ref old_token), Node::Dom(ref new_token)) => {
//       // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("dom"));
//       get_html_token_diff(old_token, new_token, path)
//     },
//     (Node::Text(ref old_text), Node::Text(ref new_text)) => get_text_diff(old_text, new_text, path),
//     (Node::Vec(ref old_vec), Node::Vec(ref new_vec)) => get_vec_diff(old_vec, new_vec, path),
//     (Node::Comment(ref old_comment), Node::Comment(ref new_comment)) => {
//       get_comment_diff(old_comment, new_comment, path)
//     },
//     _ => get_replace_diff(new_node, path),
//   }
// }

// fn optionalize_and_extend_with_none<T>(
//   iter: impl Iterator<Item = T>,
// ) -> impl Iterator<Item = Option<T>> {
//   iter.map(|item| Some(item)).chain(repeat_with(|| None))
// }

// fn optionalize_and_zip<T, U>(
//   left_iter: impl ExactSizeIterator<Item = T>,
//   right_iter: impl ExactSizeIterator<Item = U>,
// ) -> impl Iterator<Item = (Option<T>, Option<U>)> {
//   let max_len = max(left_iter.len(), right_iter.len());
//   let left_optionalized = optionalize_and_extend_with_none(left_iter);
//   let right_optionalized = optionalize_and_extend_with_none(right_iter);
//   left_optionalized.zip(right_optionalized).take(max_len)
// }

// fn clone_and_extend(path: &Path, next_item: usize) -> Path {
//   let mut path = path.clone();
//   path.extend(&[next_item]);
//   path
// }

// fn get_html_token_diff(old_token: &HtmlToken, new_token: &HtmlToken, path: Path) -> Diff {
//   /**
//    * If the node_type's are different, we replace
//    * If they're the same, we potentially change attributes
//    * And call get_path_diff on each zipped child
//    */
//   let old_node_type = &old_token.node_type;
//   let new_node_type = &new_token.node_type;
//   if old_node_type != new_node_type {
//     let new_inner_html = new_token.as_inner_html(&path);
//     vec![(
//       path,
//       DiffOperation::Replace(ReplaceOperation { new_inner_html }),
//     )]
//   } else {
//     let iter = optionalize_and_zip(old_token.children.iter(), new_token.children.iter());
//     iter
//       .enumerate()
//       .flat_map(|(i, zipped)| match zipped {
//         (Some(old_child), Some(new_child)) => {
//           // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("some old some new"));
//           get_path_diff(old_child, new_child, clone_and_extend(&path, i))
//         },
//         (Some(old_child), None) => vec![(
//           clone_and_extend(&path, i),
//           DiffOperation::Delete(DeleteOperation {}),
//         )],
//         (None, Some(new_child)) => vec![(
//           clone_and_extend(&path, i),
//           DiffOperation::Insert(InsertOperation {
//             new_inner_html: new_child.as_inner_html(&path),
//           }),
//         )],
//         _ => panic!("We should not encounter two None's in get_html_token_diff"),
//       })
//       .collect::<Vec<DiffItem>>()
//     // TODO attributes
//   }
// }

// fn get_text_diff(old_text: &String, new_text: &String, path: Path) -> Diff {
//   if old_text != new_text {
//     vec![(
//       path,
//       DiffOperation::Replace(ReplaceOperation {
//         new_inner_html: new_text.to_string(),
//       }),
//     )]
//   } else {
//     vec![]
//   }
// }

// fn get_vec_diff(old_vec: &Vec<Node>, new_vec: &Vec<Node>, path: Path) -> Diff {
//   // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str("vec"));

//   // let old_vec_optionalized =
//   let iter = optionalize_and_zip(old_vec.iter(), new_vec.iter());
//   iter
//     .enumerate()
//     .flat_map(|(i, zipped)| match zipped {
//       (Some(old_node), Some(new_node)) => {
//         get_path_diff(old_node, new_node, clone_and_extend(&path, i))
//       },
//       (Some(old_node), None) => vec![(
//         clone_and_extend(&path, i),
//         DiffOperation::Delete(DeleteOperation {}),
//       )],
//       (None, Some(new_node)) => vec![(
//         clone_and_extend(&path, i),
//         DiffOperation::Insert(InsertOperation {
//           new_inner_html: new_node.as_inner_html(&path),
//         }),
//       )],
//       (None, None) => panic!("We should not encounter two None's in get_vec_diff"),
//     })
//     .collect()
// }

// fn get_comment_diff(
//   old_comment_opt: &Option<String>,
//   new_comment_opt: &Option<String>,
//   path: Path,
// ) -> Diff {
//   match (old_comment_opt, new_comment_opt) {
//     (Some(old_comment), Some(new_comment)) => get_text_diff(old_comment, new_comment, path),
//     (Some(_), None) => vec![(
//       path,
//       DiffOperation::Replace(ReplaceOperation {
//         // I think?
//         new_inner_html: "<!-- -->".to_string(),
//       }),
//     )],
//     (None, Some(new_comment)) => vec![(
//       path,
//       DiffOperation::Replace(ReplaceOperation {
//         new_inner_html: format!("<!-- {} -->", new_comment),
//       }),
//     )],
//     (None, None) => vec![],
//   }
// }

// fn get_replace_diff(new_node: &Node, path: Path) -> Diff {
//   let new_inner_html = new_node.as_inner_html(&path);
//   vec![(
//     path,
//     DiffOperation::Replace(ReplaceOperation { new_inner_html }),
//   )]
// }
