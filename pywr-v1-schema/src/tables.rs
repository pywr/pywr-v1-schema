use serde::de::value::MapDeserializer;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::vec::IntoIter;

#[derive(serde::Deserialize, serde::Serialize, schemars::JsonSchema, Clone)]
pub struct Table {
    // Do not serialize name on the object as it is used as the key in the parent map,
    // and we don't want to duplicate it in the output.
    // Supplied by the key of the parent map, so it is not part of the JSON object's schema.
    #[serde(skip_serializing)]
    #[schemars(skip)]
    pub name: String,
    pub url: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(flatten)]
    pub pandas_kwargs: HashMap<String, serde_json::Value>,
}

impl Table {
    pub fn resource_paths(&self) -> Vec<PathBuf> {
        vec![self.url.clone()]
    }

    /// Update any external resource paths referenced by this parameter if they are
    /// in the provided map.
    pub fn update_resource_paths(&mut self, new_paths: &HashMap<PathBuf, PathBuf>) {
        if let Some(new_path) = new_paths.get(&self.url) {
            self.url.clone_from(new_path);
        }
    }
}

#[derive(Clone)]
pub struct TableVec(Vec<Table>);

impl TableVec {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl IntoIterator for TableVec {
    type Item = Table;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for TableVec {
    type Target = Vec<Table>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TableVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct PywrTableMapVisitor {
    marker: PhantomData<fn() -> TableVec>,
}

impl PywrTableMapVisitor {
    fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for PywrTableMapVisitor {
    type Value = TableVec;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a valid Pywr Table definition")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = TableVec::with_capacity(access.size_hint().unwrap_or(0));

        #[derive(serde::Deserialize, Debug)]
        struct Helper {
            #[serde(flatten)]
            attributes: HashMap<String, Value>,
        }

        // While there are entries remaining in the input, add them into our map.
        while let Some((name, value)) = access.next_entry::<String, Helper>()? {
            let mut py_attributes = value.attributes.clone();
            py_attributes.insert("name".to_string(), Value::String(name.clone()));

            let tbl = Table::deserialize(MapDeserializer::new(py_attributes.into_iter()))
                .map_err(|e| de::Error::custom(format!("{}", e)))?;

            map.push(tbl);
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for TableVec {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PywrTableMapVisitor::new())
    }
}

// Tables are a JSON object keyed by table name; the key becomes `Table::name`.
impl schemars::JsonSchema for TableVec {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TableVec".into()
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "Tables keyed by name.",
            "type": "object",
            "additionalProperties": generator.subschema_for::<Table>()
        })
    }
}

impl Serialize for TableVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for tbl in &self.0 {
            map.serialize_entry(&tbl.name, tbl)?;
        }
        map.end()
    }
}
