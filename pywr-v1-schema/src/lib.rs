pub mod edge;
pub mod json_schema;
pub mod model;
pub mod nodes;
pub mod parameters;
pub mod tables;

pub use model::{PywrModel, PywrMultiModel, PywrNetwork};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PywrSchemaError {
    #[error("Failed to read file `{path}`: {source}")]
    FileOpenError { path: PathBuf, source: io::Error },
    #[error("Failed to deserialize JSON: {source}")]
    DeserializeError {
        #[source]
        source: serde_json::Error,
    },
}
