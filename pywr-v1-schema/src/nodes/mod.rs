mod break_link;
mod core;
mod delay_node;
mod loss_link;
mod multi_split;
mod piecewise_link;
mod river_gauge;
mod river_split;
mod river_split_with_gauge;
mod virtual_storage;

pub use crate::nodes::core::{
    AggregatedNode, AggregatedStorageNode, CatchmentNode, InputNode, LinkNode, OutputNode,
    ReservoirNode, StorageNode,
};
pub use crate::nodes::river_split_with_gauge::RiverSplitWithGaugeNode;
use crate::parameters::{ParameterValueType, ParameterValueTypeMut};
pub use break_link::BreakLinkNode;
pub use delay_node::DelayNode;
pub use loss_link::LossLinkNode;
pub use multi_split::MultiSplitLinkNode;
pub use piecewise_link::PiecewiseLinkNode;
pub use river_gauge::RiverGaugeNode;
pub use river_split::RiverSplitNode;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use strum_macros::VariantNames;
pub use virtual_storage::{
    AnnualVirtualStorageNode, MonthlyVirtualStorageNode, RollingVirtualStorageNode,
    SeasonalVirtualStorageNode, VirtualStorageNode,
};

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub struct NodePosition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schematic: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic: Option<(f32, f32)>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub struct NodeMeta {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<NodePosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, Value>>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone)]
pub struct CustomNode {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(flatten)]
    pub meta: NodeMeta,
    #[serde(flatten)]
    pub attributes: HashMap<String, Value>,
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, VariantNames, Clone)]
#[serde(tag = "type")]
#[schemars(transform = crate::json_schema::core_node_transform)]
pub enum CoreNode {
    #[serde(alias = "input")]
    Input(InputNode),
    #[serde(alias = "link")]
    Link(LinkNode),
    #[serde(alias = "output")]
    Output(OutputNode),
    #[serde(alias = "storage")]
    Storage(StorageNode),
    #[serde(alias = "reservoir")]
    Reservoir(ReservoirNode),
    #[serde(alias = "catchment")]
    Catchment(CatchmentNode),
    #[serde(alias = "rivergauge", alias = "Rivergauge")]
    RiverGauge(RiverGaugeNode),
    #[serde(alias = "losslink", alias = "Losslink")]
    LossLink(LossLinkNode),
    #[serde(alias = "river")]
    River(LinkNode), // TODO make this its own type.
    #[serde(alias = "piecewiselink", alias = "Piecewiselink")]
    PiecewiseLink(PiecewiseLinkNode),
    #[serde(alias = "multisplitlink", alias = "Multisplitlink")]
    MultiSplitLink(MultiSplitLinkNode),
    #[serde(alias = "breaklink")]
    BreakLink(BreakLinkNode),
    #[serde(alias = "delaynode", alias = "DelayNode", alias = "Delaynode")]
    Delay(DelayNode),
    #[serde(alias = "riversplit", alias = "Riversplit")]
    RiverSplit(RiverSplitNode),
    #[serde(alias = "riversplitwithgauge", alias = "Riversplitwithgauge")]
    RiverSplitWithGauge(RiverSplitWithGaugeNode),
    #[serde(alias = "aggregatednode", alias = "AggregatedNode")]
    Aggregated(AggregatedNode),
    #[serde(alias = "aggregatedstorage", alias = "Aggregatedstorage")]
    AggregatedStorage(AggregatedStorageNode),
    #[serde(alias = "virtualstorage", alias = "Virtualstorage")]
    VirtualStorage(VirtualStorageNode),
    #[serde(alias = "annualvirtualstorage", alias = "Annualvirtualstorage")]
    AnnualVirtualStorage(AnnualVirtualStorageNode),
    #[serde(alias = "monthlyvirtualstorage", alias = "Monthlyvirtualstorage")]
    MonthlyVirtualStorage(MonthlyVirtualStorageNode),
    #[serde(alias = "seasonalvirtualstorage", alias = "Seasonalvirtualstorage")]
    SeasonalVirtualStorage(SeasonalVirtualStorageNode),
    #[serde(alias = "rollingvirtualstorage", alias = "Rollingvirtualstorage")]
    RollingVirtualStorage(RollingVirtualStorageNode),
}

impl CoreNode {
    pub fn name(&self) -> &str {
        self.meta().name.as_str()
    }

    pub fn position(&self) -> Option<&NodePosition> {
        self.meta().position.as_ref()
    }

