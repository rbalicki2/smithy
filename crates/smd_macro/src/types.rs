pub use proc_macro2::{
  TokenStream,
  TokenTree,
};
use quote::quote;

#[derive(Debug)]
pub struct UIEventHandlingInfo {
  pub reversed_path: Vec<usize>,
  /// None implies we're matching on all events,
  /// Which is used only when is_group is true
  /// TODO: get rid of the is_group param
  pub event: Option<String>,
  /// callback is actually the TokenStream group... it's a really bad name :(
  pub callback: TokenStream,
  pub is_group: bool,
}

impl UIEventHandlingInfo {
  pub fn from_string_token_stream_pair((event, callback): StringTokenStreamPair) -> Self {
    UIEventHandlingInfo {
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

#[derive(Debug)]
pub struct DomRefInfo {
  pub reversed_path: Vec<usize>,
  pub dom_ref: TokenStream,
}

impl DomRefInfo {
  pub fn from_token_stream(t: TokenStream) -> DomRefInfo {
    DomRefInfo {
      dom_ref: t,
      reversed_path: vec![],
    }
  }
}

pub type TokenTreeSlice<'a> = &'a [TokenTree];

// TODO rename, perhaps to TokenStreamEventHandlingInfoDomRefOptTrio
// ... or something
pub type TokenStreamEventHandlingInfoPair =
  (TokenStream, Vec<UIEventHandlingInfo>, Vec<DomRefInfo>);

pub type StringTokenStreamPair = (String, TokenStream);
pub enum AttributeOrEventHandler {
  Attribute(StringTokenStreamPair),
  EventHandler(StringTokenStreamPair),
  DomRef(TokenStream),
}

pub struct SplitAttributeOrEventHandlers(
  pub Vec<StringTokenStreamPair>,
  pub Vec<StringTokenStreamPair>,
  pub Option<TokenStream>,
);
impl Into<SplitAttributeOrEventHandlers> for Vec<AttributeOrEventHandler> {
  fn into(self) -> SplitAttributeOrEventHandlers {
    let len = self.len();
    let attributes = Vec::with_capacity(len);
    let event_handlers = Vec::with_capacity(len);
    self.into_iter().fold(
      SplitAttributeOrEventHandlers(attributes, event_handlers, None),
      |SplitAttributeOrEventHandlers(mut attributes, mut event_handlers, mut dom_ref), next_val| {
        match next_val {
          AttributeOrEventHandler::Attribute(attr) => attributes.push(attr),
          AttributeOrEventHandler::EventHandler(event_handler) => {
            event_handlers.push(event_handler)
          },
          AttributeOrEventHandler::DomRef(stream) => dom_ref = Some(stream),
        };
        SplitAttributeOrEventHandlers(attributes, event_handlers, dom_ref)
      },
    )
  }
}

pub struct SplitTokenStreamEventHandlingInfoPairs(
  pub Vec<TokenStream>,
  pub Vec<UIEventHandlingInfo>,
  pub Vec<DomRefInfo>,
);
impl Into<SplitTokenStreamEventHandlingInfoPairs> for Vec<TokenStreamEventHandlingInfoPair> {
  fn into(self) -> SplitTokenStreamEventHandlingInfoPairs {
    let child_token_streams = Vec::with_capacity(self.len());
    let child_event_handling_infos = vec![];
    let child_dom_ref_token_streams = vec![];
    self.into_iter().enumerate().fold(
      SplitTokenStreamEventHandlingInfoPairs(
        child_token_streams,
        child_event_handling_infos,
        child_dom_ref_token_streams,
      ),
      |SplitTokenStreamEventHandlingInfoPairs(
        mut child_token_streams,
        mut child_event_handling_infos,
        mut child_dom_ref_token_streams,
      ),
       (i, item)| {
        child_token_streams.push(item.0);
        for mut current_event_handling_info in item.1.into_iter() {
          current_event_handling_info.reversed_path.push(i);
          child_event_handling_infos.push(current_event_handling_info);
        }
        for mut current_dom_ref_info in item.2.into_iter() {
          current_dom_ref_info.reversed_path.push(i);
          child_dom_ref_token_streams.push(current_dom_ref_info);
        }
        SplitTokenStreamEventHandlingInfoPairs(
          child_token_streams,
          child_event_handling_infos,
          child_dom_ref_token_streams,
        )
      },
    )
  }
}

#[derive(Debug)]
pub struct WindowEventHandlingInfo {
  pub event: String,
  pub callback: TokenStream,
}

#[derive(Debug)]
pub struct LifecycleEventHandlingInfo {
  pub lifecycle_event: String,
  pub callback: TokenStream,
}

#[derive(Debug)]
pub enum GlobalEventHandlingInfo {
  Window(WindowEventHandlingInfo),
  Lifecycle(LifecycleEventHandlingInfo),
}
