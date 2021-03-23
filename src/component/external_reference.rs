use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[serde(rename = "reference")]
#[yaserde(rename = "reference")]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct ExternalReference {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    pub ref_type: ExternalReferenceType,

    #[yaserde(prefix = "ns")]
    pub url: String,
    #[yaserde(prefix = "ns")]
    pub comment: Option<String>,
}

impl ExternalReference {
    pub fn new(
        ref_type: ExternalReferenceType,
        url: String,
        comment: Option<String>,
    ) -> ExternalReference {
        ExternalReference {
            ref_type,
            url,
            comment,
        }
    }
}
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExternalReferenceType {
    #[yaserde(rename = "vcs")]
    Vcs,
    #[yaserde(rename = "issue-tracker")]
    IssueTracker,
    #[yaserde(rename = "website")]
    Website,
    #[yaserde(rename = "advisories")]
    Advisories,
    #[yaserde(rename = "bom")]
    Bom,
    #[yaserde(rename = "mailing-list")]
    MailingList,
    #[yaserde(rename = "social")]
    Social,
    #[yaserde(rename = "chat")]
    Chat,
    #[yaserde(rename = "documentation")]
    Documentation,
    #[yaserde(rename = "support")]
    Support,
    #[yaserde(rename = "distribution")]
    Distribution,
    #[yaserde(rename = "license")]
    License,
    #[yaserde(rename = "build-metadata")]
    BuildMeta,
    #[yaserde(rename = "build-system")]
    BuildSystem,
    #[yaserde(rename = "other")]
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
            Option::from("All component versions are documented here".to_string()),
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
