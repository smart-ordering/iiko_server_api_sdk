use std::collections::BTreeMap;

use quick_xml::{Reader, events::Event};
use serde::{Deserialize, Serialize};

use crate::error::{IikoError, Result};

/// Loss-tolerant representation of one internal iiko XML value.
///
/// Internal DTOs change between iiko releases. The SDK preserves element names, attributes,
/// text and repeated children instead of pretending the undocumented payload is a stable Rust
/// struct. Requests remain strongly typed and method-specific.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalXmlNode {
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attributes: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<InternalXmlNode>,
}

impl InternalXmlNode {
    pub fn child(&self, name: &str) -> Option<&InternalXmlNode> {
        self.children.iter().find(|child| child.name == name)
    }

    pub fn children_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a InternalXmlNode> + 'a {
        self.children.iter().filter(move |child| child.name == name)
    }

    pub fn class_name(&self) -> Option<&str> {
        self.attributes.get("cls").map(String::as_str)
    }

    pub fn is_null(&self) -> bool {
        self.attributes
            .get("null")
            .is_some_and(|value| value == "1")
    }

    fn recursive_text(&self, output: &mut String) {
        if let Some(text) = &self.text {
            if !output.is_empty() {
                output.push(' ');
            }
            output.push_str(text);
        }
        for child in &self.children {
            child.recursive_text(output);
        }
    }
}

/// Successful `ServerResult` envelope returned by one allowlisted internal read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalReadResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<InternalXmlNode>,
}

pub(crate) fn parse_internal_server_result(xml: &str) -> Result<InternalReadResult> {
    let root = parse_xml_tree(xml)?;
    if root.name != "result" {
        return Err(IikoError::XmlParsing(
            "internal response root must be result".to_string(),
        ));
    }

    let status = root
        .child("status")
        .and_then(|node| node.text.as_deref())
        .unwrap_or_default();
    if status != "SUCCESS" {
        let mut message = String::new();
        if let Some(error) = root
            .child("errorsContainer")
            .and_then(|node| node.child("rootError"))
        {
            error.recursive_text(&mut message);
        }
        let message = normalize_error_message(&message);
        return Err(IikoError::Api(if message.is_empty() {
            format!("iiko internal service returned {status}")
        } else {
            message
        }));
    }

    let correlation_id = root
        .child("correlationId")
        .and_then(|node| node.text.clone());
    let value = root
        .children
        .into_iter()
        .find(|node| node.name == "resultValue")
        .filter(|node| !node.is_null())
        .and_then(|node| node.children.into_iter().next());
    Ok(InternalReadResult {
        correlation_id,
        value,
    })
}

fn parse_xml_tree(xml: &str) -> Result<InternalXmlNode> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut stack = Vec::<InternalXmlNode>::new();
    let mut root = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(event)) => stack.push(node_from_start(&reader, &event)?),
            Ok(Event::Empty(event)) => {
                attach_node(node_from_start(&reader, &event)?, &mut stack, &mut root)?
            }
            Ok(Event::Text(event)) => {
                if let Some(node) = stack.last_mut() {
                    let text = event
                        .xml_content()
                        .map_err(|error| IikoError::XmlParsing(error.to_string()))?;
                    append_text(node, text.as_ref());
                }
            }
            Ok(Event::CData(event)) => {
                if let Some(node) = stack.last_mut() {
                    let text = event
                        .decode()
                        .map_err(|error| IikoError::XmlParsing(error.to_string()))?;
                    append_text(node, text.as_ref());
                }
            }
            Ok(Event::GeneralRef(event)) => {
                if let Some(node) = stack.last_mut() {
                    if let Some(character) = event
                        .resolve_char_ref()
                        .map_err(|error| IikoError::XmlParsing(error.to_string()))?
                    {
                        append_text(node, &character.to_string());
                    } else {
                        let name = event
                            .decode()
                            .map_err(|error| IikoError::XmlParsing(error.to_string()))?;
                        let value = quick_xml::escape::resolve_xml_entity(name.as_ref())
                            .ok_or_else(|| {
                                IikoError::XmlParsing(format!("unrecognized XML entity {name}"))
                            })?;
                        append_text(node, value);
                    }
                }
            }
            Ok(Event::End(_)) => {
                let node = stack.pop().ok_or_else(|| {
                    IikoError::XmlParsing("internal response has an unmatched end tag".to_string())
                })?;
                attach_node(node, &mut stack, &mut root)?;
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(error) => return Err(IikoError::XmlParsing(error.to_string())),
        }
    }

    if !stack.is_empty() {
        return Err(IikoError::XmlParsing(
            "internal response has an unclosed element".to_string(),
        ));
    }
    root.ok_or_else(|| IikoError::XmlParsing("internal response is empty".to_string()))
}

