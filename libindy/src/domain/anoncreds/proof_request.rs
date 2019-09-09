use std::collections::HashMap;
use std::fmt;
use ursa::cl::Nonce;

use utils::validation::Validatable;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use utils::wql::Query;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attributes: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
    pub non_revoked: Option<NonRevocedInterval>
}

impl Default for ProofRequest {
    fn default() -> ProofRequest {
        ProofRequest {
            nonce: Nonce::new().unwrap(),
            name: String::from("default"),
            version: String::from("1.0"),
            requested_attributes: HashMap::new(),
            requested_predicates: HashMap::new(),
            non_revoked: None,
        }
    }
}

#[derive(Debug)]
pub enum ProofRequests {
    ProofRequestV1(ProofRequest),
    ProofRequestV2(ProofRequest),
}

impl ProofRequests {
    pub fn value<'a>(&'a self) -> &'a ProofRequest {
        match self {
            ProofRequests::ProofRequestV1(proof_req) => proof_req,
            ProofRequests::ProofRequestV2(proof_req) => proof_req,
        }
    }
}

impl<'de> Deserialize<'de> for ProofRequests
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Helper {
            ver: Option<String>,
        }

        let v = Value::deserialize(deserializer)?;

        let helper = Helper::deserialize(&v).map_err(de::Error::custom)?;

        match helper.ver {
            Some(version) => {
                match version.as_ref() {
                    "2.0" => {
                        let proof_request = ProofRequest::deserialize(v).map_err(de::Error::custom)?;
                        Ok(ProofRequests::ProofRequestV2(proof_request))
                    }
                    _ => Err(de::Error::unknown_variant(&version, &["2.0"]))
                }
            }
            None => {
                let proof_request = ProofRequest::deserialize(v).map_err(de::Error::custom)?;
                Ok(ProofRequests::ProofRequestV1(proof_request))
            }
        }
    }
}

pub type ProofRequestExtraQuery = HashMap<String, Query>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NonRevocedInterval {
    pub from: Option<u64>,
    pub to: Option<u64>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub restrictions: Option<Query>,
    pub non_revoked: Option<NonRevocedInterval>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub name: String,
    pub p_type: PredicateTypes,
    pub p_value: i32,
    pub restrictions: Option<Query>,
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

impl Validatable for ProofRequests {
    fn validate(&self) -> Result<(), String> {
        let value = self.value();

        if value.requested_attributes.is_empty() && value.requested_predicates.is_empty() {
            return Err(String::from("Proof Request validation failed: both `requested_attributes` and `requested_predicates` are empty"));
        }

        for (_, requested_attribute) in value.requested_attributes.iter() {
            if requested_attribute.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_attribute));
            }
        }

        for (_, requested_predicate) in value.requested_predicates.iter() {
            if requested_predicate.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_predicate));
            }
        }

        Ok(())
    }
}