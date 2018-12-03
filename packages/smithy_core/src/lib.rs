use smithy_types::SmithyComponent;
use web_sys::Element;

pub fn mount(app: Vec<SmithyComponent>, el: Element) {
  el.set_inner_html("<b>foo</b>");
}
