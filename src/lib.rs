//! #CycloneDx-Rust
//!
//! CycloneDx-Rust is a Crate library for encoding and decoding [CycloneDx](https://cyclonedx.org/) files in both XML and JSON format
//! to the 1.2 spec
//!
//! To encode the CycloneDx you cab=n either build up the structure using the provided <X>::new() methods, passing in the parameters where necessary
//! or make use of the builder pattern.
//! The builder patterns are created at build time so intelli-sense may not be available. Howver, each struct, for example:
//! ```
//! use cyclonedx_rust::CycloneDX;
//!
//! CycloneDX::new(None, None, None, None);
//! ```
//! can be built as follows:
//! ```
//! use cyclonedx_rust::CycloneDXBuilder;
//!
//! CycloneDXBuilder::default()
//!  .metadata(None)
//!  .components(None)
//!  .services(None)
//!  .dependencies(None)
//!  .build();
//! ```
//!
//! # Encoding
//! An example of how to encode a CycloneDX BoM to a file:
//!
//! ```
//! use cyclonedx_rust::{CycloneDX, CycloneDXFormatType};
//! use std::io::BufWriter;
//! use std::fs::File;
//!
//! let mut buffer = BufWriter::new(File::create("foo.txt").unwrap());
//! let cyclone_dx = CycloneDX::new(None, None, None, None);
//! CycloneDX::encode(&mut buffer, cyclone_dx, CycloneDXFormatType::XML);
//! ```
//!
//! # Decoding
//! An example of how to decode a CycloneDX BoM:
//!
//! ```
//! use cyclonedx_rust::{CycloneDX, CycloneDXFormatType};
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::path::PathBuf;
//!
//! let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//! test_folder.push("resources/test/bom-1.2.xml");
//! let file = File::open(test_folder);
//! let mut reader = BufReader::new(file.unwrap());
//!
//! let result: CycloneDX = CycloneDX::decode(reader, CycloneDXFormatType::XML).unwrap();
//! ```
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use yaserde::ser::Config;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::dependency_type::DependencyTypes;
use crate::service::Services;
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

#[skip_serializing_none]
#[derive(Default, Builder, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[yaserde(rename = "bom")]
#[serde(rename = "bom", rename_all = "camelCase")]
#[yaserde(
    prefix = "ns",
    default_namespace = "ns",
    namespace = "ns: http://cyclonedx.org/schema/bom/1.2"
)]
pub struct CycloneDX {
    // JSON only
    #[yaserde(skip_serializing_if = "json_skip")]
    bom_format: String,
    #[yaserde(skip_serializing_if = "json_skip")]
    spec_version: String,

    #[yaserde(attribute)]
    version: String,

    #[yaserde(rename = "serialNumber", attribute)]
    serial_number: String,

    metadata: Option<Metadata>,
    components: Option<Components>,
    services: Option<Services>,
    dependencies: Option<DependencyTypes>,
}

impl CycloneDX {
    pub fn new(
        metadata: Option<Metadata>,
        components: Option<Components>,
        services: Option<Services>,
        dependencies: Option<DependencyTypes>,
    ) -> Self {
        CycloneDX {
            bom_format: BOM_FORMAT.to_string(),
            spec_version: SPEC_VERSION.to_string(),
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),
            metadata,
            components,
            services,
            dependencies,
        }
    }

    pub fn decode<R>(
        reader: R,
        format: CycloneDXFormatType,
    ) -> Result<CycloneDX, CycloneDXDecodeError>
    where
        R: std::io::Read,
    {
        let result: Result<CycloneDX, String> = match format {
            CycloneDXFormatType::XML => {
                let result: Result<CycloneDX, String> = yaserde::de::from_reader(reader);
                match result {
                    Ok(response) => Ok(response),
                    Err(err) => Err(err),
                }
            }
            CycloneDXFormatType::JSON => {
                unimplemented!();
                let cyclone_dx: CycloneDX = serde_json::from_reader(reader).unwrap();
                Ok(cyclone_dx)
            }
        };

        if result.is_err() {
            return Err(CycloneDXDecodeError {});
        }
        Ok(result.unwrap())
    }

    pub fn encode<W>(
        writer: W,
        cyclone_dx: CycloneDX,
        format: CycloneDXFormatType,
    ) -> Result<(), CycloneDXEncodeError>
    where
        W: std::io::Write,
    {
        let result = match format {
            CycloneDXFormatType::XML => {
                let config: Config = Config {
                    perform_indent: true,
                    write_document_declaration: true,
                    indent_string: None,
                };
                match yaserde::ser::serialize_with_writer(&cyclone_dx, writer, &config) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }

            CycloneDXFormatType::JSON => {
                unimplemented!();
                match serde_json::to_writer_pretty(writer, &cyclone_dx) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.to_string()),
                }
            }
        };

        if result.is_err() {
            return Err(CycloneDXEncodeError {});
        }
        Ok(())
    }

    pub const fn json_skip(&self, _: &str) -> bool {
        true
    }
}

