use crate::{
  CollapsedHtmlToken,
  CollapsedNode,
};
use custom_derive::custom_derive;
use enum_derive::{
  enum_derive_util,
  EnumFromInner,
};

pub type Attributes = std::collections::HashMap<String, String>;

custom_derive! {
  /// An enum representing the different types of nodes, plus a special
  /// wrapper `Node::Vec`.
  ///
  /// A `Node` represents the result of a call to `.render()` from the
  /// `Component` interface. It does not exactly represent node tree in
  /// the DOM. Rather, `CollapsedNode` is a closer representation of the
  /// DOM.
  #[derive(Debug, Clone, EnumFromInner, Eq, PartialEq)]
  pub enum Node {
    Dom(HtmlToken),
    Text(String),
    Vec(Vec<Node>),
    Comment(Option<String>),
  }
}

fn clone_and_extend(path: &Vec<usize>, next_item: usize) -> Vec<usize> {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

pub trait AsInnerHtml {
  fn as_inner_html(&self, path_so_far: &Vec<usize>) -> String;
}

impl AsInnerHtml for Vec<CollapsedNode> {
  fn as_inner_html(&self, path_so_far: &Vec<usize>) -> String {
    let a = self
      .iter()
      .enumerate()
      .map(|(i, node)| node.as_inner_html(&clone_and_extend(path_so_far, i)))
      .collect();
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
      "as inner html for vec\npath so far {:?} \nself - {:?} \ncollected as inner html- {:?}",
      path_so_far, self, a
    )));
    a
  }
}

impl AsInnerHtml for CollapsedNode {
  fn as_inner_html(&self, path_so_far: &Vec<usize>) -> String {
    let a = match self {
      CollapsedNode::Dom(token) => token.as_inner_html(path_so_far),
      CollapsedNode::Text(s) => s.to_string(),
      CollapsedNode::Comment(str_opt) => match str_opt {
        Some(s) => format!("<!-- {} -->", s),
        None => "<!-- -->".into(),
      },
    };
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
      "as inner html collapsed node \nself {:?}\npath {:?}\na {:?}",
      self, path_so_far, a
    )));
    a
  }
}

/// A struct representing an HTML element.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlToken {
  pub node_type: String,
  pub children: Vec<Node>,
  pub attributes: Attributes,
}

fn format_attributes(attr: &Attributes) -> String {
  // TODO is calling `format!` bad from a package size perspective?
  attr.iter().fold("".to_string(), |accum, (key, val)| {
    if val != "" {
      format!("{} {}=\"{}\"", accum, key, val)
    } else {
      format!("{} {}", accum, key)
    }
  })
}

fn format_path(path: &Path) -> String {
  // the take() function takes a usize, which cannot be negative, thus this might
  // panic if not for this check.
  if path.len() > 0 {
    let path_str = path.iter().fold("".to_string(), |accum, path_segment| {
      format!("{}{},", accum, path_segment)
    });
    path_str.chars().take(path_str.len() - 1).collect()
  } else {
    "".to_string()
  }
}

use lazy_static::lazy_static;
lazy_static! {
  static ref VOID_TAGS: std::collections::HashSet<String> = {
    // see https://www.w3.org/TR/2011/WD-html-markup-20110113/syntax.html#syntax-elements
    // area, base, br, col, command, embed, hr, img, input, keygen, link, meta, param, source, track, wbr
    let mut void_tags = std::collections::HashSet::new();
    void_tags.insert("area".to_string());
    void_tags.insert("base".to_string());
    void_tags.insert("br".to_string());
    void_tags.insert("col".to_string());
    void_tags.insert("command".to_string());
    void_tags.insert("embed".to_string());
    void_tags.insert("hr".to_string());
    void_tags.insert("img".to_string());
    void_tags.insert("input".to_string());
    void_tags.insert("keygen".to_string());
    void_tags.insert("link".to_string());
    void_tags.insert("meta".to_string());
    void_tags.insert("param".to_string());
    void_tags.insert("source".to_string());
    void_tags.insert("track".to_string());
    void_tags.insert("wbr".to_string());
    void_tags
  };
}

