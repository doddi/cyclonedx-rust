use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use yaserde_derive::YaSerialize;

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub struct ExternalReference {
    #[serde(rename = "type")]
    #[yaserde(rename = "type", attribute)]
    externalref_type: ExternalReferenceType,

    url: String,
    comment: String,
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
    Other,
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
            ExternalReferenceType::Other => write!(f, "Other"),
        }
    }
}
