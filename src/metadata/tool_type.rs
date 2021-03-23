use crate::common::hash_type::Hashes;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct ToolTypes {
    pub tool: Vec<ToolType>,
}

impl ToolTypes {
    pub fn new(tool: Vec<ToolType>) -> ToolTypes {
        ToolTypes { tool }
    }
}

#[derive(Clone, PartialEq, Debug, Builder, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct ToolType {
    #[yaserde(prefix = "ns")]
    pub vendor: String,
    #[yaserde(prefix = "ns")]
    pub name: String,
    #[yaserde(prefix = "ns")]
    pub version: String,
    pub hashes: Option<Hashes>,
}

impl ToolType {
    pub fn new(vendor: String, name: String, version: String, hashes: Option<Hashes>) -> ToolType {
        ToolType {
            vendor,
            name,
            version,
            hashes,
        }
    }
}
