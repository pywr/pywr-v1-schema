use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrNode)]
pub struct DelayNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timesteps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_flow: Option<f64>,
}

impl DelayNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
