use crate::parameters::{ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, PartialEq, Debug, Clone, Copy,
)]
#[schemars(transform = crate::json_schema::predicate_transform)]
pub enum Predicate {
    #[serde(alias = "<", alias = "lt", alias = "LT")]
    LT,
    #[serde(alias = ">", alias = "gt", alias = "GT")]
    GT,
    #[serde(alias = "==", alias = "eq", alias = "EQ")]
    EQ,
    #[serde(alias = "<=", alias = "le", alias = "LE")]
    LE,
    #[serde(alias = ">=", alias = "ge", alias = "GE")]
    GE,
}

fn default_predicate() -> Predicate {
    Predicate::LT
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ParameterThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    #[serde(default = "default_predicate")]
    pub predicate: Predicate,
}

impl ParameterThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct NodeThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    #[serde(default = "default_predicate")]
    pub predicate: Predicate,
}

impl NodeThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct StorageThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub storage_node: String,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    #[serde(default = "default_predicate")]
    pub predicate: Predicate,
}

impl StorageThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct MultipleThresholdIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub node: String,
    pub thresholds: Vec<ParameterValue>,
}

impl MultipleThresholdIndexParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct MultipleThresholdParameterIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub parameter: ParameterValue,
    pub thresholds: Vec<ParameterValue>,
}

impl MultipleThresholdParameterIndexParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct CurrentYearThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    #[serde(default = "default_predicate")]
    pub predicate: Predicate,
}

impl CurrentYearThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct CurrentOrdinalDayThresholdParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub threshold: ParameterValue,
    pub values: Option<Vec<f64>>,
    #[serde(default = "default_predicate")]
    pub predicate: Predicate,
}

impl CurrentOrdinalDayThresholdParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        HashMap::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::{
        thresholds::{
            CurrentOrdinalDayThresholdParameter, CurrentYearThresholdParameter,
            MultipleThresholdIndexParameter, MultipleThresholdParameterIndexParameter,
            NodeThresholdParameter, StorageThresholdParameter,
        },
        ParameterThresholdParameter, ParameterValue, Predicate,
    };

    #[test]
    fn test_param() {
        let data = r#"
            {
                "type": "parameterthreshold",
                "parameter": "Param1",
                "threshold": 5.0,
                "predicate": ">=",
                "values": [
                    2.0,
                    0
                ]
            }
            "#;
        let param: ParameterThresholdParameter = serde_json::from_str(data).unwrap();

        match param.predicate {
            Predicate::GE => {}
            _ => panic!("Predicate is not correct"),
        }

        match param.parameter {
            ParameterValue::Reference(val) => {
                assert_eq!(val, "Param1");
            }
            _ => panic!("Parameter is not a reference"),
        }

        match param.threshold {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 5.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        assert_eq!(param.values, Some(vec![2.0, 0.0]));
    }

    #[test]
    fn test_node() {
        let data = r#"
            {
                "type": "nodethreshold",
                "node": "Gauge1",
                "threshold": 5.0,
                "values": [
                    2.0,
                    0
                ]
            }
            "#;
        let param: NodeThresholdParameter = serde_json::from_str(data).unwrap();

        match param.predicate {
            Predicate::LT => {}
            _ => panic!("Predicate is not correct"),
        }

        assert_eq!(param.node, "Gauge1");

        match param.threshold {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 5.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        assert_eq!(param.values, Some(vec![2.0, 0.0]));
    }

    #[test]
    fn test_storage() {
        let data = r#"
            {
                "type": "storagethreshold",
                "storage_node": "Res1",
                "threshold": 1000.0,
                "predicate": ">",
                "values": [
                    10.0,
                    0
                ]
            }
            "#;
        let param: StorageThresholdParameter = serde_json::from_str(data).unwrap();

        match param.predicate {
            Predicate::GT => {}
            _ => panic!("Predicate is not correct"),
        }

        assert_eq!(param.storage_node, "Res1");

        match param.threshold {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 1000.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        assert_eq!(param.values, Some(vec![10.0, 0.0]));
    }

    #[test]
    fn test_multi_thresholds() {
        let data = r#"
            {
                "type": "multiplethresholdindex",
                "node": "Gauge1",
                "thresholds": [2.0, 7.0]
            }
            "#;
        let param: MultipleThresholdIndexParameter = serde_json::from_str(data).unwrap();

        assert_eq!(param.node, "Gauge1");

        match param.thresholds[0] {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 2.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        match param.thresholds[1] {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 7.0);
            }
            _ => panic!("Threshold is not a constant"),
        }
    }

    #[test]
    fn test_multi_thresholds_param() {
        let data = r#"
            {
                "type": "multiplethresholdparameterindex",
                "parameter": "Param1",
                "thresholds": [2.0, 7.0]
            }
            "#;
        let param: MultipleThresholdParameterIndexParameter = serde_json::from_str(data).unwrap();

        match param.parameter {
            ParameterValue::Reference(val) => {
                assert_eq!(val, "Param1");
            }
            _ => panic!("Parameter is not a reference"),
        }
        match param.thresholds[0] {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 2.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        match param.thresholds[1] {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 7.0);
            }
            _ => panic!("Threshold is not a constant"),
        }
    }

    #[test]
    fn test_current_year() {
        let data = r#"
            {
                "type": "currentyearthreshold",
                "threshold": 5.0,
                "values": [
                    2.0,
                    0
                ]
            }
            "#;
        let param: CurrentYearThresholdParameter = serde_json::from_str(data).unwrap();

        match param.predicate {
            Predicate::LT => {}
            _ => panic!("Predicate is not correct"),
        }

        match param.threshold {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 5.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        assert_eq!(param.values, Some(vec![2.0, 0.0]));
    }

    #[test]
    fn test_current_ordinal_day() {
        let data = r#"
            {
                "type": "currentordinaldaythreshold",
                "threshold": 5.0,
                "values": [
                    2.0,
                    0
                ]
            }
            "#;
        let param: CurrentOrdinalDayThresholdParameter = serde_json::from_str(data).unwrap();

        match param.predicate {
            Predicate::LT => {}
            _ => panic!("Predicate is not correct"),
        }

        match param.threshold {
            ParameterValue::Constant(val) => {
                assert_eq!(val, 5.0);
            }
            _ => panic!("Threshold is not a constant"),
        }

        assert_eq!(param.values, Some(vec![2.0, 0.0]));
    }
}
