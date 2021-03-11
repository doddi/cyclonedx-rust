use serde::{Serialize};
use yaserde_derive::YaSerialize;
use derive_builder::{Builder};
use std::fmt::{Display, Formatter};
use std::fmt;
use crate::metadata::{OrganizationalEntity};
use crate::metadata::hash_type::HashType;
use crate::metadata::license::Licenses;
use crate::metadata::attached_text::AttachedTextType;
use crate::metadata::swid::SwidType;
use crate::metadata::pedigree_type::PedigreeType;
use crate::metadata::external_reference::ExternalReference;
use crate::metadata::classification::Classification;
use crate::metadata::scope::Scope;

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
    hashes: Vec<HashType>,
    licenses: Vec<Licenses>,
    copyright: Option<String>,
    purl: Option<String>,
    swid: Option<SwidType>,
    modified: Option<bool>,
    pedigree: Option<PedigreeType>,
    external_references: Vec<ExternalReference>,
    components: Vec<Component>
}

