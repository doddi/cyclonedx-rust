use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub enum DataFlowType {
    #[serde(rename = "inbound")]
    #[yaserde(rename = "inbound")]
    InBound,
    #[serde(rename = "outbound")]
    #[yaserde(rename = "outbound")]
    Outbound,
    #[serde(rename = "bi-directional")]
    #[yaserde(rename = "bi-directional")]
    BiDirectional,
    #[serde(rename = "unknown")]
    #[yaserde(rename = "unknown")]
    Unknown,
}

impl Default for DataFlowType {
    fn default() -> Self {
        DataFlowType::Unknown
    }
}
