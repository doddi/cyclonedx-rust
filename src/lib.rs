mod metadata;

use metadata::Metadata;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use yaserde::ser::Config;
use yaserde_derive::YaSerialize;

const XMLNS: &'static str = "http://cyclonedx.org/schema/Bom/1.2";
const BOM_FORMAT: &'static str = "CycloneDX";
const SPEC_VERSION: &'static str = "1.2";
const DEFAULT_VERSION: &'static str = "1";

#[derive(PartialEq)]
pub enum CycloneDXEncodeType {
    XML,
    JSON,
}

#[derive(Debug)]
pub struct CycloneDXEncodeError {}
impl Error for CycloneDXEncodeError {}
impl fmt::Display for CycloneDXEncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error encoding CycloneDX BOM")
    }
}

#[yaserde(rename = "Bom", rename_all = "camelCase")]
#[derive(YaSerialize)]
pub struct XMLCycloneDX {
    #[yaserde(rename = "serialNumber", attribute)]
    serial_number: String,
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    xmlns: String,

    cyclonedx: CycloneDX,
}

impl XMLCycloneDX {
    pub fn new() -> XMLCycloneDX {
        XMLCycloneDX {
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),
            xmlns: XMLNS.to_string(),
            cyclonedx: CycloneDX::new(None),
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct JSONCycloneDX {
    bom_format: String,
    spec_version: String,
    serial_number: String,
    version: String,

    cyclonedx: CycloneDX,
}

impl JSONCycloneDX {
    pub fn new() -> JSONCycloneDX {
        JSONCycloneDX {
            bom_format: BOM_FORMAT.to_string(),
            spec_version: SPEC_VERSION.to_string(),
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),

            cyclonedx: CycloneDX::new(None),
        }
    }
}

// #[serde(rename = "Bom", rename_all = "camelCase")]
#[derive(Serialize, YaSerialize)]
#[yaserde(flatten)]
pub struct CycloneDX {
    metadata: Option<Metadata>,
}

impl CycloneDX {
    pub fn new(metadata: Option<Metadata>) -> CycloneDX {
        CycloneDX { metadata }
    }

