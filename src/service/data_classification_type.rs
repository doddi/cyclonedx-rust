use crate::service::data_flow_type::DataFlowType;
use derive_builder::Builder;
use serde::Serialize;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Builder, PartialEq, Debug, Serialize, YaSerialize, YaDeserialize)]
pub struct DataClassificationType {
    #[yaserde(attribute)]
    flow: DataFlowType,
    #[yaserde(text)]
    value: String,
}
