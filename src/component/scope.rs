use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub enum Scope {
    Required,
    Optional,
    Excluded,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Required
    }
}
