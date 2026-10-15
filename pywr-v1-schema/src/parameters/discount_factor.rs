use crate::parameters::{ParameterMeta, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct DiscountFactorParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub rate: f64,
    pub base_year: i64,
}

impl DiscountFactorParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
