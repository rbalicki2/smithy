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
* Consider splitting core types (like Component) from smd-related types (like SmithyComponent, Path ?, etc)
* Add everything behind a feature flag for minimal file size
* Parametrize smithy wrt types of events, etc.
x on_hash_change and the like

## Blockers to alpha
* macros for event types
* `impl Component` for a bunch of things
* compilation bugs, like `smd!()` `smd!(<div />` etc
* setTimeout, etc.
* better compilation error messages
* Do not use thread_local!, instantiate things in mount
  * Maybe I need to call (reactor, mount) = initiaite_smithy();

## Non-blockers
* more tests
* organize types and separate true core from SmithyComponent implementation

## Match statements

* aka sub-components... ruh roh
