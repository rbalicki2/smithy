#[test]
fn basic_rsx_tests() {
  use rsx_macro::rsx;

  // let app = rsx![(div { readonly, ...foo } {})];
  // let app = rsx![({ div })];
  // rsx![
  //   ({ "div" }
  //     { foo: "bar", another_attribute, {attribute_name}: value }
  //     [
  //       (div),
  //       {component}
  //     ]
  //   )
  // ];
  // rsx![(div {} {})];
  // rsx![(div { {attr}})];
  // rsx![(div { {attr}: true })];
  // let mut other_attributes = std::collections::HashMap::new();
  // other_attributes.insert("A", "B");
  // let attr_key = "key";
  // let attr_val = "val";

  let cb = |_: &bool| println!("byahhhh");
  let cb2 = |_: &bool| println!("byahhhh");
  let mut result = rsx![
    (div {} {
      on_test: cb
    } [(inner {} {
      on_test: cb2
    })]),
    (span {} { on_test: cb })
  ];

  use smithy::types::Component as _;
  let component = result.render();
  println!("rendered {:?}", component);
  // rsx![(div[1 + 3, 4+6])];

  // ----------------------------------------------------------------
  // What to work on next?
  // TODO's in convert_to_components
  // Handle interpolated event handlers (currently unimplemented)
  // Remove Vec<Node> from Node or allow render() to return Vec<Node>
  // Get events to be generic
  // Test on actual site (paths will probably not align)
  // ----------------------------------------------------------------
}
