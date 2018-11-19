use custom_derive::custom_derive;
use enum_derive::{
  enum_derive_util,
  EnumFromInner,
};

pub type Attributes = std::collections::HashMap<String, String>;

custom_derive! {
  #[derive(Debug, Clone, EnumFromInner)]
  pub enum Node {
    Dom(HtmlToken),
    Text(String),
    Vec(Vec<Node>),
  }
}

impl From<&mut Vec<Component>> for Node {
  fn from(v: &mut Vec<Component>) -> Node {
    Node::Vec(v.iter_mut().map(Component::render).collect())
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

#[derive(Debug, Clone)]
pub struct HtmlToken {
  pub node_type: String,
  pub children: Vec<Node>,
  pub attributes: Attributes,
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
  EventHandling((crate::Event, &'a Path)),
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
      }
    }
  }
}

/// results of smd! macro
///
/// I would not recommend writing these yourself, although you absolutely
/// can, if you want.
pub struct Component(pub Box<FnMut(Phase) -> PhaseResult>);

impl Component {
  pub fn render(&mut self) -> Node {
    self.0(Phase::Rendering).unwrap_node()
  }

  pub fn handle_event(&mut self, event: crate::Event, path: &Path) -> EventHandled {
    self.0(Phase::EventHandling((event, path))).unwrap_event_handled()
  }
}
