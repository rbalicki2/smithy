use crate::types::TokenTreeSlice;
use nom::{
  call,
  named,
};
use proc_macro2::TokenStream;

#[macro_use]
mod many_custom;
mod attributes;
mod core;
mod event_names;
mod make_smithy_tokens;
mod util;

named!(
  pub match_smd <TokenTreeSlice, TokenStream>,
  call!(self::core::match_html_component)
);
