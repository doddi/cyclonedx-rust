use heck::KebabCase;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use strum_macros::EnumString;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::writer::XmlEvent;
use yaserde::ser::Serializer;
use yaserde::YaSerialize;
use yaserde_derive::YaDeserialize;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaDeserialize, EnumString)]
#[serde(rename_all = "kebab-case")]
pub enum Classification {
    #[yaserde(rename = "required")]
    Application,
    #[yaserde(rename = "framework")]
    Framework,
    #[yaserde(rename = "library")]
    Library,
    #[yaserde(rename = "container")]
    Container,
    #[yaserde(rename = "operating-system")]
    OperatingSystem,
    #[yaserde(rename = "device")]
    Device,
    #[yaserde(rename = "firmware")]
    Firmware,
    #[yaserde(rename = "file")]
    File,
}

impl Default for Classification {
    fn default() -> Self {
        Classification::Application
    }
}

impl Display for Classification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl YaSerialize for Classification {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        let data = self.to_string();
        let _r = writer.write(XmlEvent::characters(&data.to_kebab_case()));
        Ok(())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<OwnedAttribute>,
        namespace: Namespace,
    ) -> Result<(Vec<OwnedAttribute>, Namespace), String> {
        Ok((attributes, namespace))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use yaserde::ser::Config;

    #[test]
    fn print_xml() {
        assert_eq!(
            "application",
            serialize_classification_as_xml(Classification::Application)
        );
        assert_eq!(
            "operating-system",
            serialize_classification_as_xml(Classification::OperatingSystem)
        );
        assert_eq!(
            "library",
            serialize_classification_as_xml(Classification::Library)
        );
        assert_eq!(
            "framework",
            serialize_classification_as_xml(Classification::Framework)
        );
        assert_eq!(
            "firmware",
            serialize_classification_as_xml(Classification::Firmware)
        );
        assert_eq!(
            "device",
            serialize_classification_as_xml(Classification::Device)
        );
        assert_eq!(
            "container",
            serialize_classification_as_xml(Classification::Container)
        );
        assert_eq!(
            "file",
            serialize_classification_as_xml(Classification::File)
        );
    }

    fn serialize_classification_as_xml(classification: Classification) -> String {
        let actual = yaserde::ser::to_string_with_config(
            &classification,
            &Config {
                perform_indent: false,
                write_document_declaration: false,
                indent_string: None,
            },
        )
        .unwrap();
        actual
    }
}
