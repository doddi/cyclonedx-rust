use serde::{Serialize};
use yaserde_derive::YaSerialize;
use derive_builder::{Builder};
use crate::metadata::component::Component;
use crate::metadata::attached_text::AttachedTextType;
use std::fmt::{Display, Formatter};
use std::fmt;

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
