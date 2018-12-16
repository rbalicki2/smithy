#[cfg(test)]
mod tests {
  use smithy_types::{
    Component,
    HtmlToken,
    Node,
    UiEvent,
  };
  use std::collections::HashMap;

  #[test]
  fn simple_app_state() {
    struct SimpleAppState {
      pub is_transitioned: bool,
    }

    let mut app_state = SimpleAppState {
      is_transitioned: false,
    };

    let mut div = smd!(<div on_test={|_| app_state.is_transitioned = true}>
      { if app_state.is_transitioned { "yes" } else { "no" } }
    </div>);

    let render_result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "div".into(),
      attributes: HashMap::new(),
      children: vec![Node::Text("no".into())],
    })]);
    assert_eq!(div.render(), render_result);

    let event_handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0]);
    assert_eq!(event_handled, true);

    let render_result = Node::Vec(vec![Node::Dom(HtmlToken {
      node_type: "div".into(),
      attributes: HashMap::new(),
      children: vec![Node::Text("yes".into())],
    })]);
    assert_eq!(div.render(), render_result);
  }

  #[test]
  fn event_handler_basic_div() {
    // N.B. these tests are un-ergonomic! No one would use Smithy if this was the
    // recommended way to do things. However, as you can see from simple_app_state,
    // the general use case (that does not involve snooping at the app state)
    // doesn't require as many workarounds.

    struct AppState {
      called: bool,
      param: Option<bool>,
    }
    let app_state = std::rc::Rc::new(std::cell::RefCell::new(AppState {
      called: false,
      param: None,
    }));
    let app_state_2 = app_state.clone();

    // This div only responds to events with a path of [0]
    let mut div = smd!(<div
      on_test={|event_param: &bool| {
        let mut app_state = app_state.borrow_mut();
        app_state.param = Some(*event_param);
        app_state.called = true;
      }}
    />);

    // an event called with path: [] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // an event called with [0, 0] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0, 0]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // // an event called with [1] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[1]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // However, [0] will work!
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0]);
    assert_eq!(handled, true);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, true);
      assert_eq!(app_state_2.param, Some(true));
    }
  }

  #[test]
  fn event_handler_child_div() {
    struct AppState {
      called: bool,
      param: Option<bool>,
    }
    let app_state = std::rc::Rc::new(std::cell::RefCell::new(AppState {
      called: false,
      param: None,
    }));
    let app_state_2 = app_state.clone();

    // this div will only respond to events with path: [0, 0]
    let mut div = smd!(<div>
      <div
        on_test={|event_param: &bool| {
          let mut app_state = app_state.borrow_mut();
          app_state.param = Some(*event_param);
          app_state.called = true;
        }}
      />
    </div>);

    // an event called with path: [] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // an event called with [0] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // // an event called with [1] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[1]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // However, [0, 0] will work!
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0, 0]);
    assert_eq!(handled, true);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, true);
      assert_eq!(app_state_2.param, Some(true));
    }
  }

  #[test]
  fn event_handler_child_div_in_group() {
    struct AppState {
      called: bool,
      param: Option<bool>,
    }
    let app_state = std::rc::Rc::new(std::cell::RefCell::new(AppState {
      called: false,
      param: None,
    }));
    let app_state_2 = app_state.clone();

    // this div will only respond to events with path: [0, 0]
    let mut inner = smd!(<div
      on_test={|event_param: &bool| {
        let mut app_state = app_state.borrow_mut();
        app_state.param = Some(*event_param);
        app_state.called = true;
      }}
    />);
    let mut div = smd!(<div>{ &mut inner }</div>);

    // an event called with path: [] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // an event called with [0] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // // an event called with [0, 0] should not affect anything
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0, 0]);
    assert_eq!(handled, false);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, false);
      assert_eq!(app_state_2.param, None);
    }

    // However, [0, 0, 0] will work!
    // Why three zeroes?
    // * div = smd! is a vec (hence why smd!(<div/><div />) works), so
    //   the first zero references the first item in this vec.
    // * The div's first child is { ... } hence the second zero
    // * The results of inner = smd! is another vec, hence the last zero
    //
    // we cannot omit the second step (i.e. this cannot respond to [0, 0])
    // because otherwise <div>{ first }{ second }</div> would need to know
    // the length of { first } at compile time, in order to properly dispatch
    // events to { second }. Of course, this info is not available.
    let handled = div.handle_ui_event(&UiEvent::OnTest(true), &[0, 0, 0]);
    assert_eq!(handled, true);
    {
      let app_state_2 = app_state_2.borrow();
      assert_eq!(app_state_2.called, true);
      assert_eq!(app_state_2.param, Some(true));
    }
  }

  #[test]
  fn strings_do_not_handle_ui_events() {
    let inner = "inner";
    let mut div = smd!(<div>{ inner }</div>);
    assert_eq!(div.handle_ui_event(&UiEvent::OnTest(false), &[0, 0]), false);
  }

  #[test]
  fn text_nodes_do_not_handle_ui_events() {
    let mut div = smd!(<div>inner</div>);
    assert_eq!(div.handle_ui_event(&UiEvent::OnTest(false), &[0, 0]), false);
  }
}
