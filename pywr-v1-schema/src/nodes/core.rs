use crate::nodes::NodeMeta;
use crate::parameters::{
    ParameterValue, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_v1_schema_macros::PywrNode;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct InputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
}

impl InputNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct LinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
}

impl LinkNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct OutputNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
}

impl OutputNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct StorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
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

impl StorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct ReservoirNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
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

impl ReservoirNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct CatchmentNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<ParameterValue>,
}

impl CatchmentNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone, PywrNode)]
pub struct AggregatedNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub nodes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_flow: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<ParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_weights: Option<Vec<f64>>,
}

impl AggregatedNode {
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
pub struct AggregatedStorageNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    pub storage_nodes: Vec<String>,
}

impl AggregatedStorageNode {
    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        vec![(
            "storage_nodes",
            self.storage_nodes
                .iter()
                .map(|n| n.as_str())
                .collect::<Vec<&str>>(),
        )]
        .into_iter()
        .collect()
    }
}
