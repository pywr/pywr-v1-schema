use crate::parameters::{
    ExternalDataRef, ParameterMeta, ParameterValueType, ParameterValueTypeMut, TableDataRef,
};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct DailyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<f64>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub table_ref: Option<TableDataRef>,
}

impl DailyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MonthInterpDay {
    First,
    Last,
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct MonthlyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub interp_day: Option<MonthInterpDay>,
    pub values: Option<[f64; 12]>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten)]
    pub table_ref: Option<TableDataRef>,
}

impl MonthlyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct UniformDrawdownProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub reset_day: Option<u8>,
    pub reset_month: Option<u8>,
    pub residual_days: Option<u32>,
}

impl UniformDrawdownProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct WeeklyProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub values: Option<Vec<f64>>,
    #[serde(flatten)]
    pub external: Option<ExternalDataRef>,
    #[serde(flatten)]
    pub table_ref: Option<TableDataRef>,
}

impl WeeklyProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct RbfProfileParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub days_of_year: Vec<u32>,
    pub values: Vec<f64>,
    pub lower_bounds: Option<f64>,
    pub upper_bounds: Option<f64>,
    pub variable_days_of_year_range: Option<u32>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub rbf_kwargs: HashMap<String, serde_json::Value>,
}

impl RbfProfileParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}
