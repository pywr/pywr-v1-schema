use crate::parameters::{
    ParameterMeta, ParameterValue, ParameterValueType, ParameterValueTypeMut, ParameterValues,
};
use pywr_v1_schema_macros::PywrParameter;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ControlCurveInterpolatedParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curve: Option<ParameterValue>,
    pub control_curves: Option<ParameterValues>,
    pub storage_node: String,
    pub values: Option<Vec<f64>>,
    pub parameters: Option<ParameterValues>,
}

impl ControlCurveInterpolatedParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ControlCurveIndexParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curves: ParameterValues,
    pub storage_node: String,
}

impl ControlCurveIndexParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ControlCurveParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    pub control_curve: Option<ParameterValue>,
    pub control_curves: Option<ParameterValues>,
    pub storage_node: String,
    pub values: Option<Vec<f64>>,
    pub parameters: Option<ParameterValues>,
}

impl ControlCurveParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, schemars::JsonSchema, Debug, Clone, PywrParameter,
)]
pub struct ControlCurvePiecewiseInterpolatedParameter {
    #[serde(flatten)]
    pub meta: Option<ParameterMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_curve: Option<ParameterValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_curves: Option<ParameterValues>,
    pub storage_node: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<[f64; 2]>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValues>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
}

impl ControlCurvePiecewiseInterpolatedParameter {
    pub fn node_references(&self) -> HashMap<&str, &str> {
        vec![("storage_node", self.storage_node.as_str())]
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::control_curves::ControlCurvePiecewiseInterpolatedParameter;
    use crate::parameters::ParameterValueType;

    #[test]
    fn test_control_curve_piecewise_interpolated() {
        let data = r#"
            {
                "type": "ControlCurvePiecewiseInterpolatedParameter",
                "storage_node": "Reservoir",
                "control_curves": [
                    "reservoir_cc",
                    {"type": "constant", "value":  0.2}
                ],
                "comment": "A witty comment",
                "values": [
                    [-0.1, -1.0],
                    [-100, -200],
                    [-300, -400]
                ],
                "minimum": 0.05
            }
            "#;

        let param: ControlCurvePiecewiseInterpolatedParameter = serde_json::from_str(data).unwrap();

        assert_eq!(param.node_references().len(), 1);
        assert_eq!(
            param.node_references().remove("storage_node"),
            Some("Reservoir")
        );

        assert_eq!(param.parameters().len(), 1);
        match param.parameters().remove("control_curves").unwrap() {
            ParameterValueType::List(p) => assert_eq!(p.len(), 2),
            _ => panic!("Wrong variant for control_curves."),
        };
    }
}
