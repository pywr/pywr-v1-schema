use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct Polynomial1DParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub storage_node: String,
    pub coefficients: Vec<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_proportional_volume: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

impl Polynomial1DParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}
