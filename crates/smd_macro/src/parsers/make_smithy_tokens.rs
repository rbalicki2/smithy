use crate::types::{
  LifecycleEventHandlingInfo,
  StringTokenStreamPair,
  UIEventHandlingInfo,
  WindowEventHandlingInfo,
};
use proc_macro2::{
  Ident,
  Span,
  TokenStream,
};
use quote::quote;

pub fn make_html_tokens(
  name: String,
  attributes: Vec<StringTokenStreamPair>,
  children: Vec<TokenStream>,
) -> TokenStream {
  let attribute_initialization = if attributes.len() > 0 {
    let attribute_insertion = attributes.into_iter().fold(quote!(), |accum, (key, val)| {
      quote!(
        #accum
        map.insert(#key.into(), #val.into());
      )
    });
    quote!({
      let mut map = std::collections::HashMap::new();
      #attribute_insertion
      map
    })
  } else {
    quote!(std::collections::HashMap::new())
  };

  let child_initialization = if children.len() > 0 {
    let len = children.len();
    let child_insertion = children.into_iter().fold(quote!(), |accum, child| {
      quote!(
        #accum
        children.push(#child);
      )
    });
    quote!({
      let mut children = Vec::with_capacity(#len);
      #child_insertion
      children
    })
  } else {
    quote!(vec![])
  };

  // TODO implement and call .flatten_children
  quote!(smithy_types::Node::Dom(smithy_types::HtmlToken {
    node_type: #name.into(),
    attributes: #attribute_initialization,
    children: #child_initialization,
  }))
}

pub fn make_component(
  token: TokenStream,
  ui_event_handling_infos: Vec<UIEventHandlingInfo>,
  window_event_handling_infos: Vec<WindowEventHandlingInfo>,
  lifecycle_event_handling_infos: Vec<LifecycleEventHandlingInfo>,
) -> TokenStream {
  let group_window_event_handling = ui_event_handling_infos
    .iter()
    .filter(|info| info.is_group)
    .map(|info| info.callback.clone())
    .fold(quote!{}, |accum, group| {
      quote!{
        #accum
        event_handled = (#group).handle_window_event(window_event) || event_handled;
      }
    });

  let inner_ui_event_handling =
    ui_event_handling_infos
      .into_iter()
      .fold(quote!{}, |accum, ui_event_handling_info| {
        let path = ui_event_handling_info.get_path_match();
        let callback = ui_event_handling_info.callback;
        match ui_event_handling_info.event {
          Some(event) => {
            let event = Ident::new(&event, Span::call_site());
            quote!{
              #accum
              (smithy_types::UiEvent::#event(val), #path) => {
                (#callback)(val);
                smithy_types::PhaseResult::UiEventHandling(true)
              },
            }
          },
          None => quote!{
            #accum
            // N.B. path (aka get_path_match) matches the rest of the path as the variable rest
            // which we pass onto the child
            (evt, #path) => smithy_types::PhaseResult::UiEventHandling(#callback.handle_ui_event(evt, rest)),
          },
        }
      });

  let inner_window_event_handling =
    window_event_handling_infos
      .into_iter()
      .fold(quote!{}, |accum, window_event_handling_info| {
        let WindowEventHandlingInfo { event, callback } = window_event_handling_info;
        let event = Ident::new(&event, Span::call_site());
        quote!{
          #accum
          smithy_types::WindowEvent::#event(val) => {
            (#callback)(val);
            smithy_types::PhaseResult::WindowEventHandling(true)
          }
        }
      });

  quote!({
    use smithy::types as smithy_types;
    // extern crate web_sys;
    let component: smithy_types::SmithyComponent = smithy_types::SmithyComponent(Box::new(move |phase| {
      match phase {
        smithy_types::Phase::Rendering => smithy_types::PhaseResult::Rendering(#token),
        smithy_types::Phase::UiEventHandling(ui_event_handling) => {
          match ui_event_handling {
            #inner_ui_event_handling
            _ => smithy_types::PhaseResult::UiEventHandling(false)
          }
        },
        smithy_types::Phase::WindowEventHandling(window_event) => {
          let mut event_handled = false;
          #group_window_event_handling
          match window_event {
            #inner_window_event_handling
            _ => smithy_types::PhaseResult::WindowEventHandling(event_handled),
          }
        },
        smithy_types::Phase::PostRendering(el) => {
          // N.B. this breaks tests in smd_macro!!!
          // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("post rendering in make smithy tokens (resume here!)")));
          smithy_types::PhaseResult::PostRendering
        },
      }
    }));
    component
  })
}

pub fn make_text_node(s: String) -> TokenStream {
  quote!(smithy_types::Node::Text(#s.into()))
}
