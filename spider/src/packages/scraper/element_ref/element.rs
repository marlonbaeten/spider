use fast_html5ever::Namespace;
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::matching;
use selectors::{Element, OpaqueElement};

use super::super::selector::{CssLocalName, CssString, NonTSPseudoClass, PseudoElement, Simple};
use super::ElementRef;

/// Note: will never match against non-tree-structure pseudo-classes.
impl<'a> Element for ElementRef<'a> {
    type Impl = Simple;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.node.value())
    }

    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(ElementRef::wrap)
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn first_element_child(&self) -> Option<Self> {
        self.prev_siblings().nth(0).and_then(ElementRef::wrap)
    }

    fn apply_selector_flags(&self, _: selectors::matching::ElementSelectorFlags) {
        // Apply selector flags when enabled
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn is_part(&self, _name: &CssLocalName) -> bool {
        false
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.value().name == other.value().name
    }

    fn imported_part(&self, _: &CssLocalName) -> Option<CssLocalName> {
        None
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.value().name.ns == ns!(html)
    }

    fn has_local_name(&self, name: &CssLocalName) -> bool {
        self.value().name.local == name.0
    }

    fn has_namespace(&self, namespace: &Namespace) -> bool {
        &self.value().name.ns == namespace
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        self.value().attrs.iter().any(|(key, value)| {
            !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns)
                && local_name.0 == key.local
                && operation.eval_str(value)
        })
    }

    fn match_non_ts_pseudo_class(
        &self,
        _pc: &NonTSPseudoClass,
        _context: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn is_link(&self) -> bool {
        self.value().name() == "link"
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(&self, id: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        match self.value().id {
            Some(ref val) => case_sensitivity.eq(id.0.as_bytes(), val.as_bytes()),
            None => false,
        }
    }

    fn has_class(&self, name: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.value().has_class(&name.0, case_sensitivity)
    }

    fn is_empty(&self) -> bool {
        !self
            .children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.value().is_document())
    }
}

#[cfg(test)]
mod tests {
    use crate::packages::scraper::html::Html;
    use crate::packages::scraper::selector::{CssLocalName, Selector};
    use selectors::attr::CaseSensitivity;
    use selectors::Element;

    #[test]
    fn test_has_id() {
        let html = "<p id='link_id_456'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();

        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            true,
            element.has_id(
                &CssLocalName::from("link_id_456"),
                CaseSensitivity::CaseSensitive
            )
        );

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            false,
            element.has_id(
                &CssLocalName::from("any_link_id"),
                CaseSensitivity::CaseSensitive
            )
        );
    }

    #[test]
    fn test_is_link() {
        let html = "<link href='https://www.example.com'>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("link").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(true, element.is_link());

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(false, element.is_link());
    }

    #[test]
    fn test_has_class() {
        let html = "<p class='my_class'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            true,
            element.has_class(
                &CssLocalName::from("my_class"),
                CaseSensitivity::CaseSensitive
            )
        );

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            false,
            element.has_class(
                &CssLocalName::from("my_class"),
                CaseSensitivity::CaseSensitive
            )
        );
    }
}