#[derive(PartialEq)]
pub enum CycloneDXFormatType {
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

#[derive(Debug)]
pub struct CycloneDXDecodeError {}
impl Error for CycloneDXDecodeError {}
impl fmt::Display for CycloneDXDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error decoding CycloneDX BOM")
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Components {
    pub component: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, ErrorKind};

    use crate::component::classification::Classification;
    use crate::CycloneDXFormatType::XML;
    use crate::{CycloneDX, CycloneDXFormatType};
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn error_if_invalid_writer() {
        let cyclone_dx = CycloneDX::new(None, None, None, None);

        impl std::io::Write for CycloneDX {
            fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }

            fn flush(&mut self) -> Result<(), std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }
        }

        // Used to to get access to the dummy Write trait above
        let writer = Box::new(CycloneDX::new(None, None, None, None));
        let result = CycloneDX::encode(writer, cyclone_dx, XML);

        assert!(result.is_err());
    }

    #[test]
    pub fn can_decode() {
        let reader = setup("bom-1.2.xml");

        let result: CycloneDX = yaserde::de::from_reader(reader).unwrap();

        assert_eq!(
            result.serial_number,
            "urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79"
        );

        validate(result);
    }

    #[test]
    pub fn can_decode_using_decoder() {
        let reader = setup("bom-1.2.xml");

        let cyclone_dx = CycloneDX::decode(reader, CycloneDXFormatType::XML).unwrap();

        validate(cyclone_dx);
    }

    #[test]
    pub fn can_recode_xml() {
        let mut buffer = Vec::new();
        let cyclone_dx = CycloneDX::new(None, None, None, None);
        CycloneDX::encode(&mut buffer, cyclone_dx, CycloneDXFormatType::XML);
        let response = CycloneDX::decode(&buffer[..], CycloneDXFormatType::XML).unwrap();

        assert_eq!(response.version, "1");
    }

    #[test]
    pub fn can_encode_basic_xml() {
        let mut writer = Vec::new();
        let cyclone_dx = CycloneDX::new(None, None, None, None);
        CycloneDX::encode(&mut writer, cyclone_dx, CycloneDXFormatType::XML);

        let result = String::from_utf8(writer).unwrap();
        assert!(!result.contains("CycloneDX"));
    }

    // #[test]
    // pub fn can_encode_basic_json() {
    //     let mut writer = Vec::new();
    //     let cyclone_dx = CycloneDX::new(None, None, None, None);
    //     CycloneDX::encode(&mut writer, cyclone_dx, CycloneDXFormatType::JSON);
    //
    //     let result = String::from_utf8(writer).unwrap();
    //     assert!(result.contains("CycloneDX"));
    // }

    fn validate(cyclone_dx: CycloneDX) {
        let metadata = cyclone_dx.metadata.as_ref().unwrap();
        assert_eq!(metadata.time_stamp, "2020-04-07T07:01:00Z");

        let component = cyclone_dx.components.as_ref().unwrap();
        assert_eq!(component.component.len(), 3);
        assert_eq!(
            component.component[0].name.as_ref().unwrap(),
            "tomcat-catalina"
        );
        assert_eq!(
            component.component[2].component_type,
            Classification::Framework
        );

        let services = cyclone_dx.services.as_ref().unwrap();
        assert_eq!(services.service.len(), 1);
        assert_eq!(services.service[0].name, "Stock ticker service");
        assert_eq!(
            services.service[0].endpoints.as_ref().unwrap().endpoint[0].value,
            "https://partner.org/api/v1/lookup"
        );
    }

    fn setup(file: &str) -> BufReader<File> {
        let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_folder.push("resources/test/".to_owned() + file);
        let file = File::open(test_folder);
        let reader = BufReader::new(file.unwrap());
        reader
    }
}
