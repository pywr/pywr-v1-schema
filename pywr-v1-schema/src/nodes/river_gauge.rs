use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrNode)]
pub struct RiverGaugeNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrf: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrf_cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
}

impl RiverGaugeNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
