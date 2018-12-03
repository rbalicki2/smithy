use super::{
  event_names::EVENT_NAMES,
  util,
};
use crate::types::{
  AttributeOrEventHandler,
  TokenTreeSlice,
};
use nom::{
  alt,
  apply,
  call,
  error_position,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Delimiter,
  TokenStream,
};

// N.B. there are three types of attributes values supported:
// groups, empty and literals. Obviously, only groups can hold event handlers
// (callbacks), so we either call assert_is_not_event_handler or we
// switch on whether it is an event handler.
//
// In the EventHandler case, the string *is changed to upper camel case*
// to match the Event type! Beware!
// (Should we use a newtype?)

fn assert_is_not_event_handler(string: &String) {
  assert!(
    !EVENT_NAMES.contains_key(string),
    format!(
      "attribute {} is an event name, but was not followed by curly braces",
      string
    )
  );
}

impl AttributeOrEventHandler {
  fn create_from_string(string: String, t: TokenStream) -> AttributeOrEventHandler {
    match EVENT_NAMES.get(&string) {
      Some(event_name) => AttributeOrEventHandler::EventHandler((event_name.to_string(), t)),
      None => AttributeOrEventHandler::Attribute((string, t)),
    }
  }
}

named!(
  pub match_attribute <TokenTreeSlice, AttributeOrEventHandler>,
  alt!(
    alt!(
      map!(
        tuple!(
          apply!(util::match_ident, None, false),
          apply!(util::match_punct, Some('='), None, vec![]),
          map!(apply!(util::match_group, Some(Delimiter::Brace)), super::util::enquote)
        ),
        |val| {
          AttributeOrEventHandler::create_from_string(val.0, val.2)
        }
      )
        | map!(
          tuple!(
            apply!(util::match_ident, None, false),
            apply!(util::match_punct, Some('='), None, vec![]),
            map!(call!(util::match_literal), super::util::enquote)
          ),
          |val| {
            assert_is_not_event_handler(&val.0);
            AttributeOrEventHandler::Attribute((val.0, val.2))
          }
        )
        | map!(
          apply!(util::match_ident, None, false),
          |attr_name| {
            assert_is_not_event_handler(&attr_name);
            AttributeOrEventHandler::Attribute((attr_name, quote::quote!("")))
          }
        )
    )
  )
);
