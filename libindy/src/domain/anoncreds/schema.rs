use super::DELIMITER;

use std::collections::{HashMap, HashSet};

pub const SCHEMA_MARKER: &'static str = "2";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaV1 {
    pub id: Option<String>,
    pub name: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "attrNames")]
    pub attr_names: AttributeNames,
    pub seq_no: Option<u32>,
}

impl Default for SchemaV1 {
    fn default() -> SchemaV1 {
        SchemaV1 {
            id: None,
            name: None,
            version: None,
            attr_names: AttributeNames::default(),
            seq_no: None
        }
    }
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