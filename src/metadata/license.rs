use serde::{Serialize};
use yaserde_derive::YaSerialize;
use derive_builder::{Builder};
use crate::metadata::attached_text::AttachedTextType;

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct Licenses {
    license: Vec<LicenseType>,
    expression: Option<String>
}

#[derive(Default, Clone, Builder, PartialEq, Debug, Serialize, YaSerialize)]
pub struct LicenseType {
    id: Option<String>,
    name: Option<String>,
    text: Option<AttachedTextType>,
    url: Option<String>,
}

impl LicenseType {
    pub fn new(id: Option<String>, name: Option<String>, text: Option<AttachedTextType>, url: Option<String>) -> LicenseType {
        LicenseType { id, name, text, url }
    }
}
