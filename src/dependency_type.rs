use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
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
