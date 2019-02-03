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
* file size
* confirm <input> works reasonably
x render-in-progress flag
* convert existing comments to /// comments
* prelude
x self closing tags
x attribute values should be enclosed in quotes

## Blockers to alpha
* compilation bugs, like `smd!()` `smd!(<div />` etc
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
