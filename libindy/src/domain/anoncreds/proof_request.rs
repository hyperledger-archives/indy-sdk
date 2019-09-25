use std::collections::HashMap;
use std::fmt;
use ursa::cl::Nonce;

use utils::validation::Validatable;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use utils::wql::Query;

use super::credential::Credential;
use utils::qualifier;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequestPayload {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attributes: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
    pub non_revoked: Option<NonRevocedInterval>
}

#[derive(Debug)]
pub enum ProofRequest {
    ProofRequestV1(ProofRequestPayload),
    ProofRequestV2(ProofRequestPayload),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ProofRequestsVersion {
    V1,
    V2,
}

impl ProofRequest {
    pub fn value<'a>(&'a self) -> &'a ProofRequestPayload {
        match self {
            ProofRequest::ProofRequestV1(proof_req) => proof_req,
            ProofRequest::ProofRequestV2(proof_req) => proof_req,
        }
    }

    pub fn version(&self) -> ProofRequestsVersion {
        match self {
            ProofRequest::ProofRequestV1(_) => ProofRequestsVersion::V1,
            ProofRequest::ProofRequestV2(_) => ProofRequestsVersion::V2,
        }
    }
}

impl<'de> Deserialize<'de> for ProofRequest
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
                    "1.0" => {
                        let proof_request = ProofRequestPayload::deserialize(v).map_err(de::Error::custom)?;
                        Ok(ProofRequest::ProofRequestV1(proof_request))
                    }
                    "2.0" => {
                        let proof_request = ProofRequestPayload::deserialize(v).map_err(de::Error::custom)?;
                        Ok(ProofRequest::ProofRequestV2(proof_request))
                    }
                    _ => Err(de::Error::unknown_variant(&version, &["2.0"]))
                }
            }
            None => {
                let proof_request = ProofRequestPayload::deserialize(v).map_err(de::Error::custom)?;
                Ok(ProofRequest::ProofRequestV1(proof_request))
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

impl Validatable for ProofRequest {
    fn validate(&self) -> Result<(), String> {
        let value = self.value();
        let version = self.version();

        if value.requested_attributes.is_empty() && value.requested_predicates.is_empty() {
            return Err(String::from("Proof Request validation failed: both `requested_attributes` and `requested_predicates` are empty"));
        }

        for (_, requested_attribute) in value.requested_attributes.iter() {
            if requested_attribute.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_attribute));
            }
            if let Some(ref restrictions) = requested_attribute.restrictions {
                _process_operator(&restrictions, &version)?;
            }
        }

        for (_, requested_predicate) in value.requested_predicates.iter() {
            if requested_predicate.name.is_empty() {
                return Err(format!("Proof Request validation failed: there is empty requested attribute: {:?}", requested_predicate));
            }
            if let Some(ref restrictions) = requested_predicate.restrictions {
                _process_operator(&restrictions, &version)?;
            }
        }

        Ok(())
    }
}

fn _process_operator(restriction_op: &Query, version: &ProofRequestsVersion) -> Result<(), String> {
    match restriction_op {
        Query::Eq(ref tag_name, ref tag_value) |
        Query::Neq(ref tag_name, ref tag_value) |
        Query::Gt(ref tag_name, ref tag_value) |
        Query::Gte(ref tag_name, ref tag_value) |
        Query::Lt(ref tag_name, ref tag_value) |
        Query::Lte(ref tag_name, ref tag_value) |
        Query::Like(ref tag_name, ref tag_value) => {
            _check_restriction(tag_name, tag_value, version)
        }
        Query::In(ref tag_name, ref tag_values) => {
            tag_values
                .iter()
                .map(|tag_value| _check_restriction(tag_name, tag_value, version))
                .collect::<Result<Vec<()>, String>>()?;
            Ok(())
        }
        Query::And(ref operators) | Query::Or(ref operators) => {
            operators
                .iter()
                .map(|operator| _process_operator(operator, version))
                .collect::<Result<Vec<()>, String>>()?;
            Ok(())
        }
        Query::Not(ref operator) => {
            _process_operator(operator, version)
        }
    }
}

fn _check_restriction(tag_name: &str, tag_value: &str, version: &ProofRequestsVersion) -> Result<(), String> {
    if *version == ProofRequestsVersion::V1 &&
        Credential::QUALIFIABLE_TAGS.contains(&tag_name) &&
        qualifier::is_fully_qualified(tag_value) {
        return Err("Proof Request validation failed: fully qualified identifiers can not be used for Proof Request of the first version. \
                    Please, set \"ver\":\"2.0\" to use fully qualified identifiers.".to_string());
    }
    Ok(())
}