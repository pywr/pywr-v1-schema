//! The generated JSON Schema must accept every model that the deserialisers accept.

use jsonschema::Validator;
use pywr_v1_schema::json_schema::{CustomTypes, model_schema, multi_model_schema, network_schema};
use pywr_v1_schema::{PywrModel, PywrMultiModel};
use serde_json::{Value, json};
use std::fs;
use std::path::PathBuf;

/// Models that are network fragments pulled in by another model's `includes`, not whole models.
const NETWORK_FRAGMENTS: [&str; 1] = ["extra2.json"];

/// Models in which a core node or parameter is not modelled by the crate, so the deserialisers read
/// it as a custom one. Strict validation reports them; each is expected to fail it.
const READ_AS_CUSTOM: [&str; 2] = [
    // `interpolatedvolume` with `volumes` and `values` read from external data.
    "reservoir_evaporation_areafromfile.json",
    // A storage node whose `initial_volume` is a table reference.
    "reservoir_initial_vol_from_table.json",
];

fn models_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("models")
}

fn validator_for(schema: schemars::Schema) -> Validator {
    let schema = serde_json::to_value(schema).expect("schema serialises");
    jsonschema::validator_for(&schema).expect("the generated schema is a valid JSON Schema")
}

fn read_model(name: &str) -> Value {
    let path = models_dir().join(name);
    let data = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    serde_json::from_str(&data).unwrap_or_else(|e| panic!("parse {path:?}: {e}"))
}

/// The file names of every whole model in the test models directory.
fn model_names() -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(models_dir())
        .expect("models directory")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .map(|path| {
            path.file_name()
                .expect("file name")
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| !NETWORK_FRAGMENTS.contains(&name.as_str()))
        .collect();
    names.sort();
    assert!(!names.is_empty(), "no test models found");
    names
}

