pub mod hash_type;
pub mod tool_type;

use std::fmt::{Display, Formatter};
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use serde::{Serialize};
use yaserde_derive::YaSerialize;
use std::fmt;
use derive_builder::{Builder};
use yaserde::YaSerialize;
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use yaserde::ser::Serializer;
use xml::writer::XmlEvent;
use crate::metadata::tool_type::ToolType;

#[derive(Debug, Serialize, YaSerialize)]
pub struct Metadata {
    time_stamp: String,
    tools: Vec<ToolType>,
    authors: Vec<OrganizationalContact>,
    component: Option<Component>,
    manufacturer: Vec<OrganizationalEntity>
}

impl Metadata {
    pub fn new(tools: Vec<ToolType>,
               authors: Vec<OrganizationalContact>,
               component: Option<Component>,
               manufacturer: Vec<OrganizationalEntity>
    ) -> Metadata {
        let time_stamp: DateTime<Utc> = SystemTime::now().into();
        Metadata {
            time_stamp: time_stamp.to_rfc3339(),
            tools, authors,
            component,
            manufacturer
        }
    }
}

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct OrganizationalEntity {
    name: Option<String>,
    url: Vec<String>,
    contact: Vec<OrganizationalContact>
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct OrganizationalContact {
    name: Option<String>,
    email: Vec<String>,
    phone: Vec<String>
}

impl OrganizationalContact {
    pub fn new(name: Option<String>, email: Vec<String>, phone: Vec<String>) -> OrganizationalContact {
        OrganizationalContact { name, email, phone }
    }
}

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct Component {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    component_type: Classification,

    #[serde(rename = "mime-type")]
    #[yaserde(rename = "mime-type", attribute)]
    mime_type: Option<String>,

    #[serde(rename = "bom-ref")]
    #[yaserde(rename = "bom-ref", attribute)]
    bom_ref: Option<String>,

    supplier: Option<OrganizationalEntity>,
    author: Option<String>,
    publisher: Option<String>,
    group: Option<String>,
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    scope: Option<Scope>,
    hashes: Vec<hash_type::HashType>,
    licenses: Vec<Licenses>,
    copyright: Option<String>,
    purl: Option<String>,
    swid: Option<SwidType>,
    modified: Option<bool>,
    pedigree: Option<PedigreeType>,
    external_references: Vec<ExternalReference>,
    components: Vec<Component>
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum Classification {
    Application, Framework, Library, Container, OperatingSystem, Device, Firmware, File
}

impl Display for Classification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Classification::Application => write!(f, "application"),
            Classification::Framework => write!(f, "framework"),
            Classification::Library => write!(f, "library"),
            Classification::Container => write!(f, "container"),
            Classification::OperatingSystem => write!(f, "operating-system"),
            Classification::Device => write!(f, "device"),
            Classification::Firmware => write!(f, "firmware"),
            Classification::File => write!(f, "file"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub struct ExternalReference {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    externalref_type: ExternalReferenceType,

    url: String,
    comment: String
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
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
    Other
}

impl Display for ExternalReferenceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ExternalReferenceType::Vcs => write!(f, "Vcs"),
            ExternalReferenceType::IssueTracker => write!(f, "issue-tracker"),
            ExternalReferenceType::Website => write!(f, "Website"),
            ExternalReferenceType::Advisories => write!(f, "Advisories"),
            ExternalReferenceType::Bom => write!(f, "Bom"),
            ExternalReferenceType::MailingList => write!(f, "mailing-list"),
            ExternalReferenceType::Social => write!(f, "Social"),
            ExternalReferenceType::Chat => write!(f, "Chat"),
            ExternalReferenceType::Documentation => write!(f, "Documentation"),
            ExternalReferenceType::Support => write!(f, "Support"),
            ExternalReferenceType::Distribution => write!(f, "Distribution"),
            ExternalReferenceType::License => write!(f, "License"),
            ExternalReferenceType::BuildMeta => write!(f, "build-meta"),
            ExternalReferenceType::BuildSystem => write!(f, "build-system"),
            ExternalReferenceType::Other => write!(f, "Other")
        }
    }
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct Licenses {
    license: Vec<LicenseType>,
    expression: Option<String>
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct LicenseType {
    id: Option<String>,
    name: Option<String>,
    text: Option<AttachedTextType>,
    url: Option<String>,
}

impl LicenseType {
    pub fn new(id: Option<String>, name: Option<String>, text: Option<AttachedTextType>, url: Option<String>) -> LicenseType {
        LicenseType { id, name, text, url }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum Scope {
    Required,
    Optional,
    Excluded
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Required => write!(f, "Required"),
            Scope::Optional => write!(f, "Optional"),
            Scope::Excluded => write!(f, "exluded")
        }
    }
}

// #[derive(Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
// pub struct Supplier {
//     name: String,
//     url: String,
//     contact: String
// }

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct SwidType {
    #[serde(rename = "tagId")]
    #[yaserde(rename = "tagId", attribute)]
    tag_id: String,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    version: Option<String>,

    #[serde(rename = "tagVersion")]
    #[yaserde(rename = "tagVersion", attribute)]
    tag_version: Option<i32>,

    #[yaserde(attribute)]
    patch: Option<bool>,

    text: Option<AttachedTextType>,
    url: Option<String>
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize)]
pub struct AttachedTextType {
    #[serde(rename = "content-type")]
    content_type: Option<String>,

    encoding: Option<BomEncoding>,

    value: String
}

impl YaSerialize for AttachedTextType {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        writer.write(XmlEvent::characters(self.value.as_str()));
        Ok(())
    }

    fn serialize_attributes(&self, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum BomEncoding {
    Base64
}

impl Display for BomEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BomEncoding::Base64 => write!(f, "base64")
        }
    }
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct PedigreeType {
    ancestors: Vec<Component>,
    descendants: Vec<Component>,
    variants: Vec<Component>,
    commits: Vec<CommitType>,
    patches: Vec<PatchType>,
    notes: Option<String>
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct PatchType {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    patchtype_type: BomPatchClassification,

    diff: Option<DiffType>,
    resolves: Vec<IssueType>
}

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct DiffType {
    text: Option<AttachedTextType>,
    url: Option<String>
}

impl DiffType {
    pub fn new(text: Option<AttachedTextType>, url: Option<String>) {
        DiffType {text, url };
    }
}

#[derive(Clone, Default, Builder, Debug, PartialEq, Serialize, YaSerialize)]
pub struct IssueType {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    issue_type: BomIssueClassification,

    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    source: Option<Source>,
    references: Vec<String>
}


#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub struct Source {
    name: Option<String>,
    url: Option<String>
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum BomIssueClassification {
    Detect, Enhancement, Security
}

impl Default for BomIssueClassification {
    fn default() -> Self {
        BomIssueClassification::Detect
    }
}

impl Display for BomIssueClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BomIssueClassification::Detect => write!(f, "detect"),
            BomIssueClassification::Enhancement => write!(f, "enhancement"),
            BomIssueClassification::Security => write!(f, "security"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum BomPatchClassification {
    Unofficial, Monkey, Backport, CherryPick
}

impl Default for BomPatchClassification {
    fn default() -> Self {
        BomPatchClassification::Backport
    }
}

impl Display for BomPatchClassification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BomPatchClassification::Unofficial => write!(f, "unofficial"),
            BomPatchClassification::Monkey => write!(f, "monkey"),
            BomPatchClassification::Backport => write!(f, "backport"),
            BomPatchClassification::CherryPick => write!(f, "cherry-pick")
        }
    }
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct CommitType {
    uid: Option<String>,
    url: Option<String>,
    author: Option<IdentifiableActionType>,
    committer: Option<IdentifiableActionType>,
    message: Option<String>
}

