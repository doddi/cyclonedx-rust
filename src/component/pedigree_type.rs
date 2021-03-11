use crate::common::attached_text::AttachedTextType;
use crate::component::Component;
use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct PedigreeType {
    ancestors: Vec<Component>,
    descendants: Vec<Component>,
    variants: Vec<Component>,
    commits: Vec<CommitType>,
    patches: Vec<PatchType>,
    notes: Option<String>,
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct PatchType {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    patchtype_type: BomPatchClassification,

    diff: Option<DiffType>,
    resolves: Vec<IssueType>,
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct DiffType {
    text: Option<AttachedTextType>,
    url: Option<String>,
}

impl DiffType {
    pub fn new(text: Option<AttachedTextType>, url: Option<String>) {
        DiffType { text, url };
    }
}

#[derive(Clone, Default, Builder, Debug, PartialEq, Serialize, YaSerialize, YaDeserialize)]
pub struct IssueType {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    issue_type: BomIssueClassification,

    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    source: Option<Source>,
    references: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct Source {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub enum BomIssueClassification {
    Detect,
    Enhancement,
    Security,
}

impl Default for BomIssueClassification {
    fn default() -> Self {
        BomIssueClassification::Detect
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub enum BomPatchClassification {
    Unofficial,
    Monkey,
    Backport,
    CherryPick,
}

impl Default for BomPatchClassification {
    fn default() -> Self {
        BomPatchClassification::Backport
    }
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct CommitType {
    uid: Option<String>,
    url: Option<String>,
    author: Option<IdentifiableActionType>,
    committer: Option<IdentifiableActionType>,
    message: Option<String>,
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct IdentifiableActionType {
    timestamp: Option<String>,
    name: Option<String>,
    email: Option<String>,
}

impl IdentifiableActionType {
    pub fn new(
        timestamp: Option<String>,
        name: Option<String>,
        email: Option<String>,
    ) -> IdentifiableActionType {
        IdentifiableActionType {
            timestamp,
            name,
            email,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::license::*;
    use crate::component::classification::Classification;
    use crate::component::pedigree_type::PedigreeType;
    use crate::component::*;
    use yaserde::ser::Config;

    #[test]
    pub fn print_xml() {
        let expected: PedigreeType = PedigreeTypeBuilder::default()
            .ancestors(vec![ComponentBuilder::default()
                .component_type(Classification::Application)
                .mime_type(None)
                .bom_ref(None)
                .supplier(None)
                .author(None)
                .publisher(Option::from("Apache".to_string()))
                .group(Option::from("org.apache.tomcat".to_string()))
                .name(Option::from("tomcat-catalina".to_string()))
                .version(Option::from("9.0.14".to_string()))
                .description(Option::from("Apache Catalina".to_string()))
                .scope(None)
                .hashes(Vec::new())
                .licenses(vec![LicensesBuilder::default()
                    .license(vec![LicenseTypeBuilder::default()
                        .id(Option::from("Apache-2.0".to_string()))
                        .name(None)
                        .text(None)
                        .url(None)
                        .build()
                        .unwrap()])
                    .expression(None)
                    .build()
                    .unwrap()])
                .copyright(None)
                .purl(Option::from(
                    "pkg:maven/org.apache.tomcat/tomcat-catalina@9.0.14?packaging=jar".to_string(),
                ))
                .swid(None)
                .modified(None)
                .pedigree(None)
                .external_references(Vec::new())
                .components(Vec::new())
                .build()
                .unwrap()])
            .descendants(Vec::new())
            .variants(Vec::new())
            .commits(vec![CommitTypeBuilder::default()
                .uid(Option::from(
                    "7638417db6d59f3c431d3e1f261cc637155684cd".to_string(),
                ))
                .url(Option::from(
                    "https://location/to/7638417db6d59f3c431d3e1f261cc637155684cd".to_string(),
                ))
                .author(Option::from(IdentifiableActionType::new(
                    Option::from("2018-11-07T22:01:45Z".to_string()),
                    Option::from("John Doe".to_string()),
                    Option::from("john.doe@example.com".to_string()),
                )))
                .committer(None)
                .message(None)
                .build()
                .unwrap()])
            .patches(Vec::new())
            .notes(Option::from("Commentary here".to_string()))
            .build()
            .unwrap();

        let parsed = yaserde::ser::to_string_with_config(
            &expected,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        let actual: PedigreeType = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
