use super::ParameterValues;
use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
#[schemars(transform = crate::json_schema::indexed_array_transform)]
pub struct IndexedArrayParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "params")]
    pub parameters: ParameterValues,
    pub index_parameter: ParameterValue,
}

impl IndexedArrayParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
