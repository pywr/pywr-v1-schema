//! The `export-schema` command writes the schema the library generates for the requested kind.

use pywr_v1_schema::json_schema::{CustomTypes, model_schema, multi_model_schema, network_schema};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A fresh directory for one test's output, removed when it is dropped.
struct ScratchDir(PathBuf);

impl ScratchDir {
    fn new(test: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("pywr-v1-validator-{test}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create the scratch directory");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for ScratchDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn export_schema(args: &[&str], out: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_pywr-v1-validator"))
        .arg("export-schema")
        .arg(out)
        .args(args)
        .output()
        .expect("run pywr-v1-validator")
}

fn exported(args: &[&str], test: &str) -> Value {
    let dir = ScratchDir::new(test);
    let out = dir.path().join("schema.json");
    let output = export_schema(args, &out);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = std::fs::read_to_string(&out).expect("the schema file is written");
    serde_json::from_str(&text).expect("the schema file is JSON")
}

/// The library's schema as JSON (`schemars::Schema` is not a dependency of this crate to name).
macro_rules! expected {
    ($schema:expr) => {
        serde_json::to_value($schema).expect("the schema serialises")
    };
}

#[test]
fn model_is_the_default_kind_and_is_lenient() {
    assert_eq!(
        exported(&[], "default"),
        expected!(model_schema(CustomTypes::Any))
    );
}

#[test]
fn each_kind_writes_its_own_schema() {
    for (kind, schema) in [
        ("model", model_schema(CustomTypes::Any)),
        ("multi-model", multi_model_schema(CustomTypes::Any)),
        ("network", network_schema(CustomTypes::Any)),
    ] {
        assert_eq!(
            exported(&["--kind", kind], kind),
            expected!(schema),
            "{kind}"
        );
    }
}

#[test]
fn strict_writes_the_strict_schema() {
    for (kind, schema) in [
        ("model", model_schema(CustomTypes::NonCoreOnly)),
        ("multi-model", multi_model_schema(CustomTypes::NonCoreOnly)),
        ("network", network_schema(CustomTypes::NonCoreOnly)),
    ] {
        let strict = exported(&["--kind", kind, "--strict"], &format!("strict-{kind}"));
        assert_eq!(strict, expected!(schema), "{kind}");
        assert_ne!(
            strict,
            exported(&["--kind", kind], &format!("lenient-{kind}")),
            "{kind}"
        );
    }
}

#[test]
fn an_unwritable_path_is_an_error_not_a_panic() {
    let dir = ScratchDir::new("unwritable");
    let out = dir.path().join("missing-directory").join("schema.json");
    let output = export_schema(&[], &out);

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.starts_with("error: could not write ") && stderr.contains("schema.json"),
        "{stderr}"
    );
    assert!(!stderr.contains("panicked"), "{stderr}");
}

#[test]
fn an_unknown_kind_is_rejected() {
    let dir = ScratchDir::new("unknown-kind");
    let output = export_schema(&["--kind", "nonsense"], &dir.path().join("schema.json"));
    assert!(!output.status.success());
}
