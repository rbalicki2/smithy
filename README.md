# Smithy

> [Smithy](https://www.smithy.rs) is a front-end framework for Rust

[Home page](https://www.smithy.rs) ◇ [API Docs](https://docs.smithy.rs/smithy/) ◇ [Repository](https://github.com/rbalicki2/smithy) ◇ [Example Sites](https://www.smithy.rs/examples)

## What is Smithy?

Smithy is a framework for writing WebAssembly applications entirely in Rust. Its goal is to allow you to do so using idiomatic Rust, without giving up any of the compiler's safety guarantees.

## Smithy works on nightly

Smithy v0.0.7 currently works on `1.39.0-nightly (dfd43f0fd 2019-09-01)`.

## Getting started

Getting started in Smithy is easy!

```sh
npm init smithy-app my_smithy_app
cd my_smithy_app
npm start
```

Navigate to `localhost:8080` to see your app in action!

See [the create-smithy-app repository](https://github.com/rbalicki2/create-smithy-app/) for more details.

## A simple Smithy app

A simple click counter is as follows:

```rust
#[wasm_bindgen(start)]
pub fn start() -> Result<(), wasm_bindgen::JsValue> {
  let root_element = get_root_element()?;
  let mut count = 0;
  let app = smithy::smd!(
    <div on_click={|_| count = count + 1}>
      I have been clicked {count}{' '}times.
    </div>
  );
  smithy::mount(Box::new(app), root_element);
  Ok(())
}

fn get_root_element() -> Result<web_sys::Element, wasm_bindgen::JsValue> {
  let document = web_sys::window().unwrap().document().unwrap();
  document.get_element_by_id("app")
    .ok_or(wasm_bindgen::JsValue::NULL)
}
```

## How Smithy works

### `smd!` macro

The `smd!` and `smd_borrowed!` macros convert something that looks like JSX into a wrapper around an `FnMut(smithy::types::Phase) -> smithy::types::PhaseResult`. For example, the `smd!` call in:

```rust
let mut count = 0;
let app = smithy::smd!(
  <div on_click={|_| count = count + 1}>
    I have been clicked {count}{' '}times.
  </div>
);
```

is converted into

```rust
let mut app = {
  #[allow(dead_code)]
  use smithy::types::Component;
  let component: smithy::types::SmithyComponent =
    smithy::types::SmithyComponent(Box::new(move |phase| match phase {
      smithy::types::Phase::Rendering => {
        smithy::types::PhaseResult::Rendering(smithy::types::Node::Vec(vec![
          smithy::types::Node::Dom(smithy::types::HtmlToken {
            node_type: "div".into(),
            attributes: std::collections::HashMap::new(),
            children: {
              let mut children = Vec::with_capacity(4usize);
              children.push(smithy::types::Node::Text("I have been clicked ".into()));
              children.push({ count }.render());
              children.push({ ' ' }.render());
              children.push(smithy::types::Node::Text("times.".into()));
              children
            },
          }),
        ]))
      },
      smithy::types::Phase::UiEventHandling(ui_event_handling) => match ui_event_handling {
        (evt, [0usize, 1usize, rest @ ..]) => {
          smithy::types::PhaseResult::UiEventHandling({ count }.handle_ui_event(evt, rest))
        },
        (evt, [0usize, 2usize, rest @ ..]) => {
          smithy::types::PhaseResult::UiEventHandling({ ' ' }.handle_ui_event(evt, rest))
        },
        (smithy::types::UiEvent::OnClick(val), [0usize, rest @ ..]) => {
          ({ |_| count = count + 1 })(val);
          smithy::types::PhaseResult::UiEventHandling(true)
        },
        _ => smithy::types::PhaseResult::UiEventHandling(false),
      },
      smithy::types::Phase::WindowEventHandling(window_event) => {
        let mut event_handled = false;
        event_handled = ({ count }).handle_window_event(window_event) || event_handled;
        event_handled = ({ ' ' }).handle_window_event(window_event) || event_handled;
        match window_event {
          _ => smithy::types::PhaseResult::WindowEventHandling(event_handled),
        }
      },
      smithy::types::Phase::PostRendering => {
        {
          {
            ({ count }).handle_post_render();
          }
          ({ ' ' }).handle_post_render();
        }
        smithy::types::PhaseResult::PostRendering
      },
      smithy::types::Phase::RefAssignment(path_so_far) => {
        let new_path = path_so_far
          .clone()
          .into_iter()
          .chain(vec![0usize, 1usize])
          .collect();
        ({ count }).handle_ref_assignment(new_path);
        let new_path = path_so_far
          .clone()
          .into_iter()
          .chain(vec![0usize, 2usize])
          .collect();
        ({ ' ' }).handle_ref_assignment(new_path);
        smithy::types::PhaseResult::RefAssignment
      },
    }));
  component
};
```

Notice that the `|_| count = count + 1` and `{count}` are in separate branches of the match arm. If they had not been (e.g. if `smd!` created a struct instead of an `FnMut`), this would not have compiled. The borrow checker would have complained that you cannot immutably borrow `count`, as it is already borrowed mutably in the `on_click` callback.

### Smithy phases

As you can see from the expansion of the `smd!` macro above, phases are a core concept in Smithy. In particular, an app is driven through five phases:

* rendering, in which the app is asked to return a struct containing the information about what it will write to the DOM.
* ref assignment, in which any app with `ref={&mut optional_web_sys_html_element}` will have `Some(some_html_element)` assigned to that ref.
* post rendering, in which any `post_render={|_| ...}` callbacks will be executed. These callbacks are guaranteed to have all refs already assigned, thus allowing you to do any direct DOM manipulation you need to do.
* UI event handling and window event handling, in which Smithy executes callbacks in response to events. After a callback is executed, Smithy will re-run the app through the different phases.
  * (UI event handling and window event handling are treated as separate phases, though conceptually they are very similar.)

## `smd!` vs `smd_borrowed!`

As you can see in the macro expansion above, the `smd!` macro creates a move closure. This is not always desirable. If you do not wish to create a move closure, use `smd_borrowed!` instead.

## How to get involved

Smithy is always looking for contributors! Please tweet at me `@statisticsftw` or take a look at the [Smithy roadmap](https://github.com/rbalicki2/smithy/issues/2).

In addition, please take Smithy out for a spin using [create-smithy-app](https://github.com/rbalicki2/create-smithy-app/).

Thanks! Happy coding!
