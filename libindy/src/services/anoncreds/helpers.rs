extern crate indy_crypto;

use errors::common::CommonError;

use services::anoncreds::types::{PredicateInfo, SchemaKey};
use self::indy_crypto::cl::{issuer, verifier, ClaimSchema, ClaimValues, SubProofRequest};

use std::collections::{HashSet, HashMap};

pub fn get_composite_id(issuer_did: &str, schema_key: &SchemaKey) -> String {
    format!("{}:{}:{}:{}", issuer_did, schema_key.name, schema_key.version, schema_key.did)
}

pub fn build_claim_schema(attrs: &HashSet<String>) -> Result<ClaimSchema, CommonError> {
    let mut claim_schema_builder = issuer::Issuer::new_claim_schema_builder()?;
    for attr in attrs {
        claim_schema_builder.add_attr(&attr)?;
    }
    Ok(claim_schema_builder.finalize()?)
}

pub fn build_claim_values(claim_values: &HashMap<String, Vec<String>>) -> Result<ClaimValues, CommonError> {
    let mut claim_values_builder = issuer::Issuer::new_claim_values_builder()?;
    for (attr, values) in claim_values {
        let dec_val = values.get(1)
            .ok_or(CommonError::InvalidStructure(format!("Encoded value not found")))?;

        claim_values_builder.add_value(&attr, &dec_val)?;
    }
    Ok(claim_values_builder.finalize()?)
}

pub fn build_sub_proof_request(attrs_for_claim: &Vec<String>, predicates_for_claim: &Vec<PredicateInfo>) -> Result<SubProofRequest, CommonError> {
    let mut sub_proof_request_builder = verifier::Verifier::new_sub_proof_request_builder()?;

    for attr in attrs_for_claim {
        sub_proof_request_builder.add_revealed_attr(&attr)?
    }

    for predicate in predicates_for_claim {
        sub_proof_request_builder.add_predicate(&predicate.attr_name, "GE", predicate.value)?;
    }

    Ok(sub_proof_request_builder.finalize()?)
}