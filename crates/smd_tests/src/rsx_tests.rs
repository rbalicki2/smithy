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
  let mut result = rsx![
    (div {} {
      on_click: { |_| println!("ASFASFDFDA") }
    })
  ];

  use smithy::types::Component as _;
  let component = result.render();
  println!("rendered {:?}", component);
  // rsx![(div[1 + 3, 4+6])];
}
