use serde::{Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use yaserde_derive::YaSerialize;
use yaserde::ser::Config;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

const XMLNS: &'static str = "http://cyclonedx.org/schema/bom/1.2";
const BOM_FORMAT: &'static str = "CycloneDX";
const SPEC_VERSION: &'static str = "1.2";
const DEFAULT_VERSION: &'static str = "1";

#[derive(PartialEq)]
pub enum CycloneDXEncodeType {
    XML, JSON
}

#[derive(Debug)]
pub struct CycloneDXEncodeError {}
impl Error for CycloneDXEncodeError {}
impl fmt::Display for CycloneDXEncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error encoding CycloneDX BOM")
    }
}

#[yaserde(rename = "bom", rename_all = "camelCase")]
#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct XMLCycloneDX {
    #[yaserde(rename = "serialNumber", attribute)]
    serial_number: String,
    #[yaserde(attribute)]
    version: String,
    #[yaserde(attribute)]
    xmlns: String,

    cyclonedx: CycloneDX
}

impl XMLCycloneDX {
    pub fn new() -> XMLCycloneDX {
        XMLCycloneDX {
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),
            xmlns: XMLNS.to_string(),
            cyclonedx: Default::default()
        }
    }
}


#[serde(rename_all = "camelCase")]
#[derive(Default, PartialEq, Debug, Serialize)]
pub struct JSONCycloneDX {
    bom_format: String,
    spec_version: String,
    serial_number: String,
    version: String,

    cyclonedx: CycloneDX
}

impl JSONCycloneDX {
    pub fn new() -> JSONCycloneDX {
        JSONCycloneDX {
            bom_format: BOM_FORMAT.to_string(),
            spec_version: SPEC_VERSION.to_string(),
            serial_number: "urn:uuid:".to_owned() + &uuid::Uuid::new_v4().to_string(),
            version: DEFAULT_VERSION.to_string(),

            cyclonedx: Default::default()
        }
    }
}

#[derive(PartialEq, Debug, Serialize, YaSerialize)]
pub struct Metadata {
    time_stamp: String
}

impl Metadata {
    pub fn new() -> Metadata {
        let time_stamp: DateTime<Utc> = SystemTime::now().into();
        Metadata {
            time_stamp: time_stamp.to_rfc3339()
        }
    }
}
// #[serde(rename = "bom", rename_all = "camelCase")]
#[derive(Default, PartialEq, Debug, Serialize, YaSerialize)]
#[yaserde(flatten)]
pub struct CycloneDX {
    metadata: Option<Metadata>,
}

impl CycloneDX {
    pub fn new() -> CycloneDX {
        CycloneDX {
            metadata: None
        }
    }

    pub fn encode<W>(writer: W, dx: CycloneDX, format: CycloneDXEncodeType) -> Result<(), CycloneDXEncodeError>
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
                    indent_string: None
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
    use crate::{CycloneDX, Metadata};
    use std::io::ErrorKind;
    use crate::CycloneDXEncodeType::{JSON, XML};

    // #[test]
    // fn new_bom_has_defaults() {
    //     let bom = CycloneDX::new();
    //
    //     assert_eq!(bom.version, "1");
    // }

    #[test]
    fn error_if_invalid_writer() {
        let cyclone_dx = CycloneDX::new();

        impl std::io::Write for CycloneDX {
            fn write(&mut self, _buf: &[u8]) -> Result<usize, std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }

            fn flush(&mut self) -> Result<(), std::io::Error> {
                return Err(std::io::Error::new(ErrorKind::BrokenPipe, ""));
            }
        }

        // Used to to get access to the dummy Write trait above
        let writer = Box::new(CycloneDX::new());
        let result = CycloneDX::encode(writer, cyclone_dx, JSON);

        assert!(result.is_err());
    }

    #[test]
    fn can_serialize_json() {
        let mut vec = Vec::new();
        let mut cyclone_dx = CycloneDX::new();
        cyclone_dx.metadata = Option::from(Metadata::new());

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
        let mut cyclone_dx = CycloneDX::new();
        cyclone_dx.metadata = Option::from(Metadata::new());

        let result = CycloneDX::encode(&mut vec, cyclone_dx, XML);

        let actual = String::from_utf8(vec).unwrap();
        let expected = r#"
        <bom version="1">
        <bom>"#;

        assert!(result.is_ok());
        assert_eq!(actual, expected);
    }

    fn remove_all_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
