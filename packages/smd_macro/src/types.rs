pub use proc_macro2::{
  TokenStream,
  TokenTree,
};

pub struct EventHandlingInfo {
  pub path: Box<smithy_types::Path>,
  pub event: String,
  pub callback: TokenStream,
}

pub type TokenTreeSlice<'a> = &'a [TokenTree];

pub type TokenStreamEventHandlingInfoPair = (TokenStream, Vec<EventHandlingInfo>);

pub type StringTokenStreamPair = (String, TokenStream);

pub enum AttributeOrEventHandler {
  Attribute(StringTokenStreamPair),
  EventHandler(StringTokenStreamPair),
}

pub trait SplitByType {
  fn split_by_type(self) -> (Vec<StringTokenStreamPair>, Vec<StringTokenStreamPair>);
}

impl SplitByType for Vec<AttributeOrEventHandler> {
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
