use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct OrganizationalEntity {
    pub name: Option<String>,
    pub url: Vec<String>,
    pub contact: Vec<OrganizationalContact>,
}

#[derive(
    Default, Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize,
)]
pub struct OrganizationalContact {
    pub name: Option<String>,
    pub email: Vec<String>,
    pub phone: Vec<String>,
}

impl OrganizationalContact {
    pub fn new(
        name: Option<String>,
        email: Vec<String>,
        phone: Vec<String>,
    ) -> OrganizationalContact {
        OrganizationalContact { name, email, phone }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use yaserde::ser::Config;

    #[test]
    pub fn print_xml() {
        let expected = OrganizationalEntityBuilder::default()
            .name(Option::from("Acme, Inc".to_string()))
            .url(vec!["https://example.com".to_string()])
            .contact(vec![OrganizationalContactBuilder::default()
                .name(Option::from("Acme Distribution".to_string()))
                .email(vec!["distribution@example.com".to_string()])
                .phone(Vec::new())
                .build()
                .unwrap()])
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

        let actual: OrganizationalEntity = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
