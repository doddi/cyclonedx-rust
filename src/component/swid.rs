use crate::common::attached_text::AttachedTextType;
use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct SwidType {
    #[serde(rename = "tagId")]
    #[yaserde(rename = "tagId", attribute)]
    tag_id: String,

    #[yaserde(attribute)]
    name: String,

    #[yaserde(attribute)]
    version: Option<String>,

    #[serde(rename = "tagVersion")]
    #[yaserde(rename = "tagVersion", attribute)]
    tag_version: Option<i32>,

    #[yaserde(attribute)]
    patch: Option<bool>,

    text: Option<AttachedTextType>,
    url: Option<String>,
}
