use super::DELIMITER;

use super::super::crypto::did::DidValue;

use std::collections::{HashMap, HashSet};
use named_type::NamedType;

use utils::validation::Validatable;
use utils::qualifier::Qualifier;

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
    pub fn new(did: &DidValue, name: &str, version: &str) -> SchemaId {
        SchemaId(format!("{}{}{}{}{}{}{}", did.0, DELIMITER, SCHEMA_MARKER, DELIMITER, name, DELIMITER, version))
    }

    pub fn parts(&self) -> (DidValue, String, String) {
        let parts = self.0.split_terminator(DELIMITER).collect::<Vec<&str>>();

        let (schema_issuer_did, schema_name, schema_version) = if self.is_fully_qualified() {
            (format!("{}:{}:{}", parts[0], parts[1], parts[2]), parts[4].to_string(), parts[5].to_string())
        } else {
            (parts[0].to_string(), parts[2].to_string(), parts[3].to_string())
        };
        (DidValue(schema_issuer_did), schema_name, schema_version)
    }

    pub fn qualify(&self, prefix: Option<String>) -> SchemaId {
        SchemaId(Qualifier::qualify(&self.0, prefix))
    }

    pub fn unqualify(&self) -> SchemaId {
        SchemaId(Qualifier::unqualify(&self.0))
    }

    pub fn is_fully_qualified(&self) -> bool {
        Qualifier::is_fully_qualified(&self.0)
    }
}

impl Validatable for SchemaId {
    fn validate(&self) -> Result<(), String> {
        let parts: Vec<&str> = self.0.split_terminator(DELIMITER).collect::<Vec<&str>>();

        if parts.len() == 1 {
            parts[0]
                .parse::<i32>()
                .map_err(|_| format!("SchemaId validation failed: invalid number"))?;
        } else if parts.len() == 4 {
            // pass
        } else if parts.len() == 6 {
            if !Qualifier::is_fully_qualified(&self.0) {
                return Err("SchemaId validation failed: must be fully qualified".to_string());
            }
        } else {
            return Err("SchemaId validation failed: invalid number of parts".to_string());
        }

        Ok(())
    }
}