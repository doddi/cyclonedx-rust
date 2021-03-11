use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use serde::Serialize;
use yaserde::ser::Config;
use yaserde_derive::YaSerialize;

use crate::dependency_type::DependencyType;
use crate::service::Service;
use component::Component;
use metadata::Metadata;

mod common;
pub mod component;
mod dependency_type;
pub mod metadata;
pub mod service;

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
            cyclonedx: CycloneDX::new(None, Vec::new(), Vec::new(), Vec::new()),
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

            cyclonedx: CycloneDX::new(None, Vec::new(), Vec::new(), Vec::new()),
        }
    }
}

// #[serde(rename = "Bom", rename_all = "camelCase")]
#[derive(Serialize, YaSerialize)]
#[yaserde(flatten)]
pub struct CycloneDX {
    metadata: Option<Metadata>,
    components: Vec<Component>,
    service: Vec<Service>,
    dependencies: Vec<DependencyType>,
}

impl CycloneDX {
    pub fn new(
        metadata: Option<Metadata>,
        components: Vec<Component>,
        service: Vec<Service>,
        dependencies: Vec<DependencyType>,
    ) -> CycloneDX {
        CycloneDX {
            metadata,
            components,
            service,
            dependencies,
        }
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
    use std::io::ErrorKind;

    use crate::CycloneDX;
    use crate::CycloneDXEncodeType::JSON;

    #[test]
    fn error_if_invalid_writer() {
        let cyclone_dx = CycloneDX::new(None, Vec::new(), Vec::new(), Vec::new());

        impl std::io::Write for CycloneDX {
            fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }

            fn flush(&mut self) -> Result<(), std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }
        }

        // Used to to get access to the dummy Write trait above
        let writer = Box::new(CycloneDX::new(None, Vec::new(), Vec::new(), Vec::new()));
        let result = CycloneDX::encode(writer, cyclone_dx, JSON);

        assert!(result.is_err());
    }
}
