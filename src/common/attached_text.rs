use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(
    Clone, Default, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize,
)]
#[yaserde(rename = "text")]
pub struct AttachedTextType {
    #[serde(rename = "content-type")]
    #[yaserde(rename = "content-type", attribute)]
    pub content_type: Option<String>,

    #[yaserde(attribute)]
    pub encoding: Option<BomEncoding>,

    #[yaserde(text)]
    pub value: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub enum BomEncoding {
    #[serde(rename = "base64")]
    #[yaserde(rename = "base64")]
    Base64,
}

impl Default for BomEncoding {
    fn default() -> Self {
        BomEncoding::Base64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaserde::ser::Config;

    #[test]
    fn print_xml() {
        let expected: AttachedTextType = AttachedTextTypeBuilder::default()
            .content_type(Option::from("text/xml".to_string()))
            .encoding(Option::from(BomEncoding::Base64))
            .value("content".to_string())
            .build()
            .unwrap();

        let parsed = yaserde::ser::to_string_with_config(
            &expected,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        let actual: AttachedTextType = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
