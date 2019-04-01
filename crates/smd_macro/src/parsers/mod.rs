#[macro_use]
mod many_custom;
mod attributes;
mod core;
mod event_names;
mod make_smithy_tokens;
mod util;
mod window_event_handlers;

pub use self::core::match_html_component;
