use super::{
  event_names::{
    LIFECYCLE_EVENT_NAMES,
    WINDOW_EVENT_NAMES,
  },
  util,
};
use crate::types::{
  GlobalEventHandlingInfo,
  LifecycleEventHandlingInfo,
  TokenTreeSlice,
  WindowEventHandlingInfo,
};

use nom::{
  alt,
  apply,
  error_position,
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
  pub match_window_event_handlers <TokenTreeSlice, GlobalEventHandlingInfo>,
  alt!(
    map!(
      tuple!(
        apply!(util::match_string_from_hashmap, &WINDOW_EVENT_NAMES),
        apply!(util::match_punct, Some('='), Some(Spacing::Alone), vec![]),
        apply!(util::match_group, Some(Delimiter::Brace)),
        apply!(util::match_punct, Some(';'), None, vec![])
      ),
      |(event, _, callback, _)| {
        GlobalEventHandlingInfo::Window(WindowEventHandlingInfo {
          event,
          callback: quote!(#callback),
        })
      }
    )
      | map!(
        tuple!(
          apply!(util::match_string_from_hashmap, &LIFECYCLE_EVENT_NAMES),
          apply!(util::match_punct, Some('='), Some(Spacing::Alone), vec![]),
          apply!(util::match_group, Some(Delimiter::Brace)),
          apply!(util::match_punct, Some(';'), None, vec![])
        ),
        |(lifecycle_event, _, callback, _)| {
          GlobalEventHandlingInfo::Lifecycle(LifecycleEventHandlingInfo {
            lifecycle_event,
            callback: quote!(#callback),
          })
        }
      )
  )
);