    pub fn node_type(&self) -> &str {
        match self {
            CoreNode::Input(_) => "input",
            CoreNode::Link(_) => "link",
            CoreNode::Output(_) => "output",
            CoreNode::Storage(_) => "storage",
            CoreNode::Reservoir(_) => "reservoir",
            CoreNode::Catchment(_) => "catchment",
            CoreNode::RiverGauge(_) => "rivergauge",
            CoreNode::LossLink(_) => "losslink",
            CoreNode::PiecewiseLink(_) => "piecewiselink",
            CoreNode::MultiSplitLink(_) => "multisplitlink",
            CoreNode::BreakLink(_) => "breaklink",
            CoreNode::Delay(_) => "delaynode",
            CoreNode::River(_) => "river",
            CoreNode::RiverSplit(_) => "riversplit",
            CoreNode::RiverSplitWithGauge(_) => "riversplitwithgauge",
            CoreNode::Aggregated(_) => "aggregated",
            CoreNode::AggregatedStorage(_) => "aggregatedstorage",
            CoreNode::VirtualStorage(_) => "virtualstorage",
            CoreNode::AnnualVirtualStorage(_) => "annualvirtualstorage",
            CoreNode::MonthlyVirtualStorage(_) => "monthlyvirtualstorage",
            CoreNode::SeasonalVirtualStorage(_) => "seasonalvirtualstorage",
            CoreNode::RollingVirtualStorage(_) => "rollingvirtualstorage",
        }
    }

    pub fn meta(&self) -> &NodeMeta {
        match self {
            CoreNode::Input(n) => &n.meta,
            CoreNode::Link(n) => &n.meta,
            CoreNode::Output(n) => &n.meta,
            CoreNode::Storage(n) => &n.meta,
            CoreNode::Reservoir(n) => &n.meta,
            CoreNode::Catchment(n) => &n.meta,
            CoreNode::RiverGauge(n) => &n.meta,
            CoreNode::LossLink(n) => &n.meta,
            CoreNode::PiecewiseLink(n) => &n.meta,
            CoreNode::MultiSplitLink(n) => &n.meta,
            CoreNode::BreakLink(n) => &n.meta,
            CoreNode::Delay(n) => &n.meta,
            CoreNode::River(n) => &n.meta,
            CoreNode::RiverSplit(n) => &n.meta,
            CoreNode::RiverSplitWithGauge(n) => &n.meta,
            CoreNode::Aggregated(n) => &n.meta,
            CoreNode::AggregatedStorage(n) => &n.meta,
            CoreNode::VirtualStorage(n) => &n.meta,
            CoreNode::AnnualVirtualStorage(n) => &n.meta,
            CoreNode::MonthlyVirtualStorage(n) => &n.meta,
            CoreNode::SeasonalVirtualStorage(n) => &n.meta,
            CoreNode::RollingVirtualStorage(n) => &n.meta,
        }
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType<'_>> {
        match self {
            CoreNode::Input(n) => n.parameters(),
            CoreNode::Link(n) => n.parameters(),
            CoreNode::Output(n) => n.parameters(),
            CoreNode::Storage(n) => n.parameters(),
            CoreNode::Reservoir(n) => n.parameters(),
            CoreNode::Catchment(n) => n.parameters(),
            CoreNode::RiverGauge(n) => n.parameters(),
            CoreNode::LossLink(n) => n.parameters(),
            CoreNode::PiecewiseLink(n) => n.parameters(),
            CoreNode::MultiSplitLink(n) => n.parameters(),
            CoreNode::BreakLink(n) => n.parameters(),
            CoreNode::Delay(n) => n.parameters(),
            CoreNode::River(n) => n.parameters(),
            CoreNode::RiverSplit(n) => n.parameters(),
            CoreNode::RiverSplitWithGauge(n) => n.parameters(),
            CoreNode::Aggregated(n) => n.parameters(),
            CoreNode::AggregatedStorage(n) => n.parameters(),
            CoreNode::VirtualStorage(n) => n.parameters(),
            CoreNode::AnnualVirtualStorage(n) => n.parameters(),
            CoreNode::MonthlyVirtualStorage(n) => n.parameters(),
            CoreNode::SeasonalVirtualStorage(n) => n.parameters(),
            CoreNode::RollingVirtualStorage(n) => n.parameters(),
        }
    }

    pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut<'_>> {
        match self {
            CoreNode::Input(n) => n.parameters_mut(),
            CoreNode::Link(n) => n.parameters_mut(),
            CoreNode::Output(n) => n.parameters_mut(),
            CoreNode::Storage(n) => n.parameters_mut(),
            CoreNode::Reservoir(n) => n.parameters_mut(),
            CoreNode::Catchment(n) => n.parameters_mut(),
            CoreNode::RiverGauge(n) => n.parameters_mut(),
            CoreNode::LossLink(n) => n.parameters_mut(),
            CoreNode::PiecewiseLink(n) => n.parameters_mut(),
            CoreNode::MultiSplitLink(n) => n.parameters_mut(),
            CoreNode::BreakLink(n) => n.parameters_mut(),
            CoreNode::Delay(n) => n.parameters_mut(),
            CoreNode::River(n) => n.parameters_mut(),
            CoreNode::RiverSplit(n) => n.parameters_mut(),
            CoreNode::RiverSplitWithGauge(n) => n.parameters_mut(),
            CoreNode::Aggregated(n) => n.parameters_mut(),
            CoreNode::AggregatedStorage(n) => n.parameters_mut(),
            CoreNode::VirtualStorage(n) => n.parameters_mut(),
            CoreNode::AnnualVirtualStorage(n) => n.parameters_mut(),
            CoreNode::MonthlyVirtualStorage(n) => n.parameters_mut(),
            CoreNode::SeasonalVirtualStorage(n) => n.parameters_mut(),
            CoreNode::RollingVirtualStorage(n) => n.parameters_mut(),
        }
    }

    pub fn node_references(&self) -> HashMap<&str, Vec<&str>> {
        match self {
            CoreNode::Input(n) => n.node_references(),
            CoreNode::Link(n) => n.node_references(),
            CoreNode::Output(n) => n.node_references(),
            CoreNode::Storage(n) => n.node_references(),
            CoreNode::Reservoir(n) => n.node_references(),
            CoreNode::Catchment(n) => n.node_references(),
            CoreNode::RiverGauge(n) => n.node_references(),
            CoreNode::LossLink(n) => n.node_references(),
            CoreNode::PiecewiseLink(n) => n.node_references(),
            CoreNode::MultiSplitLink(n) => n.node_references(),
            CoreNode::BreakLink(n) => n.node_references(),
            CoreNode::Delay(n) => n.node_references(),
            CoreNode::River(n) => n.node_references(),
            CoreNode::RiverSplit(n) => n.node_references(),
            CoreNode::RiverSplitWithGauge(n) => n.node_references(),
            CoreNode::Aggregated(n) => n.node_references(),
            CoreNode::AggregatedStorage(n) => n.node_references(),
            CoreNode::VirtualStorage(n) => n.node_references(),
            CoreNode::AnnualVirtualStorage(n) => n.node_references(),
            CoreNode::MonthlyVirtualStorage(n) => n.node_references(),
            CoreNode::SeasonalVirtualStorage(n) => n.node_references(),
            CoreNode::RollingVirtualStorage(n) => n.node_references(),
        }
    }

    pub fn is_virtual(&self) -> bool {
        matches!(
            self,
            CoreNode::Aggregated(_)
                | CoreNode::AggregatedStorage(_)
                | CoreNode::VirtualStorage(_)
                | CoreNode::AnnualVirtualStorage(_)
                | CoreNode::MonthlyVirtualStorage(_)
                | CoreNode::SeasonalVirtualStorage(_)
                | CoreNode::RollingVirtualStorage(_)
        )
    }
}

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone)]
#[serde(untagged)]
pub enum Node {
    Core(Box<CoreNode>),
    Custom(CustomNode),
}

impl Node {
    pub fn name(&self) -> &str {
        match self {
            Node::Core(n) => n.name(),
            Node::Custom(n) => n.meta.name.as_str(),
        }
    }

    pub fn position(&self) -> Option<&NodePosition> {
        match self {
            Node::Core(n) => n.position(),
            Node::Custom(n) => n.meta.position.as_ref(),
        }
    }

    pub fn node_type(&self) -> &str {
        match self {
            Node::Core(n) => n.node_type(),
            Node::Custom(n) => n.ty.as_str(),
        }
    }

    pub fn meta(&self) -> &NodeMeta {
        match self {
            Node::Core(n) => n.meta(),
            Node::Custom(n) => &n.meta,
        }
    }

    pub fn parameters(&self) -> HashMap<&str, ParameterValueType<'_>> {
        match self {
            Node::Core(n) => n.parameters(),
            Node::Custom(_) => HashMap::new(),
        }
    }

    pub fn parameters_mut(&mut self) -> HashMap<&str, ParameterValueTypeMut<'_>> {
        match self {
            Node::Core(n) => n.parameters_mut(),
            Node::Custom(_) => HashMap::new(),
        }
    }

    pub fn resource_paths(&self) -> Vec<PathBuf> {
        let mut resource_paths = Vec::new();

        for (_, p) in self.parameters() {
            let paths = match p {
                ParameterValueType::Single(p) => p.resource_paths(),
                ParameterValueType::List(p) => p.iter().flat_map(|p| p.resource_paths()).collect(),
                ParameterValueType::OptionalList(p) => p
                    .iter()
                    .flat_map(|p| p.as_ref().map(|p| p.resource_paths()))
                    .flatten()
                    .collect(),
            };

            resource_paths.extend(paths);
        }

        resource_paths
    }

    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        for (_, p) in self.parameters_mut() {
            match p {
                ParameterValueTypeMut::Single(p) => p.update_resource_paths(new_paths),
                ParameterValueTypeMut::List(p) => {
                    for p in p.iter_mut() {
                        p.update_resource_paths(new_paths);
                    }
                }
                ParameterValueTypeMut::OptionalList(p) => {
                    for p in p.iter_mut().flatten() {
                        p.update_resource_paths(new_paths);
                    }
                }
            };
        }
    }
}
