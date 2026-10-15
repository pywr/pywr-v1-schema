use crate::parameters::{
    ParameterMeta, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ScenarioWrapperParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub scenario: String,
    pub parameters: ParameterValues,
}

impl ScenarioWrapperParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
