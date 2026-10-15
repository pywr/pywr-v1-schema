use crate::parameters::{
    ParameterMeta, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct InterpolatedVolumeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub volumes: ParameterValues,
    pub values: ParameterValues,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedVolumeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct InterpolatedFlowParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub flows: ParameterValues,
    pub values: ParameterValues,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interp_kwargs: Option<HashMap<String, serde_json::Value>>,
}

impl InterpolatedFlowParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("node", self.node.as_str())].into_iter().collect()
    }
}
