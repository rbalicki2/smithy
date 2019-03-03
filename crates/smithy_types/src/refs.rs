use web_sys::HtmlElement;

pub type DomRef = Option<HtmlElement>;

pub type DomRefWithPath<'a> = (Vec<usize>, &'a mut DomRef);