fn errors(validator: &Validator, instance: &Value) -> Vec<String> {
    validator
        .iter_errors(instance)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

#[test]
fn every_test_model_validates() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    let failures: Vec<String> = model_names()
        .into_iter()
        .filter_map(|name| {
            let errors = errors(&validator, &read_model(&name));
            (!errors.is_empty()).then(|| format!("{name}: {}", errors.join("; ")))
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{} models failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn network_fragments_validate_as_networks() {
    let validator = validator_for(network_schema(CustomTypes::Any));
    for name in NETWORK_FRAGMENTS {
        assert_eq!(
            errors(&validator, &read_model(name)),
            Vec::<String>::new(),
            "{name}"
        );
    }
}

/// Without the custom fallback the core definitions themselves are checked, which catches fields
/// the schema does not describe (a serde `alias`, for example) that the lenient schema hides.
#[test]
fn core_definitions_in_test_models_are_valid() {
    let validator = validator_for(model_schema(CustomTypes::NonCoreOnly));
    for name in model_names() {
        let errors = errors(&validator, &read_model(&name));
        if READ_AS_CUSTOM.contains(&name.as_str()) {
            assert!(
                !errors.is_empty(),
                "{name} is no longer read as custom: remove it from READ_AS_CUSTOM"
            );
        } else {
            assert!(errors.is_empty(), "{name}: {}", errors.join("; "));
        }
    }
}

/// A schema that accepted everything would pass the tests above, so check that breaking a model in
/// each of several ways makes it fail.
#[test]
fn broken_models_are_rejected() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    let valid = read_model("river1.json");
    assert!(
        validator.is_valid(&valid),
        "river1.json should be valid before it is broken"
    );

    let mut renamed_timestepper = valid.clone();
    let object = renamed_timestepper
        .as_object_mut()
        .expect("a model is an object");
    let timestepper = object
        .remove("timestepper")
        .expect("river1.json has a timestepper");
    object.insert("timestep".to_string(), timestepper);
    assert!(
        !validator.is_valid(&renamed_timestepper),
        "renamed `timestepper` key must be rejected"
    );

    let mut unnamed_node = valid.clone();
    let node = unnamed_node["nodes"][0]
        .as_object_mut()
        .expect("a node is an object");
    let name = node.remove("name").expect("a node has a name");
    node.insert("nme".to_string(), name);
    assert!(
        !validator.is_valid(&unnamed_node),
        "renamed node `name` key must be rejected"
    );

    let mut short_edge = valid.clone();
    short_edge["edges"][0] = json!(["only-one-node"]);
    assert!(
        !validator.is_valid(&short_edge),
        "an edge with one entry must be rejected"
    );

    let mut long_slice = valid.clone();
    long_slice["scenarios"] = json!([{"name": "s", "size": 4, "slice": [0, 1, 2, 3]}]);
    assert!(
        !validator.is_valid(&long_slice),
        "a scenario slice of four entries must be rejected"
    );
}

/// Custom nodes and parameters stay valid in both modes: a type the crate does not know is not an
/// error.
#[test]
fn custom_nodes_and_parameters_are_accepted() {
    for custom_types in [CustomTypes::Any, CustomTypes::NonCoreOnly] {
        let validator = validator_for(model_schema(custom_types));
        let mut model = read_model("river1.json");
        model["nodes"]
            .as_array_mut()
            .expect("nodes")
            .push(json!({"name": "special", "type": "MyCustomNode", "anything": [1, 2, 3]}));
        model["parameters"] = json!({
            "mine": {"type": "MyCustomParameter", "anything": {"a": 1}},
            "by_alias": {"type": "constantparameter", "value": 1.0}
        });
        assert_eq!(
            errors(&validator, &model),
            Vec::<String>::new(),
            "{custom_types:?}"
        );
    }
}

/// An invalid core definition is reported only in strict mode.
#[test]
fn strict_mode_reports_invalid_core_parameters() {
    let mut model = read_model("river1.json");
    model["parameters"] =
        json!({"p": {"type": "aggregated", "agg_func": "bogus", "parameters": []}});

    assert!(validator_for(model_schema(CustomTypes::Any)).is_valid(&model));
    assert!(!validator_for(model_schema(CustomTypes::NonCoreOnly)).is_valid(&model));
}

/// Properties and values that serde reads under another name must validate.
#[test]
fn serde_aliases_are_accepted() {
    let validator = validator_for(model_schema(CustomTypes::NonCoreOnly));
    let mut model = read_model("river1.json");
    model["parameters"] = json!({
        "index": {"type": "constant", "values": 1.0},
        "array": {"type": "indexedarray", "index_parameter": "index", "params": [1.0, 2.0]},
        "threshold": {"type": "parameterthreshold", "parameter": "index", "threshold": 1.0, "predicate": ">="}
    });
    assert_eq!(errors(&validator, &model), Vec::<String>::new());

    // The deserialiser reads every one of them as a core parameter, not a custom one.
    let parsed: PywrModel = serde_json::from_value(model).expect("the model deserialises");
    let parameters = parsed.network.parameters.expect("the model has parameters");
    assert!(parameters.iter().all(|p| !p.is_custom()));
}

/// A model whose `parameters` map holds `parameters`.
fn model_with_parameters(parameters: Value) -> Value {
    let mut model = read_model("river1.json");
    model["parameters"] = parameters;
    model
}

/// The name-keyed `parameters` map reads `type` case-insensitively, with or without a `parameter`
/// suffix, so strict mode must check the definition however the type is spelt.
#[test]
fn strict_mode_reports_invalid_core_parameters_under_any_type_spelling() {
    let lenient = validator_for(model_schema(CustomTypes::Any));
    let strict = validator_for(model_schema(CustomTypes::NonCoreOnly));
    for ty in [
        "constant",
        "Constant",
        "CONSTANT",
        "cOnStAnT",
        "constantparameter",
        "ConstantParameter",
        "CONSTANTPARAMETER",
        "constantParameter",
        "constantparameterparameter",
    ] {
        let invalid = model_with_parameters(json!({"p": {"type": ty, "value": "x"}}));
        assert!(lenient.is_valid(&invalid), "{ty}: lenient");
        assert!(!strict.is_valid(&invalid), "{ty}: strict");

        let valid = model_with_parameters(json!({"p": {"type": ty, "value": 1.0}}));
        assert_eq!(errors(&strict, &valid), Vec::<String>::new(), "{ty}");
    }
}

/// A type that only differs by case from a core one is still a custom type when it names none.
#[test]
fn strict_mode_accepts_custom_parameters_beside_case_folded_core_ones() {
    let strict = validator_for(model_schema(CustomTypes::NonCoreOnly));
    let model = model_with_parameters(json!({
        "a": {"type": "MyConstant", "value": "x"},
        "b": {"type": "constants", "value": "x"},
        "c": {"type": "parameter", "value": "x"}
    }));
    assert_eq!(errors(&strict, &model), Vec::<String>::new());
}

/// The Kelvin sign lower-cases to `k`, so the deserialiser reads it as a core `WeeklyProfile`.
#[test]
fn strict_mode_follows_unicode_lower_casing() {
    let strict = validator_for(model_schema(CustomTypes::NonCoreOnly));
    let model = model_with_parameters(json!({
        "w": {"type": "Wee\u{212A}lyProfile", "values": "x"}
    }));
    assert!(!strict.is_valid(&model));
}

/// Nodes and inline parameters match `type` exactly, so a differently-cased spelling is a custom
/// definition there, in strict mode as in the deserialiser.
#[test]
fn nodes_and_inline_parameters_match_type_exactly() {
    let strict = validator_for(model_schema(CustomTypes::NonCoreOnly));

    let mut model = read_model("river1.json");
    model["nodes"][0] = json!({"name": "n", "type": "INPUT", "max_flow": "not-a-number"});
    assert!(strict.is_valid(&model));
    let parsed: PywrModel = serde_json::from_value(model).expect("the model deserialises");
    let nodes = parsed.network.nodes.expect("the model has nodes");
    assert!(
        nodes
            .iter()
            .any(|node| matches!(node, pywr_v1_schema::nodes::Node::Custom(_)))
    );

    let inline = model_with_parameters(json!({
        "outer": {"type": "max", "parameter": {"type": "CONSTANT", "value": "x"}, "threshold": 1.0}
    }));
    assert!(strict.is_valid(&inline));
    let invalid_inline = model_with_parameters(json!({
        "outer": {"type": "max", "parameter": {"type": "constant", "value": "x"}, "threshold": 1.0}
    }));
    assert!(!strict.is_valid(&invalid_inline));
}

/// `HydroPowerTarget` is read as a core parameter, so strict mode checks its definition, here
/// with the `target` of the shipped model replaced by a value of the wrong type.
#[test]
fn strict_mode_checks_the_differently_cased_core_parameter_of_a_test_model() {
    let strict = validator_for(model_schema(CustomTypes::NonCoreOnly));
    let mut model = read_model("hydropower_target_example.json");
    assert!(strict.is_valid(&model));

    let parameter = model["parameters"]
        .as_object_mut()
        .expect("parameters")
        .values_mut()
        .find(|p| p["type"] == "HydroPowerTarget")
        .expect("the model has a HydroPowerTarget parameter");
    parameter["turbine_elevation"] = json!(["not", "a", "number"]);
    assert!(!strict.is_valid(&model));
}

/// A model whose timestepper runs from `start` to `end`.
fn model_with_dates(start: &str, end: &str) -> Value {
    let mut model = read_model("river1.json");
    model["timestepper"]["start"] = json!(start);
    model["timestepper"]["end"] = json!(end);
    model
}

/// Whether the deserialiser reads the dates of `model`.
fn deserialises(model: &Value) -> bool {
    serde_json::from_value::<PywrModel>(model.clone()).is_ok()
}

/// The timestepper's dates are accepted and rejected by the deserialiser, for the forms in everyday
/// use and for the ones it refuses. The schema describes them with `schemars`' own `jiff` support,
/// a `partial-date-time` format that validators do not define, so only the deserialiser refuses a
/// malformed one and the schema still accepts a string there.
#[test]
fn timestepper_dates_follow_the_deserialiser() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    for date in [
        "2015-01-01",
        "2015-01-01T00:00:00",
        "2015-01-01 00:00",
        "2015-01-01T23:59:59.999",
        "2015-01-01T00:00:00+01:00",
        "2015-01-01T00:00:00+01:00[Europe/London]",
        "20150101",
        "2016-02-29",
    ] {
        let model = model_with_dates(date, "2015-12-31");
        assert!(deserialises(&model), "{date}: deserialiser");
        assert!(validator.is_valid(&model), "{date}: schema");
    }
    for date in [
        "2015-13-01",
        "2015-02-30",
        "2015/01/01",
        "2015-01-01T00:00:00Z",
        "2015-01-01T24:00:00",
        "01/02/2015",
        "2015-1-1",
        "tomorrow",
        "",
    ] {
        for model in [
            model_with_dates(date, "2015-12-31"),
            model_with_dates("2015-01-01", date),
        ] {
            assert!(!deserialises(&model), "{date:?}: deserialiser");
            assert!(
                validator.is_valid(&model),
                "{date:?}: the schema describes the field with a format, not a pattern"
            );
        }
    }
}

/// A multi-model file has the same timestepper, and its dates are checked by the deserialiser.
#[test]
fn multi_model_timestepper_dates_are_checked() {
    let validator = validator_for(multi_model_schema(CustomTypes::Any));
    let model = |start: &str| {
        json!({
            "metadata": {"title": "t"},
            "timestepper": {"start": start, "end": "2015-12-31", "timestep": 1},
            "models": [{"name": "a", "path": "a.json"}]
        })
    };
    let reads = |start: &str| serde_json::from_value::<PywrMultiModel>(model(start)).is_ok();
    assert!(validator.is_valid(&model("2015-01-01")));
    assert!(reads("2015-01-01"));
    assert!(!reads("2015/01/01"));
}

/// A multi-model file holding one sub-model with the given members.
fn multi_model(sub_model: Value) -> Value {
    json!({
        "metadata": {"title": "t"},
        "timestepper": {"start": "2015-01-01", "end": "2015-12-31", "timestep": 1},
        "models": [sub_model]
    })
}

/// The multi-model schema accepts what `PywrMultiModel` deserialises: a sub-model given inline,
/// by path or by file name, and rejects the shapes it refuses.
#[test]
fn multi_model_schema_follows_the_deserialiser() {
    let validator = validator_for(multi_model_schema(CustomTypes::Any));
    let inline = read_model("river1.json");

    for (label, document, valid) in [
        (
            "inline",
            multi_model(json!({"name": "a", "data": inline})),
            true,
        ),
        (
            "path",
            multi_model(json!({"name": "a", "path": "a.json"})),
            true,
        ),
        (
            "filename",
            multi_model(json!({"name": "a", "filename": "a.json", "solver": "glpk"})),
            true,
        ),
        ("unnamed", multi_model(json!({"path": "a.json"})), false),
        ("numeric name", multi_model(json!({"name": 1})), false),
        (
            "numeric path",
            multi_model(json!({"name": "a", "path": 1})),
            false,
        ),
        (
            "broken inline model",
            multi_model(json!({"name": "a", "data": {"metadata": {}}})),
            false,
        ),
        (
            "no models",
            json!({
                "metadata": {"title": "t"},
                "timestepper": {"start": "2015-01-01", "end": "2015-12-31", "timestep": 1}
            }),
            false,
        ),
        (
            "models not a list",
            json!({
                "metadata": {"title": "t"},
                "timestepper": {"start": "2015-01-01", "end": "2015-12-31", "timestep": 1},
                "models": {"name": "a"}
            }),
            false,
        ),
    ] {
        assert_eq!(
            serde_json::from_value::<pywr_v1_schema::PywrMultiModel>(document.clone()).is_ok(),
            valid,
            "{label}: deserialiser"
        );
        assert_eq!(validator.is_valid(&document), valid, "{label}: schema");
    }
}

/// A multi-model file reaches the same node and parameter definitions as a model, so strict mode
/// applies inside an inline sub-model.
#[test]
fn multi_model_strict_mode_checks_inline_models() {
    let mut inline = read_model("river1.json");
    inline["parameters"] = json!({"p": {"type": "CONSTANT", "value": "x"}});
    let document = multi_model(json!({"name": "a", "data": inline}));

    assert!(validator_for(multi_model_schema(CustomTypes::Any)).is_valid(&document));
    assert!(!validator_for(multi_model_schema(CustomTypes::NonCoreOnly)).is_valid(&document));
}

/// The body of a table needs a `url`, takes the pandas keyword arguments it does not name, and its
/// name comes from the key of the `tables` map, not from the body.
#[test]
fn table_bodies_are_checked() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    let with_table = |table: Value| {
        let mut model = read_model("river1.json");
        model["tables"] = json!({"t": table});
        model
    };

    for (label, table, valid) in [
        ("url", json!({"url": "data.csv"}), true),
        (
            "columns",
            json!({"url": "data.csv", "columns": ["a", "b"], "index_col": 0}),
            true,
        ),
        (
            "named in the body",
            json!({"url": "data.csv", "name": "t"}),
            true,
        ),
        ("no url", json!({"column": "a"}), false),
        ("numeric url", json!({"url": 5}), false),
        (
            "numeric column",
            json!({"url": "data.csv", "column": 5}),
            false,
        ),
        (
            "columns not a list",
            json!({"url": "data.csv", "columns": "a"}),
            false,
        ),
        ("table not an object", json!("data.csv"), false),
    ] {
        let model = with_table(table);
        assert_eq!(
            serde_json::from_value::<PywrModel>(model.clone()).is_ok(),
            valid,
            "{label}: deserialiser"
        );
        assert_eq!(validator.is_valid(&model), valid, "{label}: schema");
    }

    let mut as_list = read_model("river1.json");
    as_list["tables"] = json!([{"name": "t", "url": "data.csv"}]);
    assert!(!validator.is_valid(&as_list), "tables as a list");
}

