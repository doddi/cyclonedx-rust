use crate::common::attached_text::AttachedTextType;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(
    Clone, Default, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize,
)]
#[serde(rename = "licenses")]
#[yaserde(rename = "licenses")]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct Licenses {
    #[yaserde(prefix = "ns")]
    pub license: Vec<LicenseType>,
    #[yaserde(prefix = "ns")]
    pub expression: Option<String>,
}

#[derive(
    Default, Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize,
)]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct LicenseType {
    #[yaserde(prefix = "ns")]
    pub id: Option<String>,
    #[yaserde(prefix = "ns")]
    pub name: Option<String>,
    pub text: Option<AttachedTextType>,
    #[yaserde(prefix = "ns")]
    pub url: Option<String>,
}

impl LicenseType {
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        text: Option<AttachedTextType>,
        url: Option<String>,
    ) -> LicenseType {
        LicenseType {
            id,
            name,
            text,
            url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::attached_text::*;
    use yaserde::ser::Config;

    #[test]
    pub fn print_expression_license_xml() {
        let licenses: Licenses = LicensesBuilder::default()
            .license(Vec::new())
            .expression(Option::from(
                "EPL-2.0 OR GPL-2.0-with-classpath-exception".to_string(),
            ))
            .build()
            .unwrap();

        let expected = r#"<licenses xmlns="http://cyclonedx.org/schema/bom/1.2"><expression>EPL-2.0 OR GPL-2.0-with-classpath-exception</expression></licenses>"#;
        let actual = yaserde::ser::to_string_with_config(
            &licenses,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        assert_eq!(expected.to_string(), actual);
    }

    #[test]
    pub fn print_license_xml() {
        let expected: Licenses = LicensesBuilder::default()
            .license(vec![LicenseTypeBuilder::default()
                .id(Option::from("Apache-2.0".to_string()))
                .name(None)
                .text(Option::from(
                    AttachedTextTypeBuilder::default()
                        .content_type(Option::from("text/plain".to_string()))
                        .encoding(Option::from(BomEncoding::Base64))
                        .value("base64_value".to_string())
                        .build()
                        .unwrap(),
                ))
                .url(Option::from(
                    "https://www.apache.org/licenses/LICENSE-2.0.txt".to_string(),
                ))
                .build()
                .unwrap()])
            .expression(None)
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

        let actual: Licenses = yaserde::de::from_str(parsed.as_str()).unwrap();
        assert_eq!(expected, actual);
    }
}
