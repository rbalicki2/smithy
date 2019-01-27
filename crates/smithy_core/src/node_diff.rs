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
pub struct DeleteChildOperation {
  child_index: usize,
}

#[derive(Debug)]
pub struct UpdateAttributesOperation {
  pub new_attributes: Attributes,
}

#[derive(Debug)]
pub enum DiffOperation {
  Replace(ReplaceOperation),
  Insert(InsertOperation),
  DeleteChild(DeleteChildOperation),
  UpdateAttributes(UpdateAttributesOperation),
}

pub type DiffItem = (Path, DiffOperation);
pub type Diff = Vec<DiffItem>;

pub trait Diffable {
  fn get_diff_with(&self, other: &Self) -> Diff;
}

pub trait ApplicableTo<E> {
  fn apply_to(&self, other: E);
}

impl ApplicableTo<&web_sys::Element> for DiffItem {
  fn apply_to(&self, el: &web_sys::Element) {
    let target_el = {
      let path_to_parent = &self.0;
      let path_selector = format!(
        "[data-smithy-path=\"{}\"]",
        path_to_parent
          .iter()
          .map(|u| u.to_string())
          .collect::<Vec<String>>()
          .join(",")
      );
      // this should never fail, the path_to_parent should always point to an
      // existing node...
      // TODO don't unwrap
      let target_el = el.query_selector(&path_selector).unwrap().unwrap();
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&path_selector));
      target_el
    };
    match &self.1 {
      DiffOperation::Replace(replace_operation) => {
        target_el.set_inner_html(&replace_operation.new_inner_html)
      },
      DiffOperation::Insert(insert_operation) => {},
      DiffOperation::DeleteChild(delete_child_operation) => {},
      DiffOperation::UpdateAttributes(update_attributes_operation) => {
        for (attr, attr_value) in &update_attributes_operation.new_attributes {
          let _ = target_el.set_attribute(&attr, &attr_value);
        }
      },
    };
  }
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
    get_vec_path_diff(self, other, vec![])
  }
}

fn get_vec_path_diff(
  old_nodes: &Vec<CollapsedNode>,
  new_nodes: &Vec<CollapsedNode>,
  path: Path,
) -> Diff {
  let zipped = optionalize_and_zip(old_nodes.iter(), new_nodes.iter());
  zipped
    .enumerate()
    .flat_map(|(i, (current, new))| match (current, new) {
      (Some(old_node), Some(new_node)) => {
        get_diff_between_tokens(old_node, new_node, clone_and_extend(&path, i))
      },
      (Some(old_node), None) => vec![(
        // old_node.path.clone(),
        clone_and_extend(&path, i),
        DiffOperation::DeleteChild(DeleteChildOperation { child_index: 123 }),
      )],
      (None, Some(new_node)) => vec![(
        clone_and_extend(&path, i),
        // new_node.path.clone(),
        DiffOperation::Insert(InsertOperation {
          new_inner_html: new_node.as_inner_html(&path),
        }),
      )],
      (None, None) => panic!("Should not happen - we should not encounter two none's here"),
    })
    .collect()
}

fn get_diff_between_tokens(old_node: &CollapsedNode, new_node: &CollapsedNode, path: Path) -> Diff {
  // let path = old_node.path.clone();
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
  web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    "\n\n\nhtml token diff {}\nold- {:?}\nold attr {:?}\nnew- {:?}\nnew attr {:?} \npath {:?}",
    old_token.attributes != new_token.attributes,
    old_token,
    old_token.attributes,
    new_token,
    new_token.attributes,
    path
  )));
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
    let mut diff = iter
      .enumerate()
      .flat_map(|(i, zipped)| match zipped {
        (Some(old_child), Some(new_child)) => {
          get_diff_between_tokens(old_child, new_child, clone_and_extend(&path, i))
        },
        (Some(old_child), None) => vec![(
          clone_and_extend(&path, i),
          DiffOperation::DeleteChild(DeleteChildOperation { child_index: 123 }),
        )],
        (None, Some(new_child)) => vec![(
          clone_and_extend(&path, i),
          DiffOperation::Insert(InsertOperation {
            new_inner_html: new_child.as_inner_html(&path),
          }),
        )],
        _ => panic!("We should not encounter two None's in get_html_token_diff"),
      })
      .collect::<Vec<DiffItem>>();

    if old_token.attributes != new_token.attributes {
      diff.push((
        old_token.path.clone(),
        DiffOperation::UpdateAttributes(UpdateAttributesOperation {
          new_attributes: new_token.attributes.clone(),
        }),
      ))
    };

    diff
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
    (Some(_old_comment), None) => vec![(
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
