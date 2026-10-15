use crate::nodes::NodeMeta;
use crate::parameters::{
    OptionalParameterValues, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub struct MultiSplitLinkNode {
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_flows: Option<OptionalParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub costs: Option<ParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_slots: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<ParameterValues>,
}

impl MultiSplitLinkNode {
    pub fn parameters(&self) -> HashMap<&str, ParameterValueType<'_>> {
        let mut attributes = HashMap::new();
        if let Some(max_flows) = &self.max_flows {
            attributes.insert("max_flows", max_flows.into());
        }
        if let Some(costs) = &self.costs {
            attributes.insert("costs", costs.into());
        }
        if let Some(factors) = &self.factors {
            attributes.insert("factors", factors.into());
        }
        attributes
    }

    pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut<'_>> {
        let mut attributes = HashMap::new();
        if let Some(max_flows) = &mut self.max_flows {
            attributes.insert("max_flows", max_flows.into());
        }
        if let Some(costs) = &mut self.costs {
            attributes.insert("costs", costs.into());
        }
        if let Some(factors) = &mut self.factors {
            attributes.insert("factors", factors.into());
        }
        attributes
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        HashMap::new()
    }
}
