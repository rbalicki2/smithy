use crate::types::{
  AttributeOrEventHandler,
  DomRefInfo,
  GlobalEventHandlingInfo,
  LifecycleEventHandlingInfo,
  SplitAttributeOrEventHandlers,
  SplitTokenStreamEventHandlingInfoPairs,
  TokenStreamEventHandlingInfoPair,
  TokenTreeSlice,
  UIEventHandlingInfo,
  WindowEventHandlingInfo,
};
use nom::{
  alt,
  apply,
  call,
  delimited,
  error_position,
  map,
  named,
  tuple,
  tuple_parser,
};
use proc_macro2::{
  Delimiter,
  Spacing,
  TokenStream,
};
use quote::quote;
use std::iter::Extend;

use super::{
  make_smithy_tokens::{
    make_html_tokens,
    make_text_node,
  },
  util,
  window_event_handlers::match_window_event_handlers,
};

named!(
  match_self_closing_token <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  map!(
    delimited!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
      tuple!(
        apply!(util::match_ident, None, false),
        many_0_custom!(super::attributes::match_attribute)
      ),
      tuple!(
        apply!(util::match_punct, Some('/'), Some(Spacing::Joint), vec![]),
        apply!(util::match_punct, Some('>'), None, vec![])
      )
    ),
    |(name, attributes_and_event_handlers)| {
      // does not work
      let SplitAttributeOrEventHandlers(attributes, event_handlers, dom_ref_vec)
        = attributes_and_event_handlers.into();

      // let dom_ref_vec = dom_ref_opt.map(DomRefInfo::from_token_stream).into_iter().collect::<Vec<DomRefInfo>>();

      let component = make_html_tokens(name, attributes, vec![]);
      // println!("\nself closing\n{:?}", dom_ref_vec);

      let event_handlers = event_handlers
        .into_iter()
        .map(UIEventHandlingInfo::from_string_token_stream_pair)
        .collect();
      (component, event_handlers, dom_ref_vec)
    }
  )
);

named!(
  match_opening_tag <TokenTreeSlice, (String, Vec<AttributeOrEventHandler>)>,
  delimited!(
    apply!(util::match_punct, Some('<'), Some(Spacing::Alone), vec![]),
    tuple!(
      apply!(util::match_ident, None, false),
      many_0_custom!(super::attributes::match_attribute)
    ),
    apply!(util::match_punct, Some('>'), None, vec![])
  )
);

named!(
  match_closing_tag <TokenTreeSlice, String>,
  delimited!(
    tuple!(
      apply!(util::match_punct, Some('<'), Some(Spacing::Joint), vec![]),
      apply!(util::match_punct, Some('/'), Some(Spacing::Alone), vec![])
    ),
    apply!(util::match_ident, None, false),
    apply!(util::match_punct, Some('>'), None, vec![])
  )
);

named!(
  match_regular_token <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  map!(
    tuple!(
      match_opening_tag,
      many_0_custom!(match_node),
      match_closing_tag
    ),
    |((name, attributes_and_event_handlers), children_and_events, closing_tag_name)| {
      assert_eq!(
        name,
        closing_tag_name,
        "Opening and closing tag names ({} and {}) do not match",
        name,
        closing_tag_name,
      );
      let SplitAttributeOrEventHandlers(attributes, event_handlers, dom_ref_opt)
        = attributes_and_event_handlers.into();
      let SplitTokenStreamEventHandlingInfoPairs(children, child_event_infos, mut dom_ref_vec)
        = children_and_events.into();

      let token_stream = make_html_tokens(name, attributes, children);

      let mut event_infos: Vec<UIEventHandlingInfo> = event_handlers
        .into_iter()
        .map(UIEventHandlingInfo::from_string_token_stream_pair)
        .collect();
      event_infos.extend(child_event_infos.into_iter());

      // let dom_ref_opt = dom_ref_opt.map(DomRefInfo::from_token_stream);
      dom_ref_vec.extend(dom_ref_opt.into_iter());

      (token_stream, event_infos, dom_ref_vec)
    }
  )
);

named!(
  match_html_token <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  alt!(
    match_self_closing_token
      | match_regular_token
  )
);

