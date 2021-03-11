mod data_classification_type;
mod data_flow_type;

use crate::common::license::Licenses;
use crate::common::organization::OrganizationalEntity;
use crate::component::external_reference::ExternalReference;
use crate::service::data_classification_type::DataClassificationType;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Service {
    #[serde(rename = "bom-ref")]
    #[yaserde(rename = "bom-ref", attribute)]
    bom_ref: Option<String>,

    provider: Option<OrganizationalEntity>,
    group: Option<String>,
    name: String,
    version: Option<String>,
    description: Option<String>,
    endpoints: Option<Endpoints>,
    authenticated: Option<bool>,
    #[serde(rename = "x-trust-boundary")]
    #[yaserde(rename = "x-trust-boundary")]
    x_trust_boundary: Option<bool>,
    data: Option<Classifications>,
    licenses: Option<Licenses>,
    #[serde(rename = "externalReferences")]
    #[yaserde(rename = "externalReferences")]
    external_references: Option<ExternalReferences>,
    services: Vec<Service>,
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct ExternalReferences {
    reference: Vec<ExternalReference>,
}

impl ExternalReferences {
    pub fn new(reference: Vec<ExternalReference>) -> ExternalReferences {
        ExternalReferences { reference }
    }
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Classifications {
    classification: Vec<DataClassificationType>,
}

impl Classifications {
    pub fn new(classification: Vec<DataClassificationType>) -> Classifications {
        Classifications { classification }
    }
}

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Endpoints {
    endpoint: Vec<EndpointType>,
}

impl Endpoints {
    pub fn new(endpoint: Vec<EndpointType>) -> Endpoints {
        Endpoints { endpoint }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct EndpointType {
    #[yaserde(text)]
    value: String,
}

impl EndpointType {
    pub fn new(value: String) -> EndpointType {
        EndpointType { value }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::license::*;
    use crate::common::organization::*;
    use crate::component::external_reference::*;
    use crate::service::data_classification_type::*;
    use crate::service::data_flow_type::DataFlowType;
    use crate::CycloneDX;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
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
            .endpoints(Option::from(Endpoints::new(vec![
                EndpointType::new("https://partner.org/api/v1/lookup".to_string()),
                EndpointType::new("https://partner.org/api/v1/stock".to_string()),
            ])))
            .authenticated(Option::from(true))
            .x_trust_boundary(Option::from(true))
            .data(Option::from(Classifications::new(vec![
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
            ])))
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
            .external_references(Option::from(ExternalReferences::new(vec![
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
            ])))
            .services(Vec::new())
            .build()
            .unwrap();

        let parsed = yaserde::ser::to_string_with_config(
            &expected,
            &Config {
                perform_indent: true,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();

        let actual: Service = yaserde::de::from_str(parsed.as_str()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn can_decode() {
        let reader = setup("service-1.2.xml");

        let response: Service = yaserde::de::from_reader(reader).unwrap();

        assert_eq!(response.name, "Stock ticker service");
        assert_eq!(response.group.unwrap(), "org.partner");
        assert_eq!(response.version.unwrap(), "2020-Q2");
        assert_eq!(
            response.description.unwrap(),
            "Provides real-time stock information"
        );
        let endpoints = response.endpoints.unwrap();
        assert_eq!(endpoints.endpoint.len(), 2);
        assert_eq!(
            endpoints.endpoint[0].value,
            "https://partner.org/api/v1/lookup"
        );
        assert_eq!(
            endpoints.endpoint[1].value,
            "https://partner.org/api/v1/stock"
        );
        assert_eq!(response.authenticated.unwrap(), true);
        assert_eq!(response.x_trust_boundary.unwrap(), true);

        let classifications = response.data.unwrap().classification;
        assert_eq!(classifications.len(), 3);
        assert_eq!(classifications[0].flow, DataFlowType::InBound);
        assert_eq!(classifications[0].value, "PII");
        assert_eq!(classifications[1].flow, DataFlowType::Outbound);
        assert_eq!(classifications[1].value, "PIFI");
        assert_eq!(classifications[2].flow, DataFlowType::BiDirectional);
        assert_eq!(classifications[2].value, "public");

        let licenses = response.licenses.unwrap();
        assert!(!licenses.expression.is_some());
        assert_eq!(licenses.license.len(), 1);
        let license_type = licenses.license[0].clone();
        assert_eq!(license_type.name.unwrap(), "Partner license");

        let references = response.external_references.unwrap().reference;
        assert_eq!(references.len(), 2);
        assert_eq!(references[0].ref_type, ExternalReferenceType::Website);
        assert_eq!(references[0].url, "http://partner.org");
        assert_eq!(references[1].ref_type, ExternalReferenceType::Documentation);
        assert_eq!(references[1].url, "http://api.partner.org/swagger");
    }

    fn setup(file: &str) -> BufReader<File> {
        let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_folder.push("resources/test/".to_owned() + file);
        let file = File::open(test_folder);
        let mut reader = BufReader::new(file.unwrap());
        reader
    }
}