// TODO as_inner_html needs to accept a path vec
impl AsInnerHtml for CollapsedHtmlToken {
  fn as_inner_html(&self, path_so_far: &Vec<usize>) -> String {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
      "abc- as inner html for collapsed html token {:#?} \npath so far {:?}",
      self, path_so_far
    )));
    let path_string = format!(" data-smithy-path=\"{}\"", format_path(path_so_far));
    let attributes_string = if self.attributes.len() > 0 {
      format!(" {}", format_attributes(&self.attributes))
    } else {
      "".to_string()
    };

    // I think we need to use the path here???
    if !VOID_TAGS.contains(&self.node_type) {
      let child_html = self
        .children
        .iter()
        .enumerate()
        .map(|(i, node)| node.as_inner_html(&clone_and_extend(path_so_far, i)))
        .collect::<Vec<String>>()
        .join("");
      format!(
        "<{}{}{}>{}</{}>",
        self.node_type, attributes_string, path_string, child_html, self.node_type
      )
    } else {
      format!("<{}{}{} />", self.node_type, attributes_string, path_string)
    }
  }
}

pub type Path = [usize];

/// An enum representing the different phases that a Smithy app can go through.
///
/// A call to `smd!` is a `SmithyComponent`, which is a wrapper around a
/// `Box<dyn FnMut(Phase) -> PhaseResult>`. The content of this
/// function is a match statement over the `Phase` parameter.
pub enum Phase<'a> {
  Rendering,
  PostRendering,
  UiEventHandling((&'a crate::UiEvent, &'a Path)),
  WindowEventHandling(&'a crate::WindowEvent),
  RefAssignment(Vec<usize>),
}

pub type EventHandled = bool;

/// An enum representing the results of a `SmithyComponent` handling a `Phase`.
///
/// A call to `smd!` is a `SmithyComponent`, which is a wrapper around a
/// `Box<dyn FnMut(Phase) -> PhaseResult>`.
///
/// The data contained in the `PhaseResult` will inform the future behavior of
/// the app. For example, when responding to an event, the app will re-render
/// as long as there was at least one handler for that event. That information
/// is contained in the `EventHandled` data.
#[derive(Debug)]
pub enum PhaseResult {
  // TODO make this an Option<Node>
  Rendering(Node),
  PostRendering,
  UiEventHandling(EventHandled),
  WindowEventHandling(EventHandled),
  RefAssignment,
}

impl PhaseResult {
  pub fn unwrap_node(self) -> Node {
    match self {
      PhaseResult::Rendering(node) => node,
      _ => panic!("unwrap_node called on PhaseResult that was not of variant Rendering"),
    }
  }

  pub fn unwrap_event_handled(self) -> EventHandled {
    match self {
      PhaseResult::UiEventHandling(event_handled) => event_handled,
      PhaseResult::WindowEventHandling(event_handled) => event_handled,
      _ => {
        panic!("unwrap_event_handled called on PhaseResult that was not of variant UiEventHandling or WindowEventHandling")
      },
    }
  }
}

/// The results of calling the `smd!` macro is a vector of `SmithyComponent`s.
pub struct SmithyComponent<'a>(pub Box<dyn FnMut(Phase) -> PhaseResult + 'a>);

/// The main trait of Smithy.
pub trait Component {
  fn render(&mut self) -> Node;
  fn handle_post_render(&mut self) {}
  fn handle_ref_assignment(&mut self, _path_so_far: Vec<usize>) {}
  fn handle_ui_event(&mut self, _event: &crate::UiEvent, _path: &Path) -> EventHandled {
    false
  }
  fn handle_window_event(&mut self, _event: &crate::WindowEvent) -> EventHandled {
    false
  }
}

impl<'a> Component for SmithyComponent<'a> {
  fn handle_ui_event(&mut self, event: &crate::UiEvent, path: &Path) -> EventHandled {
    self.0(Phase::UiEventHandling((event, path))).unwrap_event_handled()
  }

  fn handle_window_event(&mut self, event: &crate::WindowEvent) -> EventHandled {
    self.0(Phase::WindowEventHandling(event)).unwrap_event_handled()
  }

  fn render(&mut self) -> Node {
    self.0(Phase::Rendering).unwrap_node()
  }

  fn handle_post_render(&mut self) {
    self.0(Phase::PostRendering);
  }

  fn handle_ref_assignment(&mut self, path_so_far: Vec<usize>) {
    self.0(Phase::RefAssignment(path_so_far));
  }
}
