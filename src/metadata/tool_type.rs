use crate::common::hash_type::{HashType, Hashes};

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
pub struct ToolType {
    pub vendor: String,
    pub name: String,
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
