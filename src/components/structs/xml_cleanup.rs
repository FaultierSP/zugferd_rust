pub(super) const REQUIRED_EMPTY_ELEMENT: &str = "<ram:ApplicableHeaderTradeDelivery";

/// Removes elements without content, both opening and closing tag
/// (`<a></a>` and `<a/>`), repeating until parents that became empty are gone too.
pub(super) fn remove_empty_elements(mut xml: String) -> String {
    loop {
        let mut changed = false;
        let mut pos = 0;
        while let Some(rel) = xml[pos..].find('<') {
            let start = pos + rel;
            let Some(end_rel) = xml[start..].find('>') else {
                break;
            };
            let end = start + end_rel + 1;
            let tag = &xml[start..end];
            if tag.starts_with("<?") || tag.starts_with("</") {
                pos = end;
                continue;
            }
            // Mandatory in the schema even when empty (MINIMUM profile).
            if tag.starts_with(REQUIRED_EMPTY_ELEMENT) {
                pos = end;
                continue;
            }
            if tag.ends_with("/>") {
                xml.replace_range(start..end, "");
                changed = true;
                pos = start;
                continue;
            }
            let name = tag[1..tag.len() - 1]
                .split_whitespace()
                .next()
                .unwrap_or("");
            let closing = format!("</{}>", name);
            if xml[end..].starts_with(&closing) {
                xml.replace_range(start..end + closing.len(), "");
                changed = true;
                pos = start;
            } else {
                pos = end;
            }
        }
        if !changed {
            return xml;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::remove_empty_elements;

    fn clean(xml: &str) -> String {
        remove_empty_elements(xml.to_string())
    }

    #[test]
    fn removes_empty_element_with_closing_tag() {
        assert_eq!(clean("<r><a></a><b>x</b></r>"), "<r><b>x</b></r>");
    }

    #[test]
    fn removes_self_closing_element() {
        assert_eq!(clean("<r><a/><b>x</b></r>"), "<r><b>x</b></r>");
        assert_eq!(clean("<r><a attr=\"1\"/><b>x</b></r>"), "<r><b>x</b></r>");
    }

    #[test]
    fn removes_empty_element_with_attributes() {
        assert_eq!(clean("<r><a id=\"1\"></a><b>x</b></r>"), "<r><b>x</b></r>");
    }

    #[test]
    fn removes_containers_that_become_empty() {
        assert_eq!(
            clean("<r><p><q><a></a></q></p><b>x</b></r>"),
            "<r><b>x</b></r>"
        );
        assert_eq!(clean("<r><p><q/></p></r>"), "");
    }

    #[test]
    fn keeps_containers_with_content() {
        let xml = "<r><p><q>1</q><a></a></p></r>";
        assert_eq!(clean(xml), "<r><p><q>1</q></p></r>");
    }

    #[test]
    fn keeps_declaration_and_required_empty_element() {
        let xml = "<?xml version='1.0'?><r><ram:ApplicableHeaderTradeDelivery></ram:ApplicableHeaderTradeDelivery><a></a></r>";
        assert_eq!(
            clean(xml),
            "<?xml version='1.0'?><r><ram:ApplicableHeaderTradeDelivery></ram:ApplicableHeaderTradeDelivery></r>"
        );
    }
}
