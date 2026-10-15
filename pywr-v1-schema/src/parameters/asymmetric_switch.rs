use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct AsymmetricSwitchIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub on_index_parameter: ParameterValue,
    pub off_index_parameter: ParameterValue,
}

impl AsymmetricSwitchIndexParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
