use smithy_types::{
  AsInnerHtml,
  Attributes,
  CollapsedHtmlToken,
  CollapsedNode,
};

type NewInnerHtml = String;

pub type Path = Vec<usize>;

#[derive(Debug)]
pub struct ReplaceChildOperation {
  pub new_inner_html: NewInnerHtml,
  pub child_index: usize,
}

#[derive(Debug)]
pub struct InsertChildOperation {
  pub new_inner_html: NewInnerHtml,
  pub child_index: usize,
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
  ReplaceChild(ReplaceChildOperation),
  InsertChild(InsertChildOperation),
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

fn node_from_str(s: &str) -> web_sys::Node {
  let doc = web_sys::window().unwrap().document().unwrap();
  let new_container_el = doc.create_element("div").unwrap();
  new_container_el.set_inner_html(s);
  new_container_el.first_child().unwrap()
}

fn apply_diff_item_to_element_ref(diff_op: &DiffOperation, target_el: &web_sys::Element) {
  web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
    "apply diff {:?}",
    diff_op
  )));
  match &diff_op {
    DiffOperation::ReplaceChild(replace_child_operation) => {
      let child_opt = target_el
        .child_nodes()
        .get(replace_child_operation.child_index as u32);
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "replace op {:?} {:?}\nexisting inner {:?}",
        child_opt.is_some(),
        target_el.child_nodes().length(),
        target_el.inner_html()
      )));

      match child_opt {
        Some(child) => {
          let new_node = node_from_str(&replace_child_operation.new_inner_html);
          let _ = target_el.replace_child(&new_node, &child);
        },
        _ => panic!("no child found"),
      }
    },
    DiffOperation::InsertChild(insert_child_operation) => {
      let new_inner_dom = node_from_str(&insert_child_operation.new_inner_html);

      let child_opt = target_el
        .child_nodes()
        .get(insert_child_operation.child_index as u32);

      // TODO figure out how to get child_opt.map(|x| &x) to compile
      let _ = match child_opt {
        Some(child) => target_el.insert_before(&new_inner_dom, Some(&child)),
        None => target_el.insert_before(&new_inner_dom, None),
      };
    },
    DiffOperation::DeleteChild(delete_child_operation) => {
      web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
        "delete {:?} {:?}",
        target_el
          .child_nodes()
          .get(delete_child_operation.child_index as u32)
          .is_some(),
        target_el.child_nodes().length()
      )));
      let child = target_el
        .child_nodes()
        .get(delete_child_operation.child_index as u32)
        .unwrap();

      let _ = target_el.remove_child(&child);
    },
    DiffOperation::UpdateAttributes(update_attributes_operation) => {
      for (attr, attr_value) in &update_attributes_operation.new_attributes {
        let _ = target_el.set_attribute(&attr, &attr_value);
      }
    },
  };
}

impl ApplicableTo<&web_sys::Element> for DiffItem {
  fn apply_to(&self, el: &web_sys::Element) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
      "apply to {:?}",
      self
    )));

    if self.0.len() == 0 {
      apply_diff_item_to_element_ref(&self.1, el);
    } else {
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

        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
          "inner {:?}\n\nis_some {:?}\nselector {:?}",
          el.inner_html(),
          el.query_selector(&path_selector).unwrap().is_some(),
          path_selector
        )));

        let target_el = el.query_selector(&path_selector).unwrap().unwrap();
        target_el
      };
      apply_diff_item_to_element_ref(&self.1, &target_el);
    }
  }
}

/**
 * New diffing algo
 *
 * - Wrap the outermost Vec<CollapsedNode>
 *   in another CollapsedNode, representing <div id="app" />
 *
 * Diffing Algo:
 *
 * - Starting with the <div id="app" />, keep track of its path (aka [])
 * - For each zipped optionalized child, match:
 *   - (Some(original), Some(new)) =>
 *     - If node_type is the same
 *       - Change attributes
 *       - Recurse
 *     - Else
 *       - ReplaceChildChild
 *   - (Some(original), None) =>
 *     - RemoveChild
 *   - (None, Some(new)) =>
 *     - DeleteChild
 */

impl Diffable for Vec<CollapsedNode> {
  fn get_diff_with(&self, other: &Self) -> Diff {
    get_vec_path_diff(self, other)
  }
}

fn get_i(i: usize, max_len: usize, potentially_deleting: bool) -> usize {
  if potentially_deleting {
    max_len - i - 1
  } else {
    i
  }
}

fn get_vec_path_diff(old_nodes: &Vec<CollapsedNode>, new_nodes: &Vec<CollapsedNode>) -> Diff {
  let potentially_deleting = old_nodes.len() > new_nodes.len();
  let max_len = std::cmp::max(old_nodes.len(), new_nodes.len());
  // N.B. this is *really redundant* and should be refactored away.
  let path = vec![];

  let zipped: Box<Iterator<Item = (Option<&CollapsedNode>, Option<&CollapsedNode>)>> =
    if potentially_deleting {
      let zipped = crate::zip_util::optionalize_and_zip(old_nodes.iter(), new_nodes.iter());
      let mut vec = zipped.collect::<Vec<(Option<&CollapsedNode>, Option<&CollapsedNode>)>>();
      vec.reverse();
      Box::new(vec.into_iter())
    } else {
      Box::new(crate::zip_util::optionalize_and_zip(
        old_nodes.iter(),
        new_nodes.iter(),
      ))
    };

  zipped
    .enumerate()
    .flat_map(|(i, (current, new))| {
      let real_i = get_i(i, max_len, potentially_deleting);
      match (current, new) {
        (Some(old_node), Some(new_node)) => {
          get_diff_between_tokens(old_node, new_node, &path, real_i)
        },
        (Some(_old_node), None) => vec![(
          path.clone(),
          DiffOperation::DeleteChild(DeleteChildOperation {
            child_index: real_i,
          }),
        )],
        (None, Some(new_node)) => vec![(
          path.clone(),
          DiffOperation::InsertChild(InsertChildOperation {
            new_inner_html: new_node.as_inner_html(&path),
            child_index: real_i,
          }),
        )],
        (None, None) => panic!("Should not happen - we should not encounter two none's here"),
      }
    })
    .collect()
}

