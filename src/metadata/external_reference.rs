use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
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
#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
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

impl Default for ExternalReferenceType {
    fn default() -> Self {
        ExternalReferenceType::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaserde::ser::Config;

    #[test]
    fn print_xml() {
        let expected = ExternalReference::new(
            ExternalReferenceType::Documentation,
            "http://example.org/docs".to_string(),
            "All component versions are documented here".to_string(),
        );
        let parsed = yaserde::ser::to_string_with_config(
            &expected,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        let actual: ExternalReference = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