fn node_from_start(
    reader: &Reader<&[u8]>,
    event: &quick_xml::events::BytesStart<'_>,
) -> Result<InternalXmlNode> {
    let name = String::from_utf8_lossy(event.name().as_ref()).into_owned();
    let mut attributes = BTreeMap::new();
    for attribute in event.attributes().with_checks(false) {
        let attribute = attribute.map_err(|error| IikoError::XmlParsing(error.to_string()))?;
        let key = String::from_utf8_lossy(attribute.key.as_ref()).into_owned();
        let value = attribute
            .decode_and_unescape_value(reader.decoder())
            .map_err(|error| IikoError::XmlParsing(error.to_string()))?;
        attributes.insert(key, value.into_owned());
    }
    Ok(InternalXmlNode {
        name,
        attributes,
        text: None,
        children: Vec::new(),
    })
}

fn attach_node(
    node: InternalXmlNode,
    stack: &mut [InternalXmlNode],
    root: &mut Option<InternalXmlNode>,
) -> Result<()> {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(node);
        return Ok(());
    }
    if root.replace(node).is_some() {
        return Err(IikoError::XmlParsing(
            "internal response contains multiple root elements".to_string(),
        ));
    }
    Ok(())
}

fn append_text(node: &mut InternalXmlNode, text: &str) {
    if text.trim().is_empty() {
        return;
    }
    match &mut node.text {
        Some(existing) => existing.push_str(text),
        None => node.text = Some(text.to_string()),
    }
}

fn normalize_error_message(message: &str) -> String {
    let normalized = message.split_whitespace().collect::<Vec<_>>().join(" ");
    normalized.chars().take(1_024).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_unknown_fields_attributes_and_repeated_items() {
        let xml = r#"<result cls="ServerResult"><status>SUCCESS</status><correlationId>corr-1</correlationId><resultValue><r cls="java.util.ArrayList"><i cls="FutureDto" eid="111"><name>A &amp; B</name><unknown null="1"/><tags><i>x</i><i>y</i></tags></i></r></resultValue><errorsContainer><rootError null="1"/></errorsContainer></result>"#;
        let result = parse_internal_server_result(xml).unwrap();
        let value = result.value.unwrap();
        assert_eq!(result.correlation_id.as_deref(), Some("corr-1"));
        assert_eq!(value.class_name(), Some("java.util.ArrayList"));
        let item = value.child("i").unwrap();
        assert_eq!(item.attributes.get("eid").map(String::as_str), Some("111"));
        assert_eq!(item.child("name").unwrap().text.as_deref(), Some("A & B"));
        assert!(item.child("unknown").unwrap().is_null());
        assert_eq!(
            item.child("tags")
                .unwrap()
                .children_named("i")
                .filter_map(|node| node.text.as_deref())
                .collect::<Vec<_>>(),
            vec!["x", "y"]
        );
    }

    #[test]
    fn rejects_server_result_failure_and_bounds_error_text() {
        let xml = format!(
            "<result><status>SYSTEM_ERROR</status><errorsContainer><rootError><message>{}</message></rootError></errorsContainer></result>",
            "x".repeat(2_000)
        );
        let error = parse_internal_server_result(&xml).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("API error"));
        assert!(message.len() < 1_100);
    }

    #[test]
    fn rejects_malformed_xml() {
        assert!(parse_internal_server_result("<result><status>SUCCESS").is_err());
    }
}