#[derive(Default, Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub struct IdentifiableActionType {
    timestamp: Option<String>,
    name: Option<String>,
    email: Option<String>
}

impl IdentifiableActionType {
    pub fn new(timestamp: Option<String>, name: Option<String>, email: Option<String>) -> IdentifiableActionType {
        IdentifiableActionType { timestamp, name, email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metadata::hash_type::HashType;
    use crate::metadata::hash_type::HashAlg::{Sha1, Sha256};
    use crate::metadata::tool_type::{ToolType, ToolTypeBuilder};

    #[test]
    fn tool_builder() {
        let mut tool_builder = ToolTypeBuilder::default();
        let tool: ToolType = tool_builder
            .name("name".to_string())
            .version("version".to_string())
            .vendor("vendor".to_string())
            .hashes([ HashType::new(Sha1, "1234567890".to_string()), HashType::new(Sha256, "0987654321".to_string())].to_vec())
            .build().unwrap();

        assert_eq!(tool.name, "name");
        assert_eq!(tool.version, "version");
        assert_eq!(tool.vendor, "vendor");
        let vec = tool.hashes;
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].alg, Sha1);
        assert_eq!(vec[0].value, "1234567890".to_string());
        assert_eq!(vec[1].alg, Sha256);
        assert_eq!(vec[1].value, "0987654321".to_string());
    }

    #[test]
    fn author_builder() {
        let author: OrganizationalContact = OrganizationalContactBuilder::default()
            .name(Some("name".to_owned()))
            .phone(["phone".to_owned()].to_vec())
            .email(["email".to_owned()].to_vec())
            .build().unwrap();

        assert_eq!(author.name, Some(String::from("name")));
        assert_eq!(author.email, [String::from("email")].to_vec());
        assert_eq!(author.phone, [String::from("phone")].to_vec());
    }
}
