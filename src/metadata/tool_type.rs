use crate::metadata::hash_type::HashType;

use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::YaSerialize;

#[derive(Debug, Builder, Serialize, YaSerialize)]
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
