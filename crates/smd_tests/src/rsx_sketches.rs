use rsx_macro::rsx;

#[test]
fn foo() {
  // static tag name
  // dynamic tag name

  // tag w/empty attributes
  // tag w/static attributes
  // tag w/static attribute keys and dynamic attribute values
  // tag w/dynamic attribute keys and static attribute values
  // tag w/dynamic attribute keys and dynamic attribute values
  // tag w/interpolated object as part of attributes

  // tag w/omitted attributes and empty event handlers
  // *same as above for event handlers*

  // tag w/inline (rsx!) children
  // tag w/interpolated children

  // global event handlers...? (separate macro?)
  // can't be separate macro or everything needs to be wrapped
  // in an Rc<RefCell<T>> ? Maybe not, maybe it just needs to be
  // a non-move closure

  // ref and post render... in event handler??? WTF THAT'S WEIRD
  // if it's generic over events

  // cannot have a with_ref! macro because then we have to decide
  // what type of closure (move vs non move) it would have to be,
  // and we'd have to have a special with_ref for the outermost one
  // or maybe just wrap it in rsx, a la:
  rsx![with_ref!(&mut my_ref, (div))];
  // is there some issue with nested rsx! here? I think a non-move
  // closure handles all these cases... right, since the outer rsx!
  // macro captures all the variables...?

  let a = rsx![
    ({ "div" }
      // attributes
      { foo: "bar", is_poop, {attribute_name}: value }
      // event handlers
      { on_click: |_| {}, on_mouse_over: handler, {handler_name}: byah }
      [
        (div)
        {component}
      ]
    )
  ];

  let a = rsx![
    (a {href: "www.google.com", ...foo} [
      "come to ",
      (b ["google"]),
      (if { true } {}),
      (hr + 3),
      some_text,
      some_fn(&mut some_var)
    ])
  ];

  let a = rsx![
    (html [
      (head [
        (meta { charset: "UTF-8"}),
        (title ["My Smithy App"]),
      ])
      (body [
        (script { src: "./index.js" }),
        (script ["if (typeof wasm_bindgen !== 'undefined') { wasm_bindgen('./index_bg.wasm') }"]),
        (div { id: "app" })
      ])
    ])
  ];

  let a = rsx![
    (a {} { on_click: handler } ["click me"])
  ];

  let a = rsx![
    post_render!(|| {
      // if let Some(input)
    }),
    (style { type: "text/css" } [ css.as_css_string() ]),
    (div { class: css.classes.board } [
      {
        clone_and_zip(board.iter_mut(), &current_player).map(|(mut item, current_player)| {
          if let Some(player) = item {
            rsx![
              (div { class: css.classes.square } [player.to_string()])
            ]
          } else {
            rsx![
              (div
                { class: format!("{} {}", css.classes.square, css.classes.square_selectable)}
                {
                  on_click: |_| {
                    *item = Some(*current_player.borrow());
                    current_player.borrow_mut().next();
                  }
                }
              )
            ]
          }
        }).collect::<Vec<_>>()
      }
    ]),
    {
      if let Some(player) = board.winner() {
        Some(rsx![
          (div { class: css.classes.overlay_background }),
          (div { class: css.class.overlay_text } [
            "Player ",
            player.to_string(),
            " won!",
          ])
        ])
      } else { None }
    },
    (div { class: css.classes.restart } {
      on_click: |_| {
        board = Board::empty();
        current_player = Rc::new(RefCell::new(Player::X));
      }
    } [
      "Restart game"
    ])
  ];

  let mut input_ref: Option<web_sys::HtmlElement> = None;
  let mut val = "".to_string();
  let a = rsx![
    post_render!(|| {
      if let Some(input) = input_ref {
        input.set_value(val);
      }
    }),
    with_ref!(&mut input_ref, (input)),
    (input { value: val } {
      on_input: |e: web_sys::InputEvent| {
        let target: web_sys::HtmlInputElement = e.target.unchecked_into();
        val = target.get_value();
      }
      // NAH:
    } { ref: &mut input_ref }),
    // (input {} {} {ref: Some(&mut input_ref) }),
  ];

  #[derive(Default)]
  struct InputBuilder<'a> {
    transform: Option<Box<Fn(String) -> String>>,
    input_ref: Option<&'a mut Option<web_sys::HtmlElement>>,
  }

  impl<'a> InputBuilder<'a> {
    fn new() -> Self {
      InputBuilder {
        transform: None,
        input_ref: None,
      }
    }

    fn transform(mut self, transform: Box<Fn(String) -> String>) -> Self {
      self.transform = Some(transform);
      self
    }

    fn input_ref(mut self, my_ref: &'a mut Option<web_sys::HtmlElement>) -> Self {
      self.input_ref = Some(my_ref);
      self
    }

    fn build(self) -> smithy::types::SmithyComponent<'a> {
      let input_ref = self.input_ref.unwrap_or(None);
      let app = rsx!(
        post_render(|| {
          if let Some(input) = input_ref {
            input.set_value(val);
          }
        })
        with_ref_opt!(
          input_ref,
          (input {
            on_input: |e: web_sys::InputEvent| {
              let target: web_sys::HtmlInputElement = e.target.unchecked_into();
              if let Some(transform) = self.transform {
                val = transform(target.get_value());
              } else {
                val = target.get_value();
              }
            }
          })
        ),
      );
      unimplemented!()
    }
  }

  let render_input = |val: &mut str| {
    let mut input_ref: Option<web_sys::HtmlElement> = None;
    rsx![
      post_render!(|| {
        if let Some(input) = input_ref {
          input.set_value(val);
        }
      }),
      with_ref!(
        &mut input_ref,
        (input { value: val } {
          on_input: |e: web_sys::InputEvent| {
            let target: web_sys::HtmlInputElement = e.target.unchecked_into();
            val = target.get_value();
          }
        })
      )
    ];
  };

  let a = rsx![
    render_input(&mut val),
    (div {} { on_click: |_| val = "reset input" } ["reset input"])
  ];

  let a = rsx![on_hash_change!(|_| val = "hash changed")];

  println!("A={}", a);
}
