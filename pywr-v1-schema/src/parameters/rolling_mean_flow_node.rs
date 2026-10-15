use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct RollingMeanFlowNodeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub timesteps: Option<i64>,
    pub days: Option<i64>,
    pub initial_flow: Option<f64>,
}

impl RollingMeanFlowNodeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
}
