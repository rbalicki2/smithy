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
