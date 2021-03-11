use crate::service::data_flow_type::DataFlowType;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, Deserialize, YaSerialize, YaDeserialize)]
pub struct DataClassificationType {
    #[yaserde(attribute)]
    pub flow: DataFlowType,
    #[yaserde(text)]
    pub value: String,
}
