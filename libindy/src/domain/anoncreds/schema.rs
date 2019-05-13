use super::DELIMITER;

use std::collections::{HashMap, HashSet};

pub const SCHEMA_MARKER: &str = "2";
pub const MAX_ATTRIBUTES_COUNT: usize = 125;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaV1 {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: AttributeNames,
    pub seq_no: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum Schema {
    #[serde(rename = "1.0")]
    SchemaV1(SchemaV1)
}

impl Schema {
    pub fn schema_id(did: &str, name: &str, version: &str) -> String {
        format!("{}{}{}{}{}{}{}", did, DELIMITER, SCHEMA_MARKER, DELIMITER, name, DELIMITER, version)
    }

    pub fn issuer_did(schema_id: &str) -> Option<String> {
        schema_id.split(':').collect::<Vec<&str>>().get(0).and_then(|s| Some(s.to_string()))
    }
}

impl From<Schema> for SchemaV1 {
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::SchemaV1(schema) => schema
        }
    }
}

pub fn schemas_map_to_schemas_v1_map(schemas: HashMap<String, Schema>) -> HashMap<String, SchemaV1> {
    let mut schemas_v1: HashMap<String, SchemaV1> = HashMap::new();

    for (schema_id, schema) in schemas {
        schemas_v1.insert(schema_id, SchemaV1::from(schema));
    }

    schemas_v1
}

pub type AttributeNames = HashSet<String>;