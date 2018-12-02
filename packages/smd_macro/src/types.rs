pub use proc_macro2::{
  TokenStream,
  TokenTree,
};

pub struct EventHandlingInfo {
  pub path: Box<smithy_types::Path>,
  pub event: String,
  pub callback: TokenStream,
}

impl EventHandlingInfo {
  pub fn from_string_token_stream_pair((event, callback): StringTokenStreamPair) -> Self {
    EventHandlingInfo {
      path: Box::new([]),
      event,
      callback,
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
          }
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
          current_event_handling_info.path = Box::new([1, 2, 3]);
          child_event_handling_infos.push(current_event_handling_info);
        }
        (child_token_streams, child_event_handling_infos)
      },
    )
  }
}
