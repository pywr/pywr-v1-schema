use crate::nodes::NodeMeta;
use crate::parameters::{
    OptionalParameterValues, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrNode)]
pub struct RiverSplitNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flows: Option<OptionalParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<ParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_names: Option<Vec<String>>,
    pub factors: ParameterValues,
}

impl RiverSplitNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
