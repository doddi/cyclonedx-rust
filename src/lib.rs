use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};
use yaserde::ser::Config;
use yaserde_derive::{YaDeserialize, YaSerialize};

use crate::dependency_type::DependencyType;
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

#[yaserde(rename = "bom", rename_all = "camelCase")]
#[derive(Default, YaSerialize, YaDeserialize)]
pub struct XMLCycloneDX {
    #[yaserde(rename = "serialNumber", attribute)]
    serial_number: String,
    #[yaserde(attribute)]
    version: String,
    // #[yaserde(attribute)]
    // xmlns: String,
    #[yaserde(flatten)]
    cyclonedx: CycloneDX,
}

impl XMLCycloneDX {
    pub fn new() -> XMLCycloneDX {
        XMLCycloneDX {
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),
            // xmlns: XMLNS.to_string(),
            cyclonedx: CycloneDX::new(None, None, None, Vec::new()),
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize)]
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

            cyclonedx: CycloneDX::new(None, None, None, Vec::new()),
        }
    }
}

#[derive(Default, Serialize, Deserialize, YaSerialize, YaDeserialize)]
#[yaserde(flatten)]
pub struct CycloneDX {
    metadata: Option<Metadata>,
    components: Option<Components>,
    services: Option<Services>,
    dependencies: Vec<DependencyType>,
}

impl CycloneDX {
    pub fn new(
        metadata: Option<Metadata>,
        components: Option<Components>,
        services: Option<Services>,
        dependencies: Vec<DependencyType>,
    ) -> CycloneDX {
        CycloneDX {
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
                let result: Result<XMLCycloneDX, String> = yaserde::de::from_reader(reader);
                match result {
                    Ok(response) => Ok(response.cyclonedx),
                    Err(err) => Err(err),
                }
            }
            CycloneDXFormatType::JSON => {
                let cyclone_dx: JSONCycloneDX = serde_json::from_reader(reader).unwrap();
                Ok(cyclone_dx.cyclonedx)
            }
        };

        if result.is_err() {
            return Err(CycloneDXDecodeError {});
        }
        Ok(result.unwrap())
    }

    pub fn encode<W>(
        writer: W,
        dx: CycloneDX,
        format: CycloneDXFormatType,
    ) -> Result<(), CycloneDXEncodeError>
    where
        W: std::io::Write,
    {
        let result = match format {
            CycloneDXFormatType::XML => {
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

            CycloneDXFormatType::JSON => {
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

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct Components {
    pub component: Vec<Component>,
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, ErrorKind};

    use crate::component::classification::Classification;
    use crate::CycloneDXFormatType::JSON;
    use crate::{CycloneDX, CycloneDXFormatType, XMLCycloneDX};
    use std::fs::File;
    use std::path::PathBuf;

    #[test]
    fn error_if_invalid_writer() {
        let cyclone_dx = CycloneDX::new(None, None, None, Vec::new());

        impl std::io::Write for CycloneDX {
            fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }

            fn flush(&mut self) -> Result<(), std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }
        }

        // Used to to get access to the dummy Write trait above
        let writer = Box::new(CycloneDX::new(None, None, None, Vec::new()));
        let result = CycloneDX::encode(writer, cyclone_dx, JSON);

        assert!(result.is_err());
    }

    #[test]
    pub fn can_decode() {
        let reader = setup("bom-1.2.xml");

        let result: XMLCycloneDX = yaserde::de::from_reader(reader).unwrap();

        assert_eq!(
            result.serial_number,
            "urn:uuid:3e671687-395b-41f5-a30f-a58921a69b79"
        );

        let cyclone_dx = result.cyclonedx;
        validate(cyclone_dx);
    }

    #[test]
    pub fn can_decode_using_decoder() {
        let reader = setup("bom-1.2.xml");

        let cyclone_dx = CycloneDX::decode(reader, CycloneDXFormatType::XML).unwrap();

        validate(cyclone_dx);
    }

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
        let mut reader = BufReader::new(file.unwrap());
        reader
    }
}
