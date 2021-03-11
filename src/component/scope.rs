use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub enum Scope {
    #[serde(rename = "required")]
    #[yaserde(rename = "required")]
    Required,
    #[serde(rename = "optional")]
    #[yaserde(rename = "optional")]
    Optional,
    #[serde(rename = "excluded")]
    #[yaserde(rename = "excluded")]
    Excluded,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Required
    }
}
