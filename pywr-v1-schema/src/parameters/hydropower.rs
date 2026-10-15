use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct HydropowerTargetParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub target: ParameterValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_elevation_parameter: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turbine_elevation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efficiency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_unit_conversion: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_unit_conversion: Option<f64>,
}

impl HydropowerTargetParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
