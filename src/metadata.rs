use std::time::SystemTime;

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::common::organization::{OrganizationalContact, OrganizationalEntity};
use crate::component::Component;
use crate::metadata::tool_type::ToolTypes;

pub mod tool_type;

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct Metadata {
    #[serde(rename = "timestamp")]
    #[yaserde(rename = "timestamp", prefix = "ns")]
    pub time_stamp: String,
    pub tools: Option<ToolTypes>,
    pub authors: Option<Authors>,
    pub component: Option<Component>,
    pub manufacture: Vec<OrganizationalEntity>,
    pub supplier: Vec<OrganizationalEntity>,
}

impl Metadata {
    pub fn new(
        tools: Option<ToolTypes>,
        authors: Option<Authors>,
        component: Option<Component>,
        manufacture: Vec<OrganizationalEntity>,
        supplier: Vec<OrganizationalEntity>,
    ) -> Metadata {
        let time_stamp: DateTime<Utc> = SystemTime::now().into();
        Metadata {
            time_stamp: time_stamp.to_rfc3339(),
            tools,
            authors,
            component,
            manufacture,
            supplier,
        }
    }
}

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Authors {
    author: Vec<OrganizationalContact>,
}

impl Authors {
    pub fn new(author: Vec<OrganizationalContact>) -> Authors {
        Authors { author }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::attached_text::BomEncoding;
    use crate::common::hash_type::HashAlg::*;
    use crate::common::hash_type::*;
    use crate::common::organization::*;
    use crate::metadata::tool_type::*;
    use crate::metadata::Metadata;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    #[test]
    fn tool_builder() {
        let mut tool_builder = ToolTypeBuilder::default();
        let tool: ToolType = tool_builder
            .name("name".to_string())
            .version("version".to_string())
            .vendor("vendor".to_string())
            .hashes(Option::from(Hashes::new(vec![
                HashType::new(Sha1, "1234567890".to_string()),
                HashType::new(Sha256, "0987654321".to_string()),
            ])))
            .build()
            .unwrap();

        assert_eq!(tool.name, "name");
        assert_eq!(tool.version, "version");
        assert_eq!(tool.vendor, "vendor");
        let vec = tool.hashes.unwrap().hash;
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0].alg, Sha1);
        assert_eq!(vec[0].value, "1234567890".to_string());
        assert_eq!(vec[1].alg, Sha256);
        assert_eq!(vec[1].value, "0987654321".to_string());
    }

    #[test]
    fn author_builder() {
        let author: OrganizationalContact = OrganizationalContactBuilder::default()
            .name(Some("name".to_owned()))
            .phone(["phone".to_owned()].to_vec())
            .email(["email".to_owned()].to_vec())
            .build()
            .unwrap();

        assert_eq!(author.name, Some(String::from("name")));
        assert_eq!(author.email, [String::from("email")].to_vec());
        assert_eq!(author.phone, [String::from("phone")].to_vec());
    }

    #[test]
    pub fn can_decode() {
        let reader = setup("metadata-1.2.xml");

        let response: Metadata = yaserde::de::from_reader(reader).unwrap();

        assert_eq!(response.time_stamp, "2020-04-07T07:01:00Z");

        let tool_types = response.tools.unwrap().tool;
        assert_eq!(tool_types.len(), 1);
        assert_eq!(tool_types[0].vendor, "Awesome Vendor");
        assert_eq!(tool_types[0].name, "Awesome Tool");
        assert_eq!(tool_types[0].version, "9.1.2");

        let hashes = tool_types[0].hashes.clone().unwrap().hash;
        assert_eq!(hashes.len(), 2);
        assert_eq!(hashes[0].alg, HashAlg::Sha1);
        assert_eq!(hashes[0].value, "25ed8e31b995bb927966616df2a42b979a2717f0");
        assert_eq!(hashes[1].alg, HashAlg::Sha256);
        assert_eq!(
            hashes[1].value,
            "a74f733635a19aefb1f73e5947cef59cd7440c6952ef0f03d09d974274cbd6df"
        );

        let authors = response.authors.unwrap().author;
        assert_eq!(authors.len(), 1);
        assert_eq!(authors[0].name.as_ref().unwrap(), "Samantha Wright");
        assert_eq!(authors[0].email[0], "samantha.wright@example.com");
        assert_eq!(authors[0].phone[0], "800-555-1212");

        let component = response.component.unwrap();
        assert_eq!(component.name.unwrap(), "Acme Application");
        assert_eq!(component.version.unwrap(), "9.1.1");
        let swid = component.swid.unwrap();
        assert_eq!(
            swid.tag_id,
            "swidgen-242eb18a-503e-ca37-393b-cf156ef09691_9.1.1"
        );
        assert_eq!(swid.name, "Acme Application");
        assert_eq!(swid.version.unwrap(), "9.1.1");
        let text_type = swid.text.unwrap();
        assert_eq!(text_type.content_type.unwrap(), "text/xml");
        assert_eq!(text_type.encoding.unwrap(), BomEncoding::Base64);
        assert_eq!(text_type.value, "PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiID8+CjxTb2Z0d2FyZUlkZW50aXR5IHhtbDpsYW5nPSJFTiIgbmFtZT0iQWNtZSBBcHBsaWNhdGlvbiIgdmVyc2lvbj0iOS4xLjEiIAogdmVyc2lvblNjaGVtZT0ibXVsdGlwYXJ0bnVtZXJpYyIgCiB0YWdJZD0ic3dpZGdlbi1iNTk1MWFjOS00MmMwLWYzODItM2YxZS1iYzdhMmE0NDk3Y2JfOS4xLjEiIAogeG1sbnM9Imh0dHA6Ly9zdGFuZGFyZHMuaXNvLm9yZy9pc28vMTk3NzAvLTIvMjAxNS9zY2hlbWEueHNkIj4gCiB4bWxuczp4c2k9Imh0dHA6Ly93d3cudzMub3JnLzIwMDEvWE1MU2NoZW1hLWluc3RhbmNlIiAKIHhzaTpzY2hlbWFMb2NhdGlvbj0iaHR0cDovL3N0YW5kYXJkcy5pc28ub3JnL2lzby8xOTc3MC8tMi8yMDE1LWN1cnJlbnQvc2NoZW1hLnhzZCBzY2hlbWEueHNkIiA+CiAgPE1ldGEgZ2VuZXJhdG9yPSJTV0lEIFRhZyBPbmxpbmUgR2VuZXJhdG9yIHYwLjEiIC8+IAogIDxFbnRpdHkgbmFtZT0iQWNtZSwgSW5jLiIgcmVnaWQ9ImV4YW1wbGUuY29tIiByb2xlPSJ0YWdDcmVhdG9yIiAvPiAKPC9Tb2Z0d2FyZUlkZW50aXR5Pg==");

        assert_eq!(response.manufacture.len(), 1);
        let manufacturer = response.manufacture[0].clone();
        assert_eq!(manufacturer.name.as_ref().unwrap(), "Acme, Inc.");
        assert_eq!(manufacturer.url.len(), 2);
        assert_eq!(manufacturer.url[0], "https://example.com");
        assert_eq!(manufacturer.url[1], "https://example2.com");
        assert_eq!(manufacturer.contact.len(), 1);
        assert_eq!(
            manufacturer.contact[0].name.as_ref().unwrap(),
            "Acme Professional Services"
        );
        assert_eq!(
            manufacturer.contact[0].email[0],
            "professional.services@example.com"
        );

        assert_eq!(response.supplier.len(), 1);
        let manufacturer = response.supplier[0].clone();
        assert_eq!(manufacturer.name.as_ref().unwrap(), "Acme, Inc.");
        assert_eq!(manufacturer.url.len(), 1);
        assert_eq!(manufacturer.url[0], "https://example.com");
        assert_eq!(manufacturer.contact.len(), 1);
        assert_eq!(
            manufacturer.contact[0].name.as_ref().unwrap(),
            "Acme Distribution"
        );
        assert_eq!(manufacturer.contact[0].email[0], "distribution@example.com");
    }

    fn setup(file: &str) -> BufReader<File> {
        let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_folder.push("resources/test/".to_owned() + file);
        let file = File::open(test_folder);
        let reader = BufReader::new(file.unwrap());
        reader
    }
}
