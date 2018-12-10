pub use proc_macro2::{
  TokenStream,
  TokenTree,
};
use quote::quote;

#[derive(Debug)]
pub struct EventHandlingInfo {
  pub reversed_path: Vec<usize>,
  /// None implies we're matching on all events
  pub event: Option<String>,
  pub callback: TokenStream,
  pub is_group: bool,
}

impl EventHandlingInfo {
  pub fn from_string_token_stream_pair((event, callback): StringTokenStreamPair) -> Self {
    EventHandlingInfo {
      reversed_path: vec![],
      event: Some(event),
      callback,
      is_group: false,
    }
  }

  /// N.B. this also reverses the path
  pub fn get_path_match(&self) -> TokenStream {
    let inner = self
      .reversed_path
      .iter()
      .rev()
      .fold(quote!{}, |accum, path_item| {
        quote!{ #accum #path_item, }
      });
    let additional_dot_dot = if self.is_group {
      quote!{ rest.. }
    } else {
      quote!{}
    };
    quote!{
      [ #inner #additional_dot_dot ]
    }
  }
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
    self.into_iter().enumerate().fold(
      (child_token_streams, child_event_handling_infos),
      |(mut child_token_streams, mut child_event_handling_infos), (i, item)| {
        child_token_streams.push(item.0);
        for mut current_event_handling_info in item.1.into_iter() {
          current_event_handling_info.reversed_path.push(i);
          child_event_handling_infos.push(current_event_handling_info);
        }
        (child_token_streams, child_event_handling_infos)
      },
    )
  }
}
