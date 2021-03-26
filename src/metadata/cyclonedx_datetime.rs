use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::io::Write;
use std::time::SystemTime;
use xml::attribute::OwnedAttribute;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::ser::Serializer;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CycloneDxDateTime {
    pub(crate) date: DateTime<Utc>,
}

impl YaDeserialize for CycloneDxDateTime {
    fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
        use std::str::FromStr;

        loop {
            match reader.next_event()? {
                XmlEvent::StartElement { .. } => {}
                XmlEvent::Characters(ref content) => {
                    return Ok(CycloneDxDateTime {
                        date: DateTime::from_str(content).unwrap(),
                    })
                }
                _ => {
                    break;
                }
            }
        }

        Err("Unable to parse cycloneDxDateTime".to_string())
    }
}

impl YaSerialize for CycloneDxDateTime {
    fn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {
        let _ret = writer.write(xml::writer::XmlEvent::start_element("timestamp"));
        let _ret = writer.write(xml::writer::XmlEvent::characters(&(self.date.to_rfc3339())));
        let _ret = writer.write(xml::writer::XmlEvent::end_element());
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

impl Default for CycloneDxDateTime {
    fn default() -> Self {
        CycloneDxDateTime {
            date: SystemTime::now().into(),
        }
    }
}