// N.B. this is separated because there seems to be a bug
// in many_1_custom. TODO look at this
named!(
  match_ident_2 <TokenTreeSlice, String>,
  alt!(
    apply!(util::match_ident, None, true)
      | apply!(util::match_punct, None, None, vec!['<'])
      | call!(util::match_literal_as_string)
  )
);

named!(
  match_string_as_node <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  map!(
    many_1_custom!(match_ident_2),
    |vec| {
      let joined = vec.iter().map(|ident| ident.to_string()).collect::<Vec<String>>().join("");
      (make_text_node(joined), vec![], vec![])
    }
  )
);

named!(
  match_group <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  map!(
    apply!(util::match_group, Some(Delimiter::Brace)),
    |x| (quote!(#x.render()), vec![
      UIEventHandlingInfo {
        reversed_path: vec![],
        event: None,
        callback: quote!(#x),
        is_group: true,
      }
    ], vec![])
  )
);

named!(
  match_node <TokenTreeSlice, TokenStreamEventHandlingInfoPair>,
  alt!(
    match_html_token
      | match_string_as_node
      | match_group
  )
);

// N.B. this is the "entry point" of this file.
named!(
  pub match_html_component <TokenTreeSlice, TokenStream>,
  map!(
    tuple!(
      many_0_custom!(match_window_event_handlers),
      many_1_custom!(match_node)
    ),
    |(global_event_handling_infos, dom_vec)| {
      // dom_vec is a vector of (TokenStream, Vec<UIHandlingInfo>, Vec<DomRefInfo>)
      // where the last token stream is the ref, if present
      // innerref is not present here
      println!("vec dom ref info in match_html_component {:?}", dom_vec);
      let (vec_of_node_tokens, event_handling_infos, dom_ref_vec) = dom_vec
        .into_iter()
        .enumerate()
        .fold(
          (vec![], vec![], vec![]),
          |(mut vec_of_node_tokens, mut event_handling_infos, mut vec_of_dom_refs), (i, (token, current_event_handling_infos, dom_ref_vec))| {
            vec_of_node_tokens.push(token);

            // append i to the path of the current_event_handling_infos and
            // append that vec to event_handling_infos
            let mut vec = current_event_handling_infos.into_iter().map(|mut info| {
              info.reversed_path.push(i);
              info
            }).collect::<Vec<UIEventHandlingInfo>>();
            event_handling_infos.append(&mut vec);

            // TODO clean this up, vec_of_refs.extend(...) and the like
            // if let Some(mut dom_ref) = dom_ref_opt {
            for mut dom_ref in dom_ref_vec {
              // println!("dom ref found");
              // println!("{}", i);
              // i is the current index of the guy, so maybe we need
              // to do this same path stuff and thus pass down the path
              // in the dom_ref_opt
              dom_ref.reversed_path.push(i);
              vec_of_dom_refs.push(dom_ref);
            }
            (vec_of_node_tokens, event_handling_infos, vec_of_dom_refs)
          }
        );
      let token = util::reduce_vec_to_node(&vec_of_node_tokens);

      let (window_event_handler_infos, lifecycle_event_handling_infos): (Vec<WindowEventHandlingInfo>, Vec<LifecycleEventHandlingInfo>)
        = global_event_handling_infos
          .into_iter()
          .fold(
            (vec![], vec![]),
            |(mut window_event_handler_infos, mut lifecycle_event_handling_infos), current_global_event_handling_info| {
              // Can this be done more parsimoniously, e.g. using a library?
              match current_global_event_handling_info {
                GlobalEventHandlingInfo::Window(window_event_handling_info) => {
                  window_event_handler_infos.push(window_event_handling_info);
                },
                GlobalEventHandlingInfo::Lifecycle(lifecycle_event_handling_info) => {
                  lifecycle_event_handling_infos.push(lifecycle_event_handling_info);
                },
              };
              (window_event_handler_infos, lifecycle_event_handling_infos)
            }
          );
      super::make_smithy_tokens::make_component(token, event_handling_infos, window_event_handler_infos, lifecycle_event_handling_infos, dom_ref_vec)
    }
  )
);