/// `values` is another name for `value` on a constant parameter, so it takes the same type; with
/// the alias unknown to the schema any `values` would be an unchecked extra property.
#[test]
fn constant_parameter_values_alias_is_typed() {
    let validator = validator_for(model_schema(CustomTypes::NonCoreOnly));
    for (members, valid) in [
        (json!({"value": 1.0}), true),
        (json!({"values": 1.0}), true),
        (json!({"values": "x"}), false),
        (json!({"values": [1.0]}), false),
        (json!({"value": "x"}), false),
    ] {
        let mut parameter = members.clone();
        parameter["type"] = json!("constant");
        let model = model_with_parameters(json!({"p": parameter}));
        assert_eq!(validator.is_valid(&model), valid, "{members}");
    }
}

/// An edge may carry entries beyond the fourth, which the deserialiser ignores.
#[test]
fn edges_may_carry_extra_entries() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    let mut model = read_model("river1.json");
    let edge = model["edges"][0].clone();
    let from_to = [edge[0].clone(), edge[1].clone()];

    for (members, valid) in [
        (json!([from_to[0], from_to[1], null, null, "extra"]), true),
        (
            json!([from_to[0], from_to[1], null, null, 7, {"a": 1}]),
            true,
        ),
        (json!([from_to[0], from_to[1], "a", "b"]), true),
        (json!([from_to[0], from_to[1], 1]), false),
        (json!([from_to[0]]), false),
    ] {
        model["edges"][0] = members.clone();
        assert_eq!(
            serde_json::from_value::<PywrModel>(model.clone()).is_ok(),
            valid,
            "{members}: deserialiser"
        );
        assert_eq!(validator.is_valid(&model), valid, "{members}: schema");
    }
}

/// The schema describes recorders as an object, which is what Pywr itself reads, although the
/// deserialiser keeps whatever JSON value it is given.
#[test]
fn recorders_are_an_object() {
    let validator = validator_for(model_schema(CustomTypes::Any));
    for (recorders, valid) in [
        (
            json!({"r": {"type": "numpyarraynoderecorder", "node": "a"}}),
            true,
        ),
        (json!({}), true),
        (Value::Null, true),
        (json!([1]), false),
        (json!("x"), false),
    ] {
        let mut model = read_model("river1.json");
        model["recorders"] = recorders.clone();
        assert_eq!(validator.is_valid(&model), valid, "{recorders}");
    }
}

/// `writeOnly` marks a property that an editor must not offer for reading, but every property of
/// the format is read from a file, including the names that are skipped on serialisation.
#[test]
fn no_property_is_write_only() {
    for custom_types in [CustomTypes::Any, CustomTypes::NonCoreOnly] {
        for schema in [
            model_schema(custom_types),
            multi_model_schema(custom_types),
            network_schema(custom_types),
        ] {
            let text = serde_json::to_string(&schema).expect("schema serialises");
            assert!(!text.contains("writeOnly"), "{custom_types:?}");
        }
    }
}
