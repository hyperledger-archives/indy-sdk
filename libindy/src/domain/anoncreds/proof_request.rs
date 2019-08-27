use serde_json;
use std::collections::HashMap;
use std::fmt;
use ursa::cl::Nonce;

use utils::validation::Validatable;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attributes: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
    pub non_revoked: Option<NonRevocedInterval>
}

pub type ProofRequestExtraQuery = HashMap<String, serde_json::Map<String, serde_json::Value>>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NonRevocedInterval {
    pub from: Option<u64>,
    pub to: Option<u64>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub restrictions: Option<serde_json::Value>,
    pub non_revoked: Option<NonRevocedInterval>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub name: String,
    pub p_type: PredicateTypes,
    pub p_value: i32,
    pub restrictions: Option<serde_json::Value>,
    pub non_revoked: Option<NonRevocedInterval>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PredicateTypes {
    #[serde(rename = ">=")]
    GE,
    #[serde(rename = "<=")]
    LE,
    #[serde(rename = ">")]
    GT,
    #[serde(rename = "<")]
    LT
}

impl fmt::Display for PredicateTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PredicateTypes::GE => write!(f, "GE"),
            PredicateTypes::GT => write!(f, "GT"),
            PredicateTypes::LE => write!(f, "LE"),
            PredicateTypes::LT => write!(f, "LT")
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestedAttributeInfo {
    pub attr_referent: String,
    pub attr_info: AttributeInfo,
    pub revealed: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestedPredicateInfo {
    pub predicate_referent: String,
    pub predicate_info: PredicateInfo
}

impl Validatable for ProofRequest {
    fn validate(&self) -> Result<(), String> {
        if self.requested_attributes.is_empty() && self.requested_predicates.is_empty() {
            return Err(String::from("Proof Request validation failed: both `requested_attributes` and `requested_predicates` are empty"));
        }

        for (_, requested_attribute) in self.requested_attributes.iter() {
            if requested_attribute.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_attribute));
            }
        }

        for (_, requested_predicate) in self.requested_predicates.iter() {
            if requested_predicate.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_predicate));
            }
        }

        Ok(())
    }
}