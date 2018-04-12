extern crate indy_crypto;

use std::collections::HashMap;

use self::indy_crypto::cl::Nonce;
use self::indy_crypto::utils::json::{JsonDecodable};

use super::filter::Filter;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofRequest {
    pub nonce: Nonce,
    pub name: String,
    pub version: String,
    pub requested_attributes: HashMap<String, AttributeInfo>,
    pub requested_predicates: HashMap<String, PredicateInfo>,
    pub non_revoked: Option<NonRevocedInterval>
}

impl<'a> JsonDecodable<'a> for ProofRequest {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NonRevocedInterval {
    pub from: Option<u64>,
    pub to: Option<u64>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AttributeInfo {
    pub name: String,
    pub restrictions: Option<Vec<Filter>>,
    pub non_revoked: Option<NonRevocedInterval>
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PredicateInfo {
    pub name: String,
    pub p_type: String,
    pub p_value: i32,
    pub restrictions: Option<Vec<Filter>>,
    pub non_revoked: Option<NonRevocedInterval>
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