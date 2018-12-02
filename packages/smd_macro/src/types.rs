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

pub enum AttributeOrEventHandler {
  Attribute((String, TokenStream)),
  EventHandler((String, TokenStream)),
}
