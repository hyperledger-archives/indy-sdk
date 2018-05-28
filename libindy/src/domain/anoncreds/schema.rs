extern crate indy_crypto;
extern crate serde;
extern crate serde_json;


use super::DELIMITER;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use std::collections::{HashMap, HashSet};

pub const SCHEMA_MARKER: &'static str = "2";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaV1 {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: HashSet<String>,
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
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

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