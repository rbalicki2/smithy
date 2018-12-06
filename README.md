# Smithy

> Smithy is a front-end framework for Rust

## TODO

x Change Vec<SmithyComponent> to a struct with a private field,
  since we never actually want to iterate over all of the values
  x false. remove Vec<SmithyComponents> entirely, preferring a single
    one
  x does having Node::Vec(Vec<Node>) make sense?
* impl Component for &str, Option, etc
  * consider implementing Component instead of Into<Node>...?
x tests
x derive eq etc on Nodes
* rational naming
* use many_0_custom in the outer smd! macro
* derive variants for the events enum
* organize types/core
* figure out why smd!(<div />) does not compile
* EVENTS <- figure out how to take a DOM event and turn it (statically) into a regular event

## Think about how to re-render on hash change
* An individual event should be handled only once and by one thing, so hashchange etc. *cannot* happen through the regular event handling mechanism.
* OTOH, the results of an smd! macro have a mutable reference to state, so you probably need to wrap your state in an Rc<RefCell<T>> to do this.
* But maybe not...?

```rs
let inner = smd!(<div on_click={...} />);
// will inner need to be inlined here? ... probably not
let outer = on_hash_change!(inner, |_| app_state.hash = get_hash())

let inner = |p| { match p { Phase::Rendering => ..., Phase::EventHandling => ... }}
let outer = HashChangeEventHandler(|p| {
  match p {
    Phase::WindowEventHandling(evt) => {
      match evt {
        WindowEvent::HashChange(e) => {
          (|_| app_state.hash = get_hash())(e);
          inner.handle_event(evt);
          PhaseResult::WindowEventHandling
        }
      }
    },
    x => inner(x)
  },
})
```

* this will be hard
* Think about what the best API is on the outside, whether it needs to be macro, etc.

## Think about how to handle props... the wrong way

* `Component` needs to be `Component<Props>`

```rs
pub trait Component<Props> {
  fn handle_event(&mut self, p: Props, _event: &crate::Event, _path: &Path) -> EventHandled {
    false
  }
  fn render(&mut self, p: Props) -> Node;
}

pub struct SmithyComponent<Props>(pub Box<FnMut(Phase, Props) -> PhaseResult>);

impl<Props> SmithyComponent<Props> {
  pub fn bind(&mut self, p: Props) -> SmithyComponent<()> {
    |phase, ()| self(phase, p)
  }
}

let mut app = smd!(props: AppState, <div>{ props.this_was_magical }</div>);

smd!(
  <div>{ &mut app.bind_props(props) }</div>
)
// equivalent to
smd!((), <div>{ ... }</div>)

// which becomes
let mut a = {
  use smithy::types as smithy_types;
  let component: smithy_types::SmithyComponent =
    smithy_types::SmithyComponent(Box::new(move |phase, props: P| match phase {
      smithy_types::Phase::Rendering => {
        smithy_types::PhaseResult::Rendering(smithy::types::Node::Vec(vec![
          smithy_types::Node::Dom(smithy_types::HtmlToken {
            node_type: "div".into(),
            attributes: std::collections::HashMap::new(),
            children: vec![],
          }),
        ]))
      },
      smithy_types::Phase::EventHandling(event_handling) => match event_handling {
        (smithy_types::Event::OnClick(val), [0usize]) => {
          ({ |_| {} })(val);
          smithy_types::PhaseResult::EventHandling(true)
        },
        _ => smithy_types::PhaseResult::EventHandling(false),
      },
    }));
  component
};
```

* That whole "props" thing smells.

## This is how to handle props, the right way

```rs
fn foo(any_props: u32, more_props: u32) -> SmithyComponent {
  smd!(
    <div>whatever {any_props}</div>
  )
}
```
* now lifetimes... :(

* Simple, done, already implemented, no need for extra stuff

## Smithy core should not know about SmithyComponent

* Improve the naming
* CoreComponent? Maybe smithy could be the name of the macro + types, and Core or something could be the name of the engine?