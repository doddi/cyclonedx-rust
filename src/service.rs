mod data_classification_type;
mod data_flow_type;

use crate::common::license::Licenses;
use crate::common::organization::OrganizationalEntity;
use crate::component::external_reference::ExternalReference;
use crate::service::data_classification_type::DataClassificationType;
use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct Service {
    #[serde(rename = "bom-ref")]
    #[yaserde(rename = "bom-ref", attribute)]
    bom_ref: Option<String>,

    provider: Option<OrganizationalEntity>,
    group: Option<String>,
    name: String,
    version: Option<String>,
    description: Option<String>,
    endpoints: Vec<String>,
    authenticated: Option<bool>,
    #[serde(rename = "x-trust-boundary")]
    #[yaserde(rename = "x-trust-boundary")]
    x_trust_boundary: Option<bool>,
    data: Vec<DataClassificationType>,
    licenses: Option<Licenses>,
    external_references: Vec<ExternalReference>,
    services: Vec<Service>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::license::*;
    use crate::common::organization::*;
    use crate::component::external_reference::*;
    use crate::service::data_classification_type::*;
    use crate::service::data_flow_type::DataFlowType;
    use yaserde::ser::Config;

    #[test]
    pub fn print_xml() {
        let expected = ServiceBuilder::default()
            .bom_ref(Option::from(
                "b2a46a4b-8367-4bae-9820-95557cfe03a8".to_string(),
            ))
            .provider(Option::from(
                OrganizationalEntityBuilder::default()
                    .name(Option::from("Partner Org".to_string()))
                    .url(vec!["https://partner.org".to_string()])
                    .contact(vec![OrganizationalContactBuilder::default()
                        .name(Option::from("Support".to_string()))
                        .email(vec!["support@partner".to_string()])
                        .phone(vec!["800-555-1212".to_string()])
                        .build()
                        .unwrap()])
                    .build()
                    .unwrap(),
            ))
            .group(Option::from("org.partner".to_string()))
            .name("Stock ticker service".to_string())
            .version(Option::from("2020-Q2".to_string()))
            .description(Option::from(
                "Provides real-time stock information".to_string(),
            ))
            .endpoints(vec![
                "https://partner.org/api/v1/lookup".to_string(),
                "https://partner.org/api/v1/stock".to_string(),
            ])
            .authenticated(Option::from(true))
            .x_trust_boundary(Option::from(true))
            .data(vec![
                DataClassificationTypeBuilder::default()
                    .flow(DataFlowType::InBound)
                    .value("PII".to_string())
                    .build()
                    .unwrap(),
                DataClassificationTypeBuilder::default()
                    .flow(DataFlowType::Outbound)
                    .value("PIFI".to_string())
                    .build()
                    .unwrap(),
                DataClassificationTypeBuilder::default()
                    .flow(DataFlowType::BiDirectional)
                    .value("public".to_string())
                    .build()
                    .unwrap(),
            ])
            .licenses(Option::from(
                LicensesBuilder::default()
                    .license(vec![LicenseTypeBuilder::default()
                        .id(None)
                        .name(Option::from("Partner License".to_string()))
                        .text(None)
                        .url(None)
                        .build()
                        .unwrap()])
                    .expression(None)
                    .build()
                    .unwrap(),
            ))
            .external_references(vec![
                ExternalReference::new(
                    ExternalReferenceType::Website,
                    "http://partner.org".to_string(),
                    None,
                ),
                ExternalReference::new(
                    ExternalReferenceType::Documentation,
                    "http://api.partner.org/swagger".to_string(),
                    None,
                ),
            ])
            .services(Vec::new())
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

        let actual: Service = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }
}
