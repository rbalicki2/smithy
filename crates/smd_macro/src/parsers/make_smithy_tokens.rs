use crate::types::{
  DomRefInfo,
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
  dom_ref_infos: Vec<DomRefInfo>,
) -> TokenStream {
  println!("\n\n\nmake_component dom ref info {:?}", dom_ref_infos);

  let dom_ref_infos = dom_ref_infos
    .into_iter()
    .fold(quote!{}, |accum, dom_ref_info| {
      let stream = dom_ref_info.dom_ref;
      quote!{
        #accum
        #stream,
      }
    });
  let dom_ref_infos =
    quote!{ { let dom_refs: Vec<&mut smithy::types::DomRef> = vec![#dom_ref_infos]; dom_refs }};

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

  let group_lifecycle_event_handling = ui_event_handling_infos
    .iter()
    .filter(|info| info.is_group)
    .map(|info| (info.callback.clone(), info.reversed_path.clone()))
    .fold(quote!{}, |accum, (group, reversed_path)| {
      quote!{{
        #accum
        // let node_list =
        // N.B. this line fails - node_list is a vec, but this should be a vec of vecs
        // TODO think about this
        // This is when we apply post render to children
        // (#group).handle_post_render(node_list);
        (#group).handle_post_render(&vec![]);
        // N.B. cannot wrap in vec![node_list] because that has type Vec<&Vec<X>> instead of &Vec<Vec<X>>
        // DAMMIT
      }}
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

  // N.B. right now "lifecycle" == "post_render", but that needs to be disambiguated
  let inner_lifecycle_event_handling =
    lifecycle_event_handling_infos
      .into_iter()
      .fold(quote!{}, |accum, lifecycle_info| {
        let cb = lifecycle_info.callback;
        quote!{
          #accum
          (#cb)(node_list);
        }
      });

  quote!({
    use smithy::types as smithy_types;
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
        smithy_types::Phase::PostRendering(node_list) => {
          #group_lifecycle_event_handling
          #inner_lifecycle_event_handling
          smithy_types::PhaseResult::PostRendering
        },
        smithy_types::Phase::RefAssignment => {
          for dom_ref in #dom_ref_infos.into_iter() {
            // TODO find and assign the dom ref here
            // dom_ref.byah();
          }
          // web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("ref assignment {:?}", #dom_ref_infos)));
          // TODO call child ones
          smithy_types::PhaseResult::RefAssignment
        },
      }
    }));
    component
  })
}

pub fn make_text_node(s: String) -> TokenStream {
  quote!(smithy_types::Node::Text(#s.into()))
}
