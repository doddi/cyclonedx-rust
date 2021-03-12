use crate::common::attached_text::AttachedTextType;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(
    Clone, Default, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize,
)]
pub struct SwidType {
    #[serde(rename = "tagId")]
    #[yaserde(rename = "tagId", attribute)]
    pub tag_id: String,

    #[yaserde(attribute)]
    pub name: String,

    #[yaserde(attribute)]
    pub version: Option<String>,

    #[serde(rename = "tagVersion")]
    #[yaserde(rename = "tagVersion", attribute)]
    pub tag_version: Option<i32>,

    #[yaserde(attribute)]
    pub patch: Option<bool>,

    pub text: Option<AttachedTextType>,
    pub url: Option<String>,
}
