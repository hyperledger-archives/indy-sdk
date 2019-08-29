use super::DELIMITER;

use std::collections::{HashMap, HashSet};
use named_type::NamedType;

use utils::validation::Validatable;

pub const SCHEMA_MARKER: &str = "2";
pub const MAX_ATTRIBUTES_COUNT: usize = 125;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SchemaV1 {
    pub id: SchemaId,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: AttributeNames,
    pub seq_no: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, NamedType)]
#[serde(tag = "ver")]
pub enum Schema {
    #[serde(rename = "1.0")]
    SchemaV1(SchemaV1)
}

impl From<Schema> for SchemaV1 {
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::SchemaV1(schema) => schema
        }
    }
}

pub type Schemas = HashMap<SchemaId, Schema>;

pub fn schemas_map_to_schemas_v1_map(schemas: Schemas) -> HashMap<SchemaId, SchemaV1> {
    schemas.into_iter().map(|(schema_id, schema)| { (schema_id, SchemaV1::from(schema)) }).collect()
}

pub type AttributeNames = HashSet<String>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchemaId(pub String);

impl Validatable for Schema {
    fn validate(&self) -> Result<(), String> {
        match self {
            Schema::SchemaV1(schema) => {
                schema.attr_names.validate()?;
                schema.id.validate()?;
                Ok(())
            }
        }
    }
}

impl Validatable for AttributeNames {
    fn validate(&self) -> Result<(), String> {
        if self.is_empty() {
            return Err(String::from("Empty list of Schema attributes has been passed"));
        }

        if self.len() > MAX_ATTRIBUTES_COUNT {
            return Err(format!("The number of Schema attributes {} cannot be greater than {}", self.len(), MAX_ATTRIBUTES_COUNT));
        }
        Ok(())
    }
}

impl SchemaId {
    pub fn new(did: &str, name: &str, version: &str) -> SchemaId {
        SchemaId(format!("{}{}{}{}{}{}{}", did, DELIMITER, SCHEMA_MARKER, DELIMITER, name, DELIMITER, version))
    }

    pub fn issuer_did(&self) -> Option<String> {
        self.0.split(DELIMITER).next().map(String::from)
    }
}

impl Validatable for SchemaId {
    fn validate(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.0.split_terminator(DELIMITER).collect::<Vec<&str>>();

        if parts.len() != 1 && parts.len() != 4 {
            return Err("SchemaId validation failed: invalid number of parts".to_string());
        }

        Ok(())
    }
}