//! The `validate` command reads a model file and reports its custom types; the file is only
//! accepted through that command.

use std::path::PathBuf;
use std::process::{Command, Output};

fn model(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("pywr-v1-schema")
        .join("tests")
        .join("models")
        .join(name)
}

fn validator(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pywr-v1-validator"))
        .args(args)
        .output()
        .expect("run pywr-v1-validator")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn validate_reads_a_model_file() {
    let path = model("river1.json");
    let output = validator(&["validate", "--path", path.to_str().expect("a UTF-8 path")]);

    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout(&output).contains("Parsed Pywr JSON file successfully!"),
        "{}",
        stdout(&output)
    );
}

#[test]
fn validate_reports_the_custom_types_a_model_uses() {
    let path = model("reservoir_evaporation_areafromfile.json");
    let output = validator(&["validate", "--path", path.to_str().expect("a UTF-8 path")]);

    assert!(output.status.success(), "{output:?}");
    assert!(
        stdout(&output).contains("custom parameter types")
            || stdout(&output).contains("custom node types"),
        "{}",
        stdout(&output)
    );
}

#[test]
fn validate_reads_a_bare_network_with_network_only() {
    let path = model("extra2.json");
    let output = validator(&[
        "validate",
        "--network-only",
        "--path",
        path.to_str().expect("a UTF-8 path"),
    ]);

    assert!(output.status.success(), "{output:?}");
    assert!(stdout(&output).contains("Parsed Pywr JSON file successfully!"));
}

#[test]
fn validate_needs_a_path() {
    let output = validator(&["validate"]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("--path"),
        "{output:?}"
    );
}
