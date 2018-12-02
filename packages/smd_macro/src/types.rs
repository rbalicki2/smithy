pub use proc_macro2::{
  TokenStream,
  TokenTree,
};
use std::iter::Extend;

pub struct EventHandlingInfo {
  pub reversed_path: Vec<usize>,
  /// None implies we're matching on all events
  pub event: Option<String>,
  pub callback: TokenStream,
}

impl EventHandlingInfo {
  pub fn from_string_token_stream_pair((event, callback): StringTokenStreamPair) -> Self {
    EventHandlingInfo {
      reversed_path: vec![],
      event: Some(event),
      callback,
    }
  }

  // TODO implement this once Path is a vec...
  // pub fn prepend_to_path(&mut self, u: usize) {
  //   self.path = {
  //     let new_path = vec![u];
  //     new_path.extend(self.path.iter());
  //     // new_path.prepend(u);
  //     // new_path.into()
  //     Box::new(*&new_path[..])
  //   };
  // }
}

pub type TokenTreeSlice<'a> = &'a [TokenTree];

pub type TokenStreamEventHandlingInfoPair = (TokenStream, Vec<EventHandlingInfo>);

pub type StringTokenStreamPair = (String, TokenStream);

pub enum AttributeOrEventHandler {
  Attribute(StringTokenStreamPair),
  EventHandler(StringTokenStreamPair),
}

// TODO convert into Into/From
pub trait SplitByType<T1, T2> {
  fn split_by_type(self) -> (T1, T2);
}

impl SplitByType<Vec<StringTokenStreamPair>, Vec<StringTokenStreamPair>>
  for Vec<AttributeOrEventHandler>
{
  fn split_by_type(self) -> (Vec<StringTokenStreamPair>, Vec<StringTokenStreamPair>) {
    let len = self.len();
    let attributes = Vec::with_capacity(len);
    let event_handlers = Vec::with_capacity(len);
    self.into_iter().fold(
      (attributes, event_handlers),
      |(mut attributes, mut event_handlers), next_val| {
        match next_val {
          AttributeOrEventHandler::Attribute(attr) => attributes.push(attr),
          AttributeOrEventHandler::EventHandler(event_handler) => {
            event_handlers.push(event_handler)
          },
        };
        (attributes, event_handlers)
      },
    )
  }
}

impl SplitByType<Vec<TokenStream>, Vec<EventHandlingInfo>>
  for Vec<TokenStreamEventHandlingInfoPair>
{
  fn split_by_type(self) -> (Vec<TokenStream>, Vec<EventHandlingInfo>) {
    let child_token_streams = Vec::with_capacity(self.len());
    let child_event_handling_infos = vec![];
    self.into_iter().fold(
      (child_token_streams, child_event_handling_infos),
      |(mut child_token_streams, mut child_event_handling_infos), item| {
        child_token_streams.push(item.0);
        for mut current_event_handling_info in item.1.into_iter() {
          // TODO maybe append or prepend to current_event_handling_info.path
          child_event_handling_infos.push(current_event_handling_info);
        }
        (child_token_streams, child_event_handling_infos)
      },
    )
  }
}
