use serde::de::{self, IgnoredAny, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use std::fmt;

#[derive(Clone)]
pub struct Edge {
    pub from_node: String,
    pub to_node: String,

    // First option is whether there was an entry or not.
    // Second option is whether the entry was null or not.
    pub from_slot: Option<Option<String>>,
    pub to_slot: Option<Option<String>>,
}

// Custom deserialization required for the edges. In the Pywr specification these can
// be a tuple (array) of either 2 or 4 values. The 3 or 4 entries can also be `null`.
impl<'de> serde::Deserialize<'de> for Edge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EdgeVisitor;

        impl<'de> Visitor<'de> for EdgeVisitor {
            type Value = Edge;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a list of strings of either length 2 or 4, where the 3rd or 4th entry can also be null.")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let from_node = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let to_node = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let from_slot = seq.next_element()?;
                let to_slot = seq.next_element()?;

                // This is very important!
                while (seq.next_element::<IgnoredAny>()?).is_some() {
                    // Ignore rest
                }

                Ok(Edge {
                    from_node,
                    to_node,
                    from_slot,
                    to_slot,
                })
            }
        }

        deserializer.deserialize_seq(EdgeVisitor)
    }
}

// An edge is a JSON array. Entries beyond the fourth are ignored by the deserialiser, so `items` is
// left open, and the third and fourth entries may be omitted, `null` or a string.
impl schemars::JsonSchema for Edge {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Edge".into()
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "description": "An edge: [from_node, to_node] or [from_node, to_node, from_slot, to_slot], where the slots may be null. Entries beyond the fourth are ignored.",
            "type": "array",
            "minItems": 2,
            "prefixItems": [
                {"type": "string"},
                {"type": "string"},
                {"type": ["string", "null"]},
                {"type": ["string", "null"]}
            ],
            "items": true
        })
    }
}

impl serde::Serialize for Edge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let length = if self.from_slot.is_some() | self.to_slot.is_some() {
            4
        } else {
            2
        };
        let mut seq = serializer.serialize_seq(Some(length))?;
        seq.serialize_element(&self.from_node)?;
        seq.serialize_element(&self.to_node)?;

        if length == 4 {
            seq.serialize_element(&self.from_slot)?;
            seq.serialize_element(&self.to_slot)?;
        }

        seq.end()
    }
}
