use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct TablesArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    #[serde(rename = "where")]
    pub wh: String,
    pub scenario: Option<String>,
    pub checksum: Option<HashMap<String, String>>,
    pub url: PathBuf,
}

impl TablesArrayParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
