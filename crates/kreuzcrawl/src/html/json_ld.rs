//! JSON-LD structured data extraction from HTML documents.

use tl::VDom;

use crate::types::JsonLdEntry;

use super::selectors::SEL_JSON_LD;

/// Extract JSON-LD structured data entries from a parsed HTML document.
pub(crate) fn extract_json_ld(dom: &VDom<'_>) -> Vec<JsonLdEntry> {
    let parser = dom.parser();
    let mut entries = Vec::new();

    if let Some(iter) = dom.query_selector(SEL_JSON_LD) {
        for handle in iter {
            if let Some(node) = handle.get(parser) {
                let raw = node.inner_text(parser).to_string();
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
                    let schema_type = val.get("@type").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                    let name = val.get("name").and_then(|v| v.as_str()).map(String::from);
                    entries.push(JsonLdEntry { schema_type, name, raw });
                }
            }
        }
    }
    entries
}
