use proc_macro2::{
  Ident,
  Span,
  TokenStream,
};
use quote::quote;

pub struct EventHandlingInfo {
  pub path: Box<smithy_types::Path>,
  // TODO
  pub event: String,
  pub callback: TokenStream,
}

// TODO handle children and event handling attributes
pub fn make_html_tokens(
  name: String,
  attributes: Vec<(String, TokenStream)>,
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

fn path_to_tokens(path: Box<[usize]>) -> TokenStream {
  let inner = path.into_iter().fold(quote!{}, |accum, path_item| {
    quote!{ #accum #path_item, }
  });
  quote!{
    [ #inner ]
  }
}

pub fn make_component(
  token: TokenStream,
  event_handling_infos: Vec<EventHandlingInfo>,
) -> TokenStream {
  // TODO possibly sort event_handling_infos
  let inner_event_handling =
    event_handling_infos
      .into_iter()
      .fold(quote!{}, |accum, event_handling_info| {
        let path = path_to_tokens(event_handling_info.path);
        let callback = event_handling_info.callback;
        let event = Ident::new(&event_handling_info.event, Span::call_site());
        quote!{
          #accum
          (smithy_types::Event::#event(val), #path) => {
            (#callback)(val);
            smithy_types::PhaseResult::EventHandling(true)
          },
        }
      });
  quote!({
    let component: smithy_types::Component = smithy_types::Component(Box::new(move |phase| {
      match phase {
        smithy_types::Phase::Rendering => smithy_types::PhaseResult::Rendering(#token),
        smithy_types::Phase::EventHandling(event_handling) => {
          match event_handling {
            #inner_event_handling
            _ => smithy_types::PhaseResult::EventHandling(false)
          }
        }
      }
    }));
    component
  })
}

pub fn make_text_node(s: String) -> TokenStream {
  quote!(smithy_types::Node::Text(#s.into()))
}
