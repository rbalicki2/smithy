#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  // TODO cannot put a and b in the vec directly! Sad!
  let mut my_ref: Option<web_sys::HtmlElement> = None;
  // let a = ;
  // .into_collapsed_node(vec![]);
  // let a = smithy::with_ref!(my_ref, (div)).render(); // .into_collapsed_node(vec![]),
  let a = {
    use ::smithy::types::Component as _;
    ::smithy::types::SmithyComponent(::std::boxed::Box::new(|phase| match phase {
      ::smithy::types::Phase::RefAssignment(ref path) if path.is_empty() => {
        let mut path = path.clone();
        path.push(0);
        let selector = path
          .into_iter()
          .map(|x| x.to_string())
          .collect::<Vec<String>>()
          .join(",");
        let document = web_sys::window().unwrap().document().unwrap();
        let el_opt: Option<::web_sys::HtmlElement> = document
          .query_selector(&format!("[data-smithy-path=\"{}\"]", selector))
          .unwrap()
          .map(::wasm_bindgen::JsCast::unchecked_into);
        my_ref = el_opt;
        ::smithy::types::PhaseResult::RefAssignment
      },
      ::smithy::types::Phase::Rendering => {
        ::smithy::types::PhaseResult::Rendering(::smithy::types::Node::Vec(vec![
          ::smithy::types::Node::Dom(::smithy::types::HtmlToken {
            node_type: "div".to_string(),
            children: vec![],
            attributes: ::std::collections::HashMap::new(),
          }),
        ]))
      },
      ::smithy::types::Phase::UiEventHandling(ui_event_handling) => match ui_event_handling {
        _ => ::smithy::types::PhaseResult::UiEventHandling(false),
      },
      ::smithy::types::Phase::PostRendering => {
        {
          ::smithy::types::PhaseResult::PostRendering
        };
        ::smithy::types::PhaseResult::PostRendering
      },
      _ => ::smithy::types::PhaseResult::UiEventHandling(false),
    }))
  }
  .0(smithy::types::Phase::Rendering)
  .unwrap_node();
  // .render();
  // let b = smithy::post_render!(|| my_ref.is_some()).render();
  let b = ({
    use ::smithy::types::Component as _;
    (::std::boxed::Box::new(|phase| match phase {
      ::smithy::types::Phase::Rendering => {
        ::smithy::types::PhaseResult::Rendering(::smithy::types::Node::Vec(vec![]))
      },
      ::smithy::types::Phase::PostRendering => {
        (|| my_ref.is_some())();
        ::smithy::types::PhaseResult::PostRendering
      },
      _ => ::smithy::types::PhaseResult::UiEventHandling(false),
    }))
  })(smithy::types::Phase::Rendering)
  .unwrap_node();
  // .render();
  let vecyah = vec![
    a, b,
    // .render();
  ];
  return;
  // let mut result = {
  //   use ::smithy::types::Component as _;
  //   ::smithy::types::SmithyComponent(::std::boxed::Box::new(move |phase| match phase {
  //     ::smithy::types::Phase::Rendering => {
  //       ::smithy::types::PhaseResult::Rendering(::smithy::types::Node::Vec(vec![
  //         {
  //           smithy::post_render!(|| web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
  //             &my_ref.is_some().to_string()
  //           )))
  //           // .render();
  //           .render()
  //           // smithy::types::Node::Vec(vec![])
  //         },
  //         smithy::with_ref!(my_ref, (div["some div"])).render(),
  //         // ::smithy::types::Node::Dom(::smithy::types::HtmlToken {
  //         //   node_type: "h1".to_string(),
  //         //   children: vec![
  //         //     "some title".render(),
  //         //   ],
  //         //   attributes: ::std::collections::HashMap::new(),
  //         // }),
  //       ]))
  //     },
  //     ::smithy::types::Phase::UiEventHandling(ui_event_handling) => match ui_event_handling {
  //       (event, [0usize, rest @ ..]) => smithy::types::PhaseResult::UiEventHandling(
  //         smithy::post_render!(|| web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
  //           &my_ref.is_some().to_string()
  //         )))
  //         .handle_ui_event(event, rest),
  //       ),
  //       (event, [1usize, 0usize, rest @ ..]) => {
  //         smithy::types::PhaseResult::UiEventHandling("some title".handle_ui_event(event, rest))
  //       },
  //       (event, [1usize, 1usize, rest @ ..]) => smithy::types::PhaseResult::UiEventHandling(
  //         smithy::with_ref!(my_ref, (div["some div"])).handle_ui_event(event, rest),
  //       ),
  //       _ => ::smithy::types::PhaseResult::UiEventHandling(false),
  //     },
  //     ::smithy::types::Phase::PostRendering => {
  //       (smithy::post_render!(|| web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(
  //         &my_ref.is_some().to_string()
  //       ))))
  //       .handle_post_render();
  //       {
  //         ("some title").handle_post_render();
  //         (smithy::with_ref!(my_ref, (div["some div"]))).handle_post_render();
  //         ::smithy::types::PhaseResult::PostRendering
  //       };
  //       ::smithy::types::PhaseResult::PostRendering
  //     },
  //     _ => ::smithy::types::PhaseResult::UiEventHandling(false),
  //   }))
  // };

  // let mut result = rsx![(div[smithy::post_render!(|| {})])];

  use smithy::types::Component as _;
  // let component = result.render();
  // println!("rendered {:?}", component);

  // ----------------------------------------------------------------
  // What to work on next?
  // rsx equivalent of post_render! and with_ref!
  // global event handlers done similarly
  // TODO's in convert_to_components
  // Post merge
  // Remove Vec<Node> from Node or allow render() to return Vec<Node>
  // Get events to be generic
  // ----------------------------------------------------------------
}
