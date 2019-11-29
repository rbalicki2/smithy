#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  let mut result = rsx![{ 123 }];

  use smithy::types::Component as _;
  let component = result.render();
  println!("rendered {:?}", component);

  // ----------------------------------------------------------------
  // What to work on next?
  // rsx equivalent of post_render! and with_ref!
  // global event handlers done similarly
  // TODO's in convert_to_components
  // Post merge
  // Remove Vec<Node> from Node or allow render() to return Vec<Node>
  // Get events to be generic
  // ----------------------------------------------------------------
}
