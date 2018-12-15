use super::{
  event_names::WINDOW_EVENT_NAMES,
  util,
};
use crate::types::{
  TokenTreeSlice,
  WindowEventHandlingInfo,
};
use nom::{
  apply,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Delimiter,
  Spacing,
};
use quote::quote;

named!(
  pub match_window_event_handlers <TokenTreeSlice, WindowEventHandlingInfo>,
  map!(
    tuple!(
      apply!(util::match_string_from_hashmap, &WINDOW_EVENT_NAMES),
      apply!(util::match_punct, Some('='), Some(Spacing::Alone), vec![]),
      apply!(util::match_group, Some(Delimiter::Brace)),
      apply!(util::match_punct, Some(';'), None, vec![])
    ),
    |(event, _, callback, _)| {
      WindowEventHandlingInfo {
        event,
        callback: quote!(#callback),
      }
    }
  )
);