    pub fn encode<W>(
        writer: W,
        dx: CycloneDX,
        format: CycloneDXEncodeType,
    ) -> Result<(), CycloneDXEncodeError>
    where
        W: std::io::Write,
    {
        let result = match format {
            CycloneDXEncodeType::XML => {
                let mut xml: XMLCycloneDX = XMLCycloneDX::new();
                xml.cyclonedx = dx;
                let config: Config = Config {
                    perform_indent: true,
                    write_document_declaration: true,
                    indent_string: None,
                };
                yaserde::ser::serialize_with_writer(&xml, writer, &config);
                Ok(())
            }

            CycloneDXEncodeType::JSON => {
                let mut json: JSONCycloneDX = JSONCycloneDX::new();
                json.cyclonedx = dx;
                serde_json::to_writer_pretty(writer, &json)
            }
        };

        if result.is_err() {
            return Err(CycloneDXEncodeError {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::metadata::attached_text::*;
    use crate::metadata::classification::Classification;
    use crate::metadata::component::*;
    use crate::metadata::hash_type::HashAlg::{Sha1, Sha256};
    use crate::metadata::hash_type::HashType;
    use crate::metadata::license::*;
    use crate::metadata::organization::*;
    use crate::metadata::scope::Scope;
    use crate::metadata::swid::*;
    use crate::metadata::tool_type::{ToolType, ToolTypeBuilder};
    use crate::CycloneDXEncodeType::{JSON, XML};
    use crate::{CycloneDX, Metadata};
    use std::io::ErrorKind;

    #[test]
    fn error_if_invalid_writer() {
        let cyclone_dx = CycloneDX::new(None);

        impl std::io::Write for CycloneDX {
            fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }

            fn flush(&mut self) -> Result<(), std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }
        }

        // Used to to get access to the dummy Write trait above
        let writer = Box::new(CycloneDX::new(None));
        let result = CycloneDX::encode(writer, cyclone_dx, JSON);

        assert!(result.is_err());
    }

    #[test]
    fn can_serialize_json() {
        let mut vec = Vec::new();
        let cyclone_dx = CycloneDX::new(Option::from(Metadata::new(
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
        )));

        let result = CycloneDX::encode(&mut vec, cyclone_dx, JSON);

        let actual = String::from_utf8(vec).unwrap();
        let expected = r#"
        {
            "bomFormat": "CycloneDX",
            "specVersion": "1.2",
            "serialNumber": "urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79",
            "version": 1,
            "metadata": {
                "timestamp": "2020-04-07T07:01:00Z"
            }
        }"#;

        assert!(result.is_ok());
        assert_eq!(
            remove_all_whitespace(actual.as_ref()),
            remove_all_whitespace(expected)
        );
    }

    #[test]
    fn can_serialize_xml() {
        let mut vec = Vec::new();

        let hashes: Vec<HashType> = vec![
            HashType::new(Sha1, "1234567890".to_string()),
            HashType::new(Sha256, "0987654321".to_string()),
        ];

        let tool: ToolType = ToolTypeBuilder::default()
            .vendor("foo".to_string())
            .name("bar".to_string())
            .version("1".to_string())
            .hashes(hashes)
            .build()
            .unwrap();

        let author: OrganizationalContact = OrganizationalContactBuilder::default()
            .name(Some("name".to_owned()))
            .phone(["phone".to_owned()].to_vec())
            .email(["email".to_owned()].to_vec())
            .build()
            .unwrap();

        let contact: OrganizationalContact = OrganizationalContact::new(
            Option::from("contactName".to_string()),
            ["email".to_string()].to_vec(),
            ["phone".to_string()].to_vec(),
        );
        let swid: SwidType = SwidTypeBuilder::default()
            .tag_id("tagid".to_string())
            .name("name".to_string())
            .version(Option::from("version".to_string()))
            .tag_version(Option::from(123))
            .patch(Option::from(false))
            .text(Option::from(
                AttachedTextTypeBuilder::default()
                    .content_type(Option::from("json".to_string()))
                    .encoding(Option::from(BomEncoding::Base64))
                    .value("value".to_string())
                    .build()
                    .unwrap(),
            ))
            .url(Option::from("url".to_string()))
            .build()
            .unwrap();

        let component: Component = ComponentBuilder::default()
            .component_type(Classification::Application)
            .mime_type(Option::from("mime".to_string()))
            .bom_ref(Option::from("bom_ref".to_string()))
            .supplier(Option::from(
                OrganizationalEntityBuilder::default()
                    .name(Option::from("name".to_string()))
                    .url(["url".to_string()].to_vec())
                    .contact([contact].to_vec())
                    .build()
                    .unwrap(),
            ))
            .author(Option::from("Author name".to_string()))
            .publisher(Option::from("publisher".to_string()))
            .group(Option::from("group".to_string()))
            .name(Option::from("name".to_string()))
            .version(Option::from("version".to_string()))
            .description(Option::from("description".to_string()))
            .scope(Option::from(Scope::Required))
            .hashes(Vec::new())
            .licenses(vec![LicensesBuilder::default()
                .license(vec![LicenseTypeBuilder::default()
                    .id(Option::from("license_id".to_string()))
                    .name(Option::from("license_name".to_string()))
                    .text(None)
                    .url(None)
                    .build()
                    .unwrap()])
                .expression(None)
                .build()
                .unwrap()])
            .copyright(Option::from("copyright".to_string()))
            .purl(Option::from("purl".to_string()))
            .swid(Option::from(swid))
            .modified(Option::from(true))
            .pedigree(None)
            .external_references(Vec::new())
            .components(Vec::new())
            .build()
            .unwrap();

        let metadata = Metadata::new(
            vec![tool],
            vec![author],
            Option::from(component),
            Vec::new(),
        );

        let cyclone_dx = CycloneDX::new(Option::from(metadata));

        let result = CycloneDX::encode(&mut vec, cyclone_dx, XML);

        let actual = String::from_utf8(vec).unwrap();
        let expected = r#"
        <Bom version="1">
        <Bom>"#;

        assert!(result.is_ok());
        assert_eq!(actual, expected);
    }

    fn remove_all_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
