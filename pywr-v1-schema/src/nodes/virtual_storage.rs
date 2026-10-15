use crate::nodes::NodeMeta;
use crate::parameters::{ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct VirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume_pc: Option<f64>,
}

impl VirtualStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_reset_day() -> u32 {
    1
}

fn default_reset_month() -> u32 {
    1
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct AnnualVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_reset_day")]
    pub reset_day: u32,
    #[serde(default = "default_reset_month")]
    pub reset_month: u32,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl AnnualVirtualStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_months() -> u8 {
    1
}

fn default_initial_months() -> u8 {
    1
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct MonthlyVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_months")]
    pub months: u8,
    #[serde(default = "default_initial_months")]
    pub initial_months: u8,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl MonthlyVirtualStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

fn default_end_day() -> u32 {
    31
}

fn default_end_month() -> u32 {
    12
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct SeasonalVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume_pc: Option<f64>,
    #[serde(default = "default_reset_day")]
    pub reset_day: u32,
    #[serde(default = "default_reset_month")]
    pub reset_month: u32,
    #[serde(default = "default_end_day")]
    pub end_day: u32,
    #[serde(default = "default_end_month")]
    pub end_month: u32,
    #[serde(default)]
    pub reset_to_initial_volume: bool,
}

impl SeasonalVirtualStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct RollingVirtualStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_volume_pc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timesteps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

impl RollingVirtualStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "nodes",
            self.nodes.iter().map(|n| n.as_str()).collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::AnnualVirtualStorageNode;

    #[test]
    fn test_annual_virtual_storage() {
        let data = r#"
            {
              "type": "annualvirtualstorage",
              "name": "Scales AL",
              "max_volume": 365,
              "nodes": [
                "Scales BHs"
              ],
              "initial_volume": 365
            }
            "#;

        let node: AnnualVirtualStorageNode = serde_json::from_str(data).unwrap();

        assert_eq!(node.meta.name, "Scales AL");
    }
}
