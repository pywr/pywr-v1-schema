use crate::parameters::{
    ExternalDataRef, ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut,
    TableDataRef,
};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
#[schemars(transform = crate::json_schema::constant_parameter_transform)]
pub struct ConstantParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(alias = "values", skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDataRef>,
}

impl ConstantParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ConstantScenarioParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub scenario: String,
    #[serde(alias = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub table: Option<TableDataRef>,
}

impl ConstantScenarioParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct MaxParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl MaxParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct NegativeParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
}

impl NegativeParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct MinParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl MinParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct NegativeMinParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl NegativeMinParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct NegativeMaxParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: Option<f64>,
}

impl NegativeMaxParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct DivisionParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub numerator: ParameterValue,
    pub denominator: ParameterValue,
}

impl DivisionParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
