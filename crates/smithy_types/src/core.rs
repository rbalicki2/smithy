use crate::{
  CollapsedHtmlToken,
  CollapsedNode,
};
use custom_derive::custom_derive;
use enum_derive::{
  enum_derive_util,
  EnumFromInner,
};
use web_sys::Node as WebSysNode;

pub type Attributes = std::collections::HashMap<String, String>;

custom_derive! {
  #[derive(Debug, Clone, EnumFromInner, Eq, PartialEq)]
  pub enum Node {
    Dom(HtmlToken),
    Text(String),
    Vec(Vec<Node>),
    Comment(Option<String>),
  }
}

pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String;
}

fn concat(path: &Path, new_item: usize) -> Vec<usize> {
  let new_path = path.clone();
  [new_path, &[new_item]].concat()
}

fn clone_and_extend(path: &Vec<usize>, next_item: usize) -> Vec<usize> {
  let mut path = path.clone();
  path.extend(&[next_item]);
  path
}

impl AsInnerHtml for Vec<CollapsedNode> {
  fn as_inner_html(&self) -> String {
    self
      .iter()
      .enumerate()
      .map(|(i, node)| node.as_inner_html())
      .collect()
  }
}

impl AsInnerHtml for CollapsedNode {
  fn as_inner_html(&self) -> String {
    match self {
      CollapsedNode::Dom(token) => token.as_inner_html(),
      CollapsedNode::Text(s) => s.to_string(),
      CollapsedNode::Comment(str_opt) => match str_opt {
        Some(s) => format!("<!-- {} -->", s),
        None => "<!-- -->".into(),
      },
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlToken {
  pub node_type: String,
  pub children: Vec<Node>,
  pub attributes: Attributes,
}

fn format_attributes(attr: &Attributes) -> String {
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

impl AsInnerHtml for CollapsedHtmlToken {
  fn as_inner_html(&self) -> String {
    let path_string = format!(" data-smithy-path=\"{}\"", format_path(&self.path));
    let attributes_string = if self.attributes.len() > 0 {
      format!(" {}", format_attributes(&self.attributes))
    } else {
      "".to_string()
    };

    if !VOID_TAGS.contains(&self.node_type) {
      let child_html = self
        .children
        .iter()
        .enumerate()
        .map(|(i, node)| node.as_inner_html())
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

/// A Component is invoked in one of two phases: Rendering and UiEventHandling.
///
/// Internally, this is represented as a match statement, allowing us to handle
/// situations like:
///
/// //TODO put this into triple backticks
/// smd!(<div on_click={|_| app_state.count = app_state.count + 1}>{ app_state.count }</div>);
///
/// In the above, there are multiple references to app_state.count, one of which is a
/// mutable reference. This works because after the macro expands, it becomes
///
/// match phase {
///   Phase::Rendering => PhaseResult::Rendering(HtmlToken {
///     node_type: "div".into(),
///     children: vec![app_state.count.into()], // immutable reference
///     attributes: HashMap::new(),
///   }),
///   Phase::UiEventHandling((event, path)) => {
///     match (&event, &path) => {
///       (|_| app_state.count = app_state.count + 1)(); // mutable reference
///       PhaseResult::UiEventHandling(true)
///     },
///     _ => PhaseResult::UiEventHandling(false),
///   }
/// }
///
/// Thus, the mutable and immutable references end up in different branches
/// of the match statement, causing them not to conflict.
pub enum Phase<'a> {
  Rendering,
  PostRendering,
  UiEventHandling((&'a crate::UiEvent, &'a Path)),
  WindowEventHandling(&'a crate::WindowEvent),
  RefAssignment(Vec<usize>),
}

pub type EventHandled = bool;

/// PhaseResult is returned from an EventHandler
///
/// This is the worst part of smithy at the moment, because a Component
/// passed Phase::Rendering *must* return a PhaseResult::Rendering, and likewise
/// a Component passed a Phase::UiEventHandling *must* return a
/// PhaseResult::UiEventHandling.
///
/// This *should* be done through the type system, but currently, that is not
/// possible.
///
/// This is OK, though, because EventHandlers are created with the smd! macro
/// and conform to this restriction.
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

/// The results of calling the smd! macro is a vector of SmithyComponents.
///
/// I would not recommend writing these yourself, although you absolutely
/// can, if you want.
pub struct SmithyComponent<'a>(pub Box<FnMut(Phase) -> PhaseResult + 'a>);

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
