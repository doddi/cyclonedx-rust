use heck::KebabCase;
use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::writer::XmlEvent;
use yaserde::ser::Serializer;
use yaserde::YaSerialize;
use yaserde_derive::YaSerialize;

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
#[serde(rename = "reference")]
#[yaserde(rename = "reference")]
pub struct ExternalReference {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    ref_type: ExternalReferenceType,

    url: String,
    comment: String,
}

impl ExternalReference {
    pub fn new(ref_type: ExternalReferenceType, url: String, comment: String) -> ExternalReference {
        ExternalReference {
            ref_type,
            url,
            comment,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize)]
pub enum ExternalReferenceType {
    Vcs,
    IssueTracker,
    Website,
    Advisories,
    Bom,
    MailingList,
    Social,
    Chat,
    Documentation,
    Support,
    Distribution,
    License,
    BuildMeta,
    BuildSystem,
    Other,
}

impl Display for ExternalReferenceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl YaSerialize for ExternalReferenceType {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        let data = self.to_string();
        writer.write(XmlEvent::characters(&data.to_kebab_case()));
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaserde::ser::Config;

    #[test]
    fn print_xml() {
        let expected = r#"<reference type="documentation"><url>http://example.org/docs</url><comment>All component versions are documented here</comment></reference>"#;

        let model = ExternalReference::new(
            ExternalReferenceType::Documentation,
            "http://example.org/docs".to_string(),
            "All component versions are documented here".to_string(),
        );
        let actual = yaserde::ser::to_string_with_config(
            &model,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
