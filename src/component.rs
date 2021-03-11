use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use classification::Classification;
use external_reference::ExternalReference;
use pedigree_type::PedigreeType;
use scope::Scope;
use swid::SwidType;

use crate::common::hash_type::HashType;
use crate::common::license::Licenses;
use crate::common::organization::OrganizationalEntity;

pub mod classification;
pub mod external_reference;
pub mod pedigree_type;
pub mod scope;
pub mod swid;

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
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
    components: Vec<Component>,
}