fn get_diff_between_tokens(
  old_node: &CollapsedNode,
  new_node: &CollapsedNode,
  path_to_parent: &Path,
  child_index: usize,
) -> Diff {
  match (old_node, new_node) {
    (CollapsedNode::Dom(ref old_token), CollapsedNode::Dom(ref new_token)) => {
      get_html_token_diff(old_token, new_token, path_to_parent, child_index)
    },
    (CollapsedNode::Text(ref old_text), CollapsedNode::Text(ref new_text)) => {
      get_text_diff(old_text, new_text, path_to_parent.to_vec(), child_index)
    },
    (CollapsedNode::Comment(ref old_comment), CollapsedNode::Comment(ref new_comment)) => {
      get_comment_diff(
        old_comment,
        new_comment,
        path_to_parent.to_vec(),
        child_index,
      )
    },
    _ => get_replace_diff(new_node, path_to_parent, child_index),
  }
}

fn clone_and_extend(path: &Path, next_item: usize) -> Path {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

fn get_html_token_diff(
  old_token: &CollapsedHtmlToken,
  new_token: &CollapsedHtmlToken,
  path_to_parent: &Path,
  child_index: usize,
) -> Diff {
  // If the node_type's are different, we replace
  // If they're the same, we potentially change attributes
  // And call get_path_diff on each zipped child
  let old_node_type = &old_token.node_type;
  let new_node_type = &new_token.node_type;
  if old_node_type != new_node_type {
    let new_inner_html = new_token.as_inner_html(path_to_parent);

    vec![(
      path_to_parent.to_vec(),
      DiffOperation::ReplaceChild(ReplaceChildOperation {
        new_inner_html,
        child_index,
      }),
    )]
  } else {
    // node types are the same, so we iterate over children
    let potentially_deleting = old_token.children.len() > new_token.children.len();
    let max_len = std::cmp::max(old_token.children.len(), new_token.children.len());

    let zipped: Box<Iterator<Item = (Option<&CollapsedNode>, Option<&CollapsedNode>)>> =
      if potentially_deleting {
        let zipped = crate::zip_util::optionalize_and_zip(
          old_token.children.iter(),
          new_token.children.iter(),
        );
        let mut vec = zipped.collect::<Vec<(Option<&CollapsedNode>, Option<&CollapsedNode>)>>();
        vec.reverse();
        Box::new(vec.into_iter())
      } else {
        Box::new(crate::zip_util::optionalize_and_zip(
          old_token.children.iter(),
          new_token.children.iter(),
        ))
      };

    let mut diff = zipped
      .enumerate()
      .flat_map(|(i, zipped)| match zipped {
        (Some(old_child), Some(new_child)) => get_diff_between_tokens(
          old_child,
          new_child,
          &old_token.path,
          get_i(i, max_len, potentially_deleting),
        ),
        (Some(_old_child), None) => vec![(
          old_token.path.clone(),
          DiffOperation::DeleteChild(DeleteChildOperation {
            child_index: get_i(i, max_len, potentially_deleting),
          }),
        )],
        (None, Some(new_child)) => vec![(
          old_token.path.clone(),
          DiffOperation::InsertChild(InsertChildOperation {
            new_inner_html: new_child.as_inner_html(path_to_parent),
            child_index: get_i(i, max_len, potentially_deleting),
          }),
        )],
        _ => panic!("We should not encounter two None's in get_html_token_diff"),
      })
      .collect::<Vec<DiffItem>>();

    if old_token.attributes != new_token.attributes || old_token.path != new_token.path {
      diff.push((
        old_token.path.clone(),
        DiffOperation::UpdateAttributes(UpdateAttributesOperation {
          new_attributes: new_token.get_attributes_including_path(),
        }),
      ))
    };

    diff
  }
}

fn get_text_diff(old_text: &String, new_text: &String, path: Path, child_index: usize) -> Diff {
  if old_text != new_text {
    vec![(
      path,
      DiffOperation::ReplaceChild(ReplaceChildOperation {
        new_inner_html: new_text.to_string(),
        child_index,
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
  child_index: usize,
) -> Diff {
  match (old_comment_opt, new_comment_opt) {
    (Some(old_comment), Some(new_comment)) => {
      get_text_diff(old_comment, new_comment, path, child_index)
    },
    (Some(_old_comment), None) => vec![(
      path,
      DiffOperation::ReplaceChild(ReplaceChildOperation {
        // I think?
        new_inner_html: "<!-- -->".to_string(),
        child_index,
      }),
    )],
    (None, Some(new_comment)) => vec![(
      path,
      DiffOperation::ReplaceChild(ReplaceChildOperation {
        new_inner_html: format!("<!-- {} -->", new_comment),
        child_index,
      }),
    )],
    (None, None) => vec![],
  }
}

fn get_replace_diff(new_node: &CollapsedNode, path_to_parent: &Path, child_index: usize) -> Diff {
  let new_inner_html = new_node.as_inner_html(&clone_and_extend(path_to_parent, child_index));
  vec![(
    path_to_parent.to_vec(),
    DiffOperation::ReplaceChild(ReplaceChildOperation {
      new_inner_html,
      child_index,
    }),
  )]
}
