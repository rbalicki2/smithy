use custom_derive::custom_derive;
use enum_derive::{
  enum_derive_util,
  EnumFromInner,
};

pub type Attributes = std::collections::HashMap<String, String>;

custom_derive! {
  #[derive(Debug, Clone, EnumFromInner, Eq, PartialEq)]
  pub enum Node {
    Dom(HtmlToken),
    Text(String),
    Vec(Vec<Node>),
  }
}

pub trait AsInnerHtml {
  fn as_inner_html(&self) -> String;
}

impl AsInnerHtml for Node {
  fn as_inner_html(&self) -> String {
    match self {
      Node::Dom(token) => token.as_inner_html(),
      Node::Text(s) => s.to_string(),
      Node::Vec(vec) => vec
        .iter()
        .map(Node::as_inner_html)
        .collect::<Vec<String>>()
        .join(""),
    }
  }
}

// These power <div>{ t: T }</div> where T: Vec<SmithyComponent> etc

impl From<&mut SmithyComponent> for Node {
  fn from(v: &mut SmithyComponent) -> Node {
    v.render()
  }
}

impl From<&str> for Node {
  fn from(s: &str) -> Self {
    Node::Text(s.to_string())
  }
}

impl From<&String> for Node {
  fn from(s: &String) -> Self {
    Node::Text(s.to_string())
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HtmlToken {
  pub node_type: String,
  pub children: Vec<Node>,
  pub attributes: Attributes,
}

impl AsInnerHtml for HtmlToken {
  fn as_inner_html(&self) -> String {
    let attributes_string = if self.attributes.len() > 0 {
      format!(" {}", self.attributes.as_inner_html())
    } else {
      "".to_string()
    };
    if self.children.len() > 0 {
      let child_html = self
        .children
        .iter()
        .map(Node::as_inner_html)
        .collect::<Vec<String>>()
        .join("");
      format!(
        "<{}{}>{}</{}>",
        self.node_type, attributes_string, child_html, self.node_type
      )
    } else {
      format!("<{}{} />", self.node_type, attributes_string)
    }
  }
}

impl AsInnerHtml for Attributes {
  fn as_inner_html(&self) -> String {
    self
      .iter()
      .map(|(key, val)| {
        if val != "" {
          format!("{}={}", key, val)
        } else {
          key.to_string()
        }
      })
      .collect::<Vec<String>>()
      .join(" ")
  }
}

pub type Path = [usize];

/// A Component is invoked in one of two phases: Rendering and EventHandling.
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
///   Phase::EventHandling((event, path)) => {
///     match (&event, &path) => {
///       (|_| app_state.count = app_state.count + 1)(); // mutable reference
///       PhaseResult::EventHandling(true)
///     },
///     _ => PhaseResult::EventHandling(false),
///   }
/// }
///
/// Thus, the mutable and immutable references end up in different branches
/// of the match statement, causing them not to conflict.
pub enum Phase<'a> {
  Rendering,
  EventHandling((&'a crate::Event, &'a Path)),
}

pub type EventHandled = bool;

/// PhaseResult is returned from an EventHandler
///
/// This is the worst part of smithy at the moment, because a Component
/// passed Phase::Rendering *must* return a PhaseResult::Rendering, and likewise
/// a Component passed a Phase::EventHandling *must* return a
/// PhaseResult::EventHandling.
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
  EventHandling(EventHandled),
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
      PhaseResult::EventHandling(event_handled) => event_handled,
      _ => {
        panic!("unwrap_event_handled called on PhaseResult that was not of variant EventHandling")
      },
    }
  }
}

/// The results of calling the smd! macro is a vector of SmithyComponents.
///
/// I would not recommend writing these yourself, although you absolutely
/// can, if you want.
pub struct SmithyComponent(pub Box<FnMut(Phase) -> PhaseResult>);

pub trait Component {
  fn handle_event(&mut self, _event: &crate::Event, _path: &Path) -> EventHandled {
    false
  }
  fn render(&mut self) -> Node;
}

impl Component for SmithyComponent {
  fn handle_event(&mut self, event: &crate::Event, path: &Path) -> EventHandled {
    self.0(Phase::EventHandling((event, path))).unwrap_event_handled()
  }

  fn render(&mut self) -> Node {
    self.0(Phase::Rendering).unwrap_node()
  }
}
