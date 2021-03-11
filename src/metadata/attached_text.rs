use derive_builder::Builder;
use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Write;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::writer::XmlEvent;
use yaserde::ser::Serializer;
use yaserde::YaSerialize;
use yaserde_derive::YaSerialize;

#[derive(Clone, Default, Builder, PartialEq, Debug, Serialize)]
pub struct AttachedTextType {
    #[serde(rename = "content-type")]
    content_type: Option<String>,

    encoding: Option<BomEncoding>,

    value: String,
}

impl YaSerialize for AttachedTextType {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        writer.write(XmlEvent::characters(self.value.as_str()));
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

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum BomEncoding {
    Base64,
}

impl Display for BomEncoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BomEncoding::Base64 => write!(f, "base64"),
        }
    }
}
