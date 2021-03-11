use crate::common::hash_type::HashType;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Debug, Builder, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct ToolType {
    pub vendor: String,
    pub name: String,
    pub version: String,
    pub hashes: Vec<HashType>,
}

impl ToolType {
    pub fn new(vendor: String, name: String, version: String, hashes: Vec<HashType>) -> ToolType {
        ToolType {
            vendor,
            name,
            version,
            hashes,
        }
    }
}
