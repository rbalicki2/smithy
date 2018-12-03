# Smithy

> Smithy is a front-end framework for Rust

## TODO

* Change Vec<SmithyComponent> to a struct with a private field,
  since we never actually want to iterate over all of the values
* impl Component for &str, Option, etc
* tests
* derive eq etc on Nodes
* rational naming
* does having Node::Vec(Vec<Node>) make sense?
* use many_0_custom in the outer smd! macro
* derive variants for the events enum

