use std::rc::Rc;

use web_sys::Element;

#[derive(PartialEq, Clone)]
pub struct ElementId(Rc<Element>);

impl std::fmt::Debug for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}/>", self.as_ref().tag_name())
    }
}

impl ElementId {
    pub fn from_element(element: impl Into<Element>) -> Self {
        Self::new(element.into())
    }
    pub fn new(element: Element) -> Self {
        Self(Rc::new(element))
    }
}

impl AsRef<Element> for ElementId {
    fn as_ref(&self) -> &Element {
        self.0.as_ref()
    }
}
