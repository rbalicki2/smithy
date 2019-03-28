pub use proc_macro2::{
  TokenStream,
  TokenTree,
};
use quote::quote;

#[derive(Debug)]
pub struct UIEventHandlingInfo {
  pub reversed_path: Vec<usize>,
  /// if event is none, this implies that this is a UIEventHandlingInfo
  /// for a group (e.g. { &mut child_el }).
  pub event: Option<String>,
  /// callback is actually the TokenStream group... it's a really bad name :(
  pub callback: TokenStream,
}

impl UIEventHandlingInfo {
  pub fn from_string_token_stream_pair((event, callback): StringTokenStreamPair) -> Self {
    UIEventHandlingInfo {
      reversed_path: vec![],
      event: Some(event),
      callback,
    }
  }

  /// N.B. this also reverses the path
  pub fn get_path_match(&self) -> TokenStream {
    let inner = self
      .reversed_path
      .iter()
      .rev()
      .fold(quote! {}, |accum, path_item| {
        quote! { #accum #path_item, }
      });
    let additional_dot_dot = if !self.event.is_some() {
      quote! { rest.. }
    } else {
      quote! {}
    };
    quote! {
      [ #inner #additional_dot_dot ]
    }
  }
}

#[derive(Debug)]
pub struct DomRefInfo {
  // TODO is this path actually reversed...?
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
  pub Vec<DomRefInfo>,
);
impl Into<SplitAttributeOrEventHandlers> for Vec<AttributeOrEventHandler> {
  fn into(self) -> SplitAttributeOrEventHandlers {
    let len = self.len();
    let attributes = Vec::with_capacity(len);
    let event_handlers = Vec::with_capacity(len);
    self.into_iter().fold(
      SplitAttributeOrEventHandlers(attributes, event_handlers, vec![]),
      |SplitAttributeOrEventHandlers(mut attributes, mut event_handlers, mut dom_ref), next_val| {
        match next_val {
          AttributeOrEventHandler::Attribute(attr) => attributes.push(attr),
          AttributeOrEventHandler::EventHandler(event_handler) => {
            event_handlers.push(event_handler)
          },
          AttributeOrEventHandler::DomRef(dom_ref_token) => {
            dom_ref.push(DomRefInfo::from_token_stream(dom_ref_token))
          },
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
