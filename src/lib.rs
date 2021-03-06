use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

const XML_NAMESPACE: &'static str = "http://cyclonedx.org/schema/bom/1.2";
const BOM_FORMAT: &'static str = "CycloneDX";
const SPEC_VERSION: &'static str = "1.2";
const DEFAULT_VERSION: &'static str = "1";

#[derive(Debug)]
pub struct CycloneDXEncodeError {}
impl Error for CycloneDXEncodeError {}
impl fmt::Display for CycloneDXEncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error encoding CycloneDX BOM")
    }
}

#[derive(Serialize, Deserialize)]
pub struct CycloneDX {
    xmlns: &'static str,

    #[serde(rename = "bomFormat")]
    bom_format: &'static str,

    #[serde(rename = "specVersion")]
    spec_version: &'static str,

    version: &'static str,
}

impl CycloneDX {
    pub fn new() -> CycloneDX {
        CycloneDX {
            xmlns: XML_NAMESPACE,
            bom_format: BOM_FORMAT,
            spec_version: SPEC_VERSION,
            version: DEFAULT_VERSION,
        }
    }

    pub fn encode<W>(&self, writer: W) -> Result<(), CycloneDXEncodeError>
    where
        W: std::io::Write,
    {
        let result = serde_json::to_writer_pretty(writer, self);

        if result.is_err() {
            return Err(CycloneDXEncodeError {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::CycloneDX;
    use std::io::ErrorKind;

    #[test]
    fn new_bom_has_defaults() {
        let bom = CycloneDX::new();

        assert_eq!(bom.xmlns, "http://cyclonedx.org/schema/bom/1.2");
        assert_eq!(bom.bom_format, "CycloneDX");
        assert_eq!(bom.spec_version, "1.2");
        assert_eq!(bom.version, "1");
    }

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
        let result = cyclone_dx.encode(writer);

        assert!(result.is_err());
    }

    #[test]
    fn can_serialize_json() {
        let mut vec = Vec::new();
        let buf = Box::new(&mut vec);
        let cyclone_dx = CycloneDX::new();

        let result = cyclone_dx.encode(buf);

        let actual = String::from_utf8(vec).unwrap();
        let expected = r#"
        {
            "xmlns": "http://cyclonedx.org/schema/bom/1.2",
            "bomFormat": "CycloneDX",
            "specVersion": "1.2",
            "version": "1"
        }"#;

        assert!(result.is_ok());
        assert_eq!(
            remove_all_whitespace(actual.as_ref()),
            remove_all_whitespace(expected)
        );
    }

    fn remove_all_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
