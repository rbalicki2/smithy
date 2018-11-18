pub type Attributes = std::collections::HashMap<String, String>;

#[derive(Debug)]
pub struct HtmlToken {
  pub node_type: String,
  pub children: Vec<HtmlToken>,
  pub attributes: Attributes,
}

pub type Path = [usize];

/// A Component is invoked in one of two phases: Render and EventHandling.
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
/// ```
/// match phase {
///   Phase::Render => PhaseResult::Render(HtmlToken {
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
/// ```
///
/// Thus, the mutable and immutable references end up in different branches
/// of the match statement, causing them not to conflict.
pub enum Phase<'a> {
  Render,
  EventHandling((crate::Event, &'a Path)),
}

pub type EventHandled = bool;

/// PhaseResult is returned from an EventHandler
///
/// This is the worst part of smithy at the moment, because a Component
/// passed Phase::Render *must* return a PhaseResult::Render, and likewise
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
  Render(HtmlToken),
  EventHandling(EventHandled),
}

impl PhaseResult {
  pub fn unwrap_html_token(self) -> HtmlToken {
    match self {
      PhaseResult::Render(token) => token,
      _ => panic!("unwrap_html_token called on PhaseResult that was not of variant Render"),
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
pub type Component = FnMut(Phase) -> PhaseResult;
