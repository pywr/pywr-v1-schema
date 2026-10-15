use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

/// Which Pywr v1 document the exported JSON Schema describes.
#[derive(Copy, Clone, Debug, ValueEnum)]
enum SchemaKind {
    /// A single-network model file.
    Model,
    /// A multi-model file.
    MultiModel,
    /// A bare network, as read by `validate --network-only`.
    Network,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Read a Pywr v1 JSON file and report the custom node and parameter types it uses.
    Validate {
        /// Path to the Pywr v1 JSON file.
        #[arg(short, long)]
        path: std::path::PathBuf,
        /// Read the file as a bare network, not a whole model.
        #[arg(short, long)]
        network_only: bool,
    },
    /// Write the JSON Schema for the Pywr v1 model format.
    ExportSchema {
        /// Path to save the JSON schema.
        out: std::path::PathBuf,
        /// The kind of document to describe.
        #[arg(short, long, value_enum, default_value_t = SchemaKind::Model)]
        kind: SchemaKind,
        /// Report invalid core nodes and parameters instead of accepting them as custom ones.
        ///
        /// The deserialisers read a node or parameter as a custom one whenever its core definition
        /// fails to parse. With this flag a custom node or parameter must use a type that is not a
        /// core type, so the error in a core definition is reported.
        #[arg(short, long)]
        strict: bool,
    },
}

/// Validate Pywr v1 model files and export their JSON Schema.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Write the JSON Schema for `kind` to `out`.
///
/// Fails, naming the file, when the schema cannot be serialised or `out` cannot be written.
fn export_schema(out: &std::path::Path, kind: SchemaKind, strict: bool) -> Result<(), String> {
    use pywr_v1_schema::json_schema::{
        CustomTypes, model_schema, multi_model_schema, network_schema,
    };

    let custom_types = if strict {
        CustomTypes::NonCoreOnly
    } else {
        CustomTypes::Any
    };
    let schema = match kind {
        SchemaKind::Model => model_schema(custom_types),
        SchemaKind::MultiModel => multi_model_schema(custom_types),
        SchemaKind::Network => network_schema(custom_types),
    };
    let json = serde_json::to_string_pretty(&schema)
        .map_err(|e| format!("could not serialise the schema: {e}"))?;
    std::fs::write(out, json).map_err(|e| format!("could not write {}: {e}", out.display()))
}

fn main() {
    match Args::parse().command {
        Command::ExportSchema { out, kind, strict } => {
            if let Err(message) = export_schema(&out, kind, strict) {
                eprintln!("error: {message}");
                std::process::exit(1);
            }
        }
        Command::Validate { path, network_only } => validate(path, network_only),
    }
}

/// Read the file at `path` as a model (or a bare network with `network_only`) and report the custom
/// node and parameter types it uses.
///
/// Exits the process when the file cannot be opened or parsed.
fn validate(path: std::path::PathBuf, network_only: bool) {
    println!("Path: {:?}", path);

    let file = File::open(path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let network: pywr_v1_schema::PywrNetwork = if network_only {
        serde_json::from_reader(reader).expect("Failed to parse Pywr JSON file.")
    } else {
        let model: pywr_v1_schema::PywrModel =
            serde_json::from_reader(reader).expect("Failed to parse Pywr JSON file.");

        model.network
    };

    println!("Parsed Pywr JSON file successfully!");

    {
        // Identify custom nodes
        let custom_types: HashSet<_> = match network.nodes {
            Some(nodes) => nodes
                .iter()
                .filter_map(|n| {
                    if let pywr_v1_schema::nodes::Node::Custom(c) = n {
                        Some(c.ty.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            None => HashSet::new(),
        };

        let mut custom_types: Vec<_> = custom_types.into_iter().collect();
        custom_types.sort();

        if !custom_types.is_empty() {
            println!("Found {} custom node types:", custom_types.len());
            for ty in custom_types {
                println!("  {}", ty);
            }
        } else {
            println!("No custom nodes found!")
        }
    }

    if let Some(parameters) = network.parameters {
        let custom_types: HashSet<_> = parameters
            .iter()
            .filter_map(|p| {
                if let pywr_v1_schema::parameters::Parameter::Custom(c) = p {
                    Some(c.ty.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut custom_types: Vec<_> = custom_types.into_iter().collect();
        custom_types.sort();

        if !custom_types.is_empty() {
            println!("Found {} custom parameter types:", custom_types.len());
            for ty in custom_types {
                println!("  {}", ty);
            }
        } else {
            println!("No custom parameters found!")
        }
    }
}
