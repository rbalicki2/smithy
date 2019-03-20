# Smithy

> Smithy is a front-end framework for Rust

## TODO

* impl Component for &str, Option, etc
  * consider implementing Component instead of Into<Node>...?
x tests
  * more tests
x derive eq etc on Nodes
* rational naming
* use many_0_custom in the outer smd! macro
* derive variants for the events enum
* organize types/core
* figure out why smd!(<div />) does not compile
* Consider splitting core types (like Component) from smd-related types (like SmithyComponent, Path ?, etc)
* Add everything behind a feature flag for minimal file size
* Parametrize smithy wrt types of events, etc.
* get rid of closure.forget() where it exists

## Blockers to pre-alpha/blog post
x macros for event types
? `impl Component` for a bunch of things
x reconciliation algorithm + diff (sounds important!!)
x confirm <input> works reasonably
  * do more?
x render-in-progress flag
* convert existing comments to /// comments
* prelude
x self closing tags
x attribute values should be enclosed in quotes
* remaining events
* file size
* build:prod and deployment

## Sites to build
* <input> demo
* routing
* API calls without race conditions

## Blockers to alpha
* compilation bugs, like `smd!()` `smd!(<div />)` etc
* better compilation error messages
* Do not use thread_local!, instantiate things in mount
  * Maybe I need to call (reactor, mount) = initiaite_smithy();
* Spacing errors
* Allow for multiple instances of smithy
* Do not use format! in smithy
* Do not define so many wasm_bindgen'ed methods, instead dynamically cast a JsValue

## Non-blockers
* more tests
* organize types and separate true core from SmithyComponent implementation
* variable names are non-opaque :(
* use a Trait for DomRef

## Issues
* Think about how to handle match statements, e.g. in the context of routing
* Perhaps angular-like ng-if's?

## Making smithy generic
* smd! returns a SmithyComponent
* SmithyComponent implements Component<SmithyDom>
* SmithyDom implements Into<WebDom>
* SmithyDom and WebDom can all be replaced with e.g. NativeDom or whatever
* Into<WebDom> is where we need to put things like "h1 can't be self closing" etc.
* WebDom implements Diffable, etc.

## Lifecycle events
* Start by adding a post_render={Fn(Vec<HtmlElement>)}
* Handled in the same way as window event handlers
* Enum of WindowEventInfo | LifeCycleEventInfo
* cannot be a NodeList, needs to be a Vec<Node>, since { &mut first }{ &mut second }
  need to each receive disjoint nodes

## GetterSetter
* way to put a single piece of state in a wrapper, so that it can be used
  in a sub-component
* since we can't pass `(state, |new_state| state = new_state)` as params to an input,
  but we can do `let state_wrapped = GetterSetter::wrap(state); render(state_wrapped)`

## TODO wrt attribute or event handlers
* SplitAttributeOrEventHandlers should also contain a ref
* It should be passed down in core.rs in smd_macro
* TokenStreamEventHandlingInfoPair needs to have a dom_ref_opt in there

## Bugs
* No need for `"keyboard-events"` feature?
