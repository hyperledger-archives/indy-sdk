extern crate indy_crypto;
extern crate serde;
extern crate serde_json;

use self::serde::ser::{self, Serialize, Serializer};
use self::serde::de::{self, Deserialize, Deserializer};
use serde_json::Value;

use super::build_id;

use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use std::collections::{HashMap, HashSet};

pub const SCHEMA_MARKER: &'static str = "\x02";

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaV0 {
    name: String,
    version: String,
    origin: String,
    attr_names: HashSet<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaV1 {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(rename = "attrNames")]
    pub attr_names: HashSet<String>
}

impl SchemaV1 {
    const VERSION: &'static str = "1";
}

#[derive(Debug)]
pub enum Schema {
    SchemaV0(SchemaV0),
    SchemaV1(SchemaV1)
}

impl Schema {
    pub fn schema_id(did: &str, name: &str, version: &str) -> String {
        build_id(did, SCHEMA_MARKER, None, name, version)
    }
}

impl JsonEncodable for Schema {}

impl<'a> JsonDecodable<'a> for Schema {}

impl<'de> Deserialize<'de> for Schema
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct VersionHelper {
            ver: Option<String>,
        }

        let v = Value::deserialize(deserializer)?;

        let helper = VersionHelper::deserialize(&v).map_err(de::Error::custom)?;

        match helper.ver {
            Some(version) => {
                match version.as_ref() {
                    SchemaV1::VERSION => {
                        let schema = SchemaV1::deserialize(v).map_err(de::Error::custom)?;
                        Ok(Schema::SchemaV1(schema))
                    }
                    _ => Err(de::Error::unknown_variant(&version, &[SchemaV1::VERSION]))
                }
            }
            None => {
                let schema = SchemaV0::deserialize(v).map_err(de::Error::custom)?;
                Ok(Schema::SchemaV0(schema))
            }
        }
    }
}

impl Serialize for Schema
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Schema::SchemaV1(ref schema) => {
                let mut v = serde_json::to_value(schema).map_err(ser::Error::custom)?;
                v["ver"] = serde_json::Value::String(SchemaV1::VERSION.to_string());
                v.serialize(serializer)
            }
            Schema::SchemaV0(ref schema) => schema.serialize(serializer),
        }
    }
}

impl From<SchemaV0> for SchemaV1 {
    fn from(schema_v0: SchemaV0) -> Self {
        SchemaV1 {
            id: Schema::schema_id(&schema_v0.origin, &schema_v0.name, &schema_v0.version),
            name: schema_v0.name,
            version: schema_v0.version,
            attr_names: schema_v0.attr_names,

        }
    }
}

impl From<Schema> for SchemaV1 {
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::SchemaV0(schema) => SchemaV1::from(schema),
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