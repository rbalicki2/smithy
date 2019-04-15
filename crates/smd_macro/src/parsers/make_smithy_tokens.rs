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
use quote::{
  quote,
  ToTokens,
};

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
  quote!(smithy::types::Node::Dom(smithy::types::HtmlToken {
    node_type: #name.into(),
    attributes: #attribute_initialization,
    children: #child_initialization,
  }))
}

fn vec_to_quote<X>(v: Vec<X>) -> TokenStream
where
  X: ToTokens,
{
  let ret = v
    .into_iter()
    .fold(quote! {}, |accum, item| quote!(#accum #item,));
  quote!(vec![#ret])
}

pub fn make_component(
  rendered_node: TokenStream,
  ui_event_handling_infos: Vec<UIEventHandlingInfo>,
  window_event_handling_infos: Vec<WindowEventHandlingInfo>,
  lifecycle_event_handling_infos: Vec<LifecycleEventHandlingInfo>,
  dom_ref_infos: Vec<DomRefInfo>,
) -> TokenStream {
  // TODO split ui_event_handling_infos into a vec of groups and a vec of non-groups
  // and deal with them separately in this function.
  // TODO even later: don't conflate these two! It's super weird that we have
  // groups represented as UIEventHandlingInfo

  let (groups, ui_event_handling_infos) =
    UIEventHandlingInfo::split_into_groups(ui_event_handling_infos);

  let (child_ref_assignment, group_window_event_handling) = groups
    .iter()
    .map(|info| (info.reversed_path.clone(), info.callback.clone()))
    .fold(
      (quote! {}, quote! {}),
      |(ref_accum, group_accum), (reversed_path, group)| {
        let quotable_path = vec_to_quote(reversed_path);
        (
          quote! {
            #ref_accum
            let mut path = #quotable_path.clone();
            path.reverse();
            let new_path = path_so_far.clone().into_iter().chain(path).collect();
            (#group).handle_ref_assignment(new_path);
          },
          quote! {
            #group_accum
            event_handled = (#group).handle_window_event(window_event) || event_handled;
          },
        )
      },
    );

  let dom_ref_infos = dom_ref_infos
    .into_iter()
    .fold(quote! {}, |accum, dom_ref_info| {
      let dom_ref = dom_ref_info.dom_ref;
      let path = vec_to_quote(dom_ref_info.reversed_path);
      quote! {
        #accum
        (#path, #dom_ref),
      }
    });
  let dom_ref_infos =
    quote! { { let dom_refs: Vec<smithy::types::DomRefWithPath> = vec![#dom_ref_infos]; dom_refs }};
  let ref_assignment_quote = quote! {
    let document = web_sys::window().unwrap().document().unwrap();
    for (path, dom_ref) in (#dom_ref_infos).into_iter() {
      use wasm_bindgen::JsCast;
      let strs = path_so_far
        .clone()
        .into_iter()
        .chain(path)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

      let selector = strs.join(",");
      // TODO avoid unwrapping here, and try to avoid calling .query_selector
      // every time.
      let el_opt: Option<web_sys::HtmlElement> = document
        .query_selector(&format!("[data-smithy-path=\"{}\"]", selector))
        .unwrap()
        .map(JsCast::unchecked_into);

      *dom_ref = el_opt;
    }
    #child_ref_assignment
  };

  let group_lifecycle_event_handling =
    groups
      .iter()
      .map(|info| info.callback.clone())
      .fold(quote! {}, |accum, group| {
        quote! {{
          #accum
          (#group).handle_post_render();
        }}
      });

  // inner_ui_event_handling is made in two parts.
  // Part 1: handle events for non-groups (true ui event handlers)
  let inner_ui_event_handling =
    ui_event_handling_infos
      .into_iter()
      .fold(quote! {}, |accum, ui_event_handling_info| {
        let path = ui_event_handling_info.get_path_match();
        let callback = ui_event_handling_info.callback;
        match ui_event_handling_info.event {
          Some(event) => {
            let event = Ident::new(&event, Span::call_site());
            quote! {
              #accum
              (smithy::types::UiEvent::#event(val), #path) => {
                (#callback)(val);
                smithy::types::PhaseResult::UiEventHandling(true)
              },
            }
          },
          None => panic!("should not happen, this is ensured by split_into_groups"),
        }
      });

  // Part 2: handle groups
  let inner_ui_event_handling = groups
    .into_iter()
    .fold(inner_ui_event_handling, |accum, group| {
      let path = group.get_path_match();
      let callback = group.callback;

      quote! {
        #accum
        (evt, #path) => smithy::types::PhaseResult::UiEventHandling(
          #callback.handle_ui_event(evt, rest)
        ),
      }
    });

  let inner_window_event_handling =
    window_event_handling_infos
      .into_iter()
      .fold(quote! {}, |accum, window_event_handling_info| {
        let WindowEventHandlingInfo { event, callback } = window_event_handling_info;
        let event = Ident::new(&event, Span::call_site());
        quote! {
          #accum
          smithy::types::WindowEvent::#event(val) => {
            (#callback)(val);
            smithy::types::PhaseResult::WindowEventHandling(true)
          }
        }
      });

  // TODO disambiguate this
  // N.B. right now "lifecycle" == "post_render", but that needs to be disambiguated
  let inner_lifecycle_event_handling =
    lifecycle_event_handling_infos
      .into_iter()
      .fold(quote! {}, |accum, lifecycle_info| {
        let cb = lifecycle_info.callback;
        quote! {
          #accum
          (#cb)();
        }
      });

  quote!({
    #[allow(dead_code)]
    use smithy::types::Component;
    let component: smithy::types::SmithyComponent = smithy::types::SmithyComponent(Box::new(move |phase| {
      match phase {
        smithy::types::Phase::Rendering => smithy::types::PhaseResult::Rendering(#rendered_node),
        smithy::types::Phase::UiEventHandling(ui_event_handling) => {
          match ui_event_handling {
            #inner_ui_event_handling
            _ => smithy::types::PhaseResult::UiEventHandling(false)
          }
        },
        smithy::types::Phase::WindowEventHandling(window_event) => {
          let mut event_handled = false;
          #group_window_event_handling
          match window_event {
            #inner_window_event_handling
            _ => smithy::types::PhaseResult::WindowEventHandling(event_handled),
          }
        },
        smithy::types::Phase::PostRendering => {
          #group_lifecycle_event_handling
          #inner_lifecycle_event_handling
          smithy::types::PhaseResult::PostRendering
        },
        smithy::types::Phase::RefAssignment(path_so_far) => {
          #ref_assignment_quote
          smithy::types::PhaseResult::RefAssignment
        },
      }
    }));
    component
  })
}

pub fn make_text_node(s: String) -> TokenStream {
  quote!(smithy::types::Node::Text(#s.into()))
}
