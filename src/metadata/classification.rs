use derive_builder::Builder;
use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};
use yaserde_derive::YaSerialize;

#[derive(Clone, PartialEq, Debug, Serialize, YaSerialize)]
pub enum Classification {
    Application,
    Framework,
    Library,
    Container,
    OperatingSystem,
    Device,
    Firmware,
    File,
}

impl Display for Classification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Classification::Application => write!(f, "application"),
            Classification::Framework => write!(f, "framework"),
            Classification::Library => write!(f, "library"),
            Classification::Container => write!(f, "container"),
            Classification::OperatingSystem => write!(f, "operating-system"),
            Classification::Device => write!(f, "device"),
            Classification::Firmware => write!(f, "firmware"),
            Classification::File => write!(f, "file"),
        }
    }
}
