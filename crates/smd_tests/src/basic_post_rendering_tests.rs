#[cfg(test)]
mod tests {
  use smithy::{
    self,
    smd,
    types::Component,
  };
  use std::{
    cell::RefCell,
    rc::Rc,
  };

  #[test]
  fn basic_post_render() {
    let post_render_has_been_called = Rc::new(RefCell::new(false));
    let post_render_has_been_called_2 = post_render_has_been_called.clone();
    let mut app = smd!(
      post_render={|| { *post_render_has_been_called.borrow_mut() = true; }};
      <div />
    );

    app.handle_post_render();
    assert!(*post_render_has_been_called_2.borrow());
  }

  #[test]
  fn nested_post_render() {
    let post_render_has_been_called = Rc::new(RefCell::new(false));
    let post_render_has_been_called_2 = post_render_has_been_called.clone();
    let mut inner = smd!(
      post_render={|| { *post_render_has_been_called.borrow_mut() = true; }};
      <div />
    );
    let mut outer = smd!({ &mut inner });

    outer.handle_post_render();
    assert!(*post_render_has_been_called_2.borrow());
  }

  #[test]
  fn post_render_happens_in_order() {
    let post_render: Rc<RefCell<Vec<&'static str>>> = Rc::new(RefCell::new(vec![]));
    let post_render_2 = post_render.clone();
    let post_render_3 = post_render.clone();

    let mut first = smd!(
      post_render={|| { post_render.borrow_mut().push("first"); }};
      <div />
    );
    let mut second = smd!(
      post_render={|| { post_render_2.borrow_mut().push("second"); }};
      <div />
    );
    let mut outer = smd!({ &mut first }{ &mut second });

    outer.handle_post_render();

    assert_eq!(*post_render_3.borrow(), vec!["first", "second"]);
  }
}
