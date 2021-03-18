use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct DependencyTypes {
    pub dependency: Vec<DependencyType>,
}

impl DependencyTypes {
    pub fn new(dependency: Vec<DependencyType>) -> DependencyTypes {
        DependencyTypes { dependency }
    }
}

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct DependencyType {
    #[serde(rename = "ref")]
    #[yaserde(rename = "ref", attribute)]
    ref_type: String,
    dependency: Vec<DependencyType>,
}

impl DependencyType {
    pub fn new(ref_type: String, dependency: Vec<DependencyType>) -> DependencyType {
        DependencyType {
            ref_type,
            dependency,
        }
    }
}
