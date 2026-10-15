use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct DataFrameParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestep_offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub pandas_kwargs: HashMap<String, serde_json::Value>,
}

impl DataFrameParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
